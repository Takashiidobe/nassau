#!/usr/bin/env python3
"""Regenerate the FileCheck expectations embedded in Nassau's fixtures.

Every fixture under tests/fixtures and tests/repl carries its expected output
as SML comment lines at the end of the file:

    (* CHECK-EXIT: 18 *)                   exit status of the compiled program
    (* CHECK-STDOUT: hello, world *)        stdout of the compiled program
    (* CHECK-STDERR: ... *)                 stderr of the compiled program
    (* CHECK-ERR: × expected int, ... *)    compiler diagnostic for fixtures under an error/ directory
                                           (fixtures under a *.unsupported/ directory get only
                                           Poly/ML's CHECK-EXIT and CHECK-STDOUT)
    (* CHECK-STDOUT: (val x (+ 1 2)) *)    syntax tree (--dump-ast) for tests/fixtures/parser,
                                           with CHECK-STDERR for its match warnings
    (* CHECK-STDOUT: fn f0 main() { *)      core IR (--dump-core) for tests/fixtures/core
    (* CHECK-STDOUT: val f : 'a -> 'a *)    inferred types (--dump-types) for tests/fixtures/types
    (* CHECK-STDOUT: 1:9-1:10 exp x : int *) every node's type (--dump-expr-types) for
                                           tests/fixtures/types/nodes
    (* CHECK-RUN-EXIT: 0 *)               native execution of a valid dump fixture
    (* CHECK-RUN-STDOUT: verified *)      program output, separate from its dump
    (* CHECK-RUN-ERR: × unsupported ... *) checked RUNTIME-SKIP rejection
    (* CHECK-REPL: val x = 1 : int *)       REPL transcript for tests/repl fixtures

Later lines of the same stream use the -NEXT suffix (and -EMPTY for blank
lines), so the block reads as a full-line match. The test harnesses strip the
comment wrapper and hand the lines to LLVM's FileCheck.

Program stdout and exit status come from Poly/ML. Compiler diagnostics and
IR dumps come from Nassau. Existing runtime diagnostic and REPL printer
expectations are validated with FileCheck and retained because their wording
and layout belong to Nassau. Nassau's actual behaviour is then
compared with what was written and any disagreement is reported, so a bug is
never silently baked into a fixture.

Usage:
    tools/update_filecheck.py                       # every fixture
    tools/update_filecheck.py 'tests/repl/*.sml'    # a glob
    tools/update_filecheck.py tests/fixtures/basic.sml
    tools/update_filecheck.py --check               # fail if anything is stale
"""

import argparse
import glob
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TESTS = ROOT / "tests"

# Lines this tool owns: generated CHECK lines and the retired exit_code note.
GENERATED = re.compile(
    r"^\(\* (?:CHECK-(?:RUN-)?(?:EXIT|STDOUT|STDERR|REPL|ERR)(?:-[A-Z]+)?:.*|exit_code:.*) \*\)\s*$"
)
ANSI = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")


class ToolError(Exception):
    pass


def escape(text):
    """Escape text for a FileCheck pattern inside an SML comment."""
    out = []
    i = 0
    while i < len(text):
        two = text[i : i + 2]
        char = text[i]
        if ord(char) < 32 or ord(char) == 127:
            # Control characters cannot live in a source comment; a tab also
            # reaches FileCheck as a canonicalised space.
            out.append("{{[[:cntrl:] ]}}")
            i += 1
            continue
        if two == "{{":
            out.append("{{[{]}}{")
            i += 2
            continue
        if two == "[[":
            out.append("{{[[]}}[")
            i += 2
            continue
        if char == "(" and two == "(*":
            out.append("{{[(]}}")
        elif char == ")" and i > 0 and text[i - 1] == "*":
            out.append("{{[)]}}")
        else:
            out.append(char)
        i += 1
    return "".join(out).rstrip()


def stream_lines(prefix, text):
    """FileCheck lines asserting that `text` is exactly the stream (modulo end)."""
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    out = []
    for index, line in enumerate(lines):
        if line == "":
            if index == 0:
                raise ToolError("output starting with a blank line is unsupported")
            out.append(f"(* {prefix}-EMPTY: *)")
        else:
            directive = prefix if index == 0 else f"{prefix}-NEXT"
            out.append(f"(* {directive}: {escape(line)} *)")
    return out


def run(command, stdin=None, cwd=None, timeout=120):
    return subprocess.run(
        command,
        input=stdin,
        cwd=cwd,
        capture_output=True,
        timeout=timeout,
    )


def decode(data):
    return data.decode("utf-8", errors="replace").replace("\r\n", "\n")


def repl_source(source):
    kept = [line for line in source.split("\n") if not line.lstrip().startswith("(*")]
    return "\n".join(kept).rstrip("\n") + "\n"


def directive(fixture, name):
    match = re.search(rf"^\(\* {re.escape(name)}: (.+) \*\)$", fixture.read_text(), re.M)
    return match.group(1) if match else None


def normalise_repl(text, prompt):
    """Strip prompts and trailing blank lines from a REPL transcript."""
    out = []
    for line in text.split("\n"):
        if prompt == "nassau> ":
            line = line.replace("nassau> ", "")
        else:
            line = re.sub(r"^(?:[-=] )+", "", line)
        out.append(line.rstrip())
    while out and out[-1] == "":
        out.pop()
    return "\n".join(out) + "\n"


class Oracle:
    def __init__(self, polyml, nassau):
        self.polyml = polyml
        self.nassau = nassau
        result = run(
            [polyml, "-q"],
            stdin=b'val _ = print ("NASSAU-INT-PRECISION: " ^ (case Int.precision of SOME n => Int.toString n | NONE => "0") ^ "\\n");\n',
        )
        match = re.search(r"NASSAU-INT-PRECISION: (\d+)", decode(result.stdout))
        if result.returncode != 0 or not match:
            raise ToolError("Poly/ML failed to report Int.precision")
        self.precision = int(match.group(1))

    def matching_precision(self, fixture):
        required = directive(fixture, "ORACLE-INT-PRECISION")
        return required is None or int(required) == self.precision

    def skip_reason(self, fixture):
        reason = directive(fixture, "POLYML-SKIP")
        if reason is not None:
            return reason
        if not self.matching_precision(fixture):
            return f"Int.precision is {self.precision}"
        return None

    def reference(self, fixture, echo=False):
        env = dict(os.environ, NASSAU_ORACLE_FILE=str(fixture),
                   NASSAU_ORACLE_ECHO="1" if echo else "0")
        return subprocess.run(
            [self.polyml, "-q", "--script", str(ROOT / "tools/polyml_oracle.sml")],
            input=b"", cwd=fixture.parent, capture_output=True, env=env, timeout=120,
        )

    def polyml_program(self, fixture):
        result = self.reference(fixture)
        return decode(result.stdout), decode(result.stderr), result.returncode

    def rejects(self, fixture):
        result = self.reference(fixture)
        warning = directive(fixture, "POLYML-WARNING")
        return result.returncode != 0 or (
            warning is not None and warning in decode(result.stderr)
        )

    def nassau_compile(self, fixture, extra=()):
        return run(
            [self.nassau, *extra, str(fixture)], stdin=b"", cwd=fixture.parent
        )

    def nassau_program(self, fixture):
        result = self.nassau_compile(fixture)
        if result.returncode != 0:
            raise ToolError(
                "Nassau failed to compile a valid fixture:\n" + decode(result.stderr)
            )
        executable = fixture.with_suffix("")
        try:
            ran = run([str(executable)], stdin=b"", cwd=fixture.parent)
        finally:
            executable.unlink(missing_ok=True)
        return decode(ran.stdout), decode(ran.stderr), ran.returncode


def classify(path):
    parts = path.relative_to(TESTS).parts
    if parts[0] == "repl":
        return "repl"
    if any(part.endswith(".unsupported") for part in parts[:-1]):
        return "unsupported"
    if "error" in parts[:-1]:
        return "error"
    # Valid lexer fixtures are token snapshots (.tokens), not FileCheck.
    if "lexer" in parts[:-1]:
        return "skip"
    if "parser" in parts[:-1]:
        return "parse"
    if "core" in parts[:-1]:
        return "core"
    if "types" in parts[:-1]:
        return "nodes" if "nodes" in parts[:-1] else "types"
    return "run"


def generate_run(oracle, fixture, warnings, prefix="CHECK"):
    reason = oracle.skip_reason(fixture)
    if reason is not None:
        lines = [line for line in fixture.read_text().splitlines() if line.startswith(f"(* {prefix}-") and GENERATED.match(line)]
        if not any(line.startswith(f"(* {prefix}-EXIT:") for line in lines):
            raise ToolError("fixture excluded from Poly/ML needs existing FileCheck expectations")
        actual = oracle.nassau_program(fixture)
        for prefix, stream in zip(
            (f"{prefix}-STDOUT", f"{prefix}-STDERR", f"{prefix}-EXIT"),
            (actual[0], actual[1], str(actual[2]) + "\n"),
        ):
            checks = [line[3:-3] for line in lines if line.startswith(f"(* {prefix}")]
            if not checks:
                if stream:
                    raise ToolError(f"unexpected {prefix} output: {stream!r}")
                continue
            with tempfile.TemporaryDirectory() as directory:
                check_file = Path(directory) / "checks.txt"
                check_file.write_text("\n".join(checks) + "\n")
                result = run(
                    [os.environ.get("FILECHECK", "FileCheck"), str(check_file),
                     f"--check-prefix={prefix}", "--match-full-lines", "--allow-empty"],
                    stdin=stream.encode(),
                )
            if result.returncode != 0:
                raise ToolError("Nassau disagrees with retained expectations:\n" + decode(result.stderr))
        warnings.append(
            f"{rel(fixture)}: skipping Poly/ML ({reason}); "
            "validated and retained existing FileCheck expectations"
        )
        return lines
    stdout, stderr, code = oracle.polyml_program(fixture)
    lines = [f"(* {prefix}-EXIT: {code} *)"]
    if stdout.strip():
        lines += stream_lines(f"{prefix}-STDOUT", stdout)
    actual = oracle.nassau_program(fixture)
    lines += retained_checks(fixture, f"{prefix}-STDERR", actual[1])
    if (actual[0], actual[2]) != (stdout, code):
        warnings.append(
            f"{rel(fixture)}: Nassau disagrees with the oracle "
            f"(nassau exit {actual[2]}, stdout {actual[0]!r}; "
            f"oracle exit {code}, stdout {stdout!r})"
        )
    return lines


def generate_unsupported(oracle, fixture, warnings):
    """Poly/ML's output only; Nassau is expected to fail until the fixture moves."""
    stdout, stderr, code = oracle.polyml_program(fixture)
    if code != 0:
        raise ToolError("Poly/ML rejects this unsupported fixture:\n" + stderr)
    lines = [f"(* CHECK-EXIT: {code} *)"]
    if stdout.strip():
        lines += stream_lines("CHECK-STDOUT", stdout)
    return lines


def generate_runtime(oracle, fixture, warnings):
    reason = directive(fixture, "RUNTIME-SKIP")
    if reason is None:
        return generate_run(oracle, fixture, warnings, "CHECK-RUN")
    compiled = oracle.nassau_compile(fixture, ["--verify"])
    if compiled.returncode == 0:
        fixture.with_suffix("").unlink(missing_ok=True)
        raise ToolError(f"fixture now compiles; remove RUNTIME-SKIP ({reason})")
    checks = retained_checks(fixture, "CHECK-RUN-ERR", decode(compiled.stderr), full_lines=False)
    if not checks:
        raise ToolError("RUNTIME-SKIP requires CHECK-RUN-ERR compiler diagnostics")
    return checks


def generate_error(oracle, fixture, warnings):
    parts = fixture.relative_to(TESTS).parts
    extra = ["--dump-tokens"] if "lexer" in parts else []
    extra += ["--dump-ast"] if "parser" in parts else []
    extra += ["--dump-types"] if "types" in parts else []
    result = oracle.nassau_compile(fixture, extra)
    if result.returncode == 0:
        raise ToolError("Nassau accepted a fixture under error/")
    stderr = ANSI.sub("", decode(result.stderr))
    message = re.search(r"^\s*× (.+)$", stderr, re.M)
    location = re.search(r"\[[^\]\n]*(:\d+:\d+)\]", stderr)
    if not message or not location:
        raise ToolError("unrecognised Nassau diagnostic:\n" + stderr)
    reason = oracle.skip_reason(fixture)
    if reason is None:
        if not oracle.rejects(fixture):
            warnings.append(f"{rel(fixture)}: oracle accepts this error/ fixture")
    else:
        warnings.append(f"{rel(fixture)}: skipping Poly/ML ({reason}); checking Nassau's diagnostic only")
    return [
        f"(* CHECK-ERR: × {escape(message.group(1))} *)",
        f"(* CHECK-ERR: {location.group(1)}] *)",
    ]


def generate_core(oracle, fixture, warnings):
    """Core IR dump from Nassau; Poly/ML only vouches that the file is valid SML."""
    return generate_parse(oracle, fixture, warnings, "--dump-core")


def generate_parse(oracle, fixture, warnings, flag="--dump-ast"):
    """Syntax-tree dump from Nassau; Poly/ML only vouches that the file is valid SML."""
    result = oracle.nassau_compile(fixture, [flag])
    if result.returncode != 0:
        raise ToolError("Nassau rejected a valid fixture:\n" + decode(result.stderr))
    reference = oracle.reference(fixture)
    if reference.returncode != 0:
        warnings.append(f"{rel(fixture)}: Poly/ML rejects this valid fixture")
    stderr = decode(result.stderr)
    expected = decode(reference.stderr).count("not exhaustive")
    actual = stderr.count("warning: match nonexhaustive")
    if actual != expected:
        warnings.append(
            f"{rel(fixture)}: Nassau reports {actual} non-exhaustive matches, Poly/ML {expected}"
        )
    lines = stream_lines("CHECK-STDOUT", decode(result.stdout))
    if stderr.strip():
        lines += stream_lines("CHECK-STDERR", stderr)
    return lines


def polyml_bindings(oracle, fixture):
    """(name, type) of every binding Poly/ML echoes when it loads the fixture."""
    result = oracle.reference(fixture, echo=True)
    bindings = []
    for line in decode(result.stdout).split("\n"):
        line = re.sub(r"^(?:[-=] )+", "", line)
        match = re.match(r"val (\S+) = (.*)$", line)
        if match and ": " in match.group(2):
            bindings.append((match.group(1), match.group(2).rsplit(": ", 1)[1].strip()))
    return bindings


def generate_types(oracle, fixture, warnings):
    """Inferred types from Nassau, cross-checked against what Poly/ML echoes."""
    result = oracle.nassau_compile(fixture, ["--dump-types"])
    if result.returncode != 0:
        raise ToolError("Nassau rejected a valid fixture:\n" + decode(result.stderr))
    stdout = decode(result.stdout)
    ours = [tuple(line[4:].split(" : ", 1)) for line in stdout.splitlines()]
    if ours != polyml_bindings(oracle, fixture):
        warnings.append(f"{rel(fixture)}: Nassau's printed type signatures differ from Poly/ML")
    lines = stream_lines("CHECK-STDOUT", stdout)
    stderr = decode(result.stderr)
    if stderr.strip():
        lines += stream_lines("CHECK-STDERR", stderr)
    return lines


def generate_nodes(oracle, fixture, warnings):
    """The type of every expression and pattern; Poly/ML only vouches that the file is valid."""
    result = oracle.nassau_compile(fixture, ["--dump-expr-types"])
    if result.returncode != 0:
        raise ToolError("Nassau rejected a valid fixture:\n" + decode(result.stderr))
    if not polyml_bindings(oracle, fixture):
        warnings.append(f"{rel(fixture)}: Poly/ML bound nothing in this fixture")
    return stream_lines("CHECK-STDOUT", decode(result.stdout))


def retained_checks(fixture, prefix, stream, full_lines=True):
    lines = [line for line in fixture.read_text().splitlines()
             if line.startswith(f"(* {prefix}")]
    if not lines:
        if stream:
            raise ToolError(f"missing {prefix} expectations for Nassau output")
        return []
    with tempfile.TemporaryDirectory() as directory:
        check_file = Path(directory) / "checks.txt"
        check_file.write_text("\n".join(line[3:-3] for line in lines) + "\n")
        command = [os.environ.get("FILECHECK", "FileCheck"), str(check_file),
                   f"--check-prefix={prefix}", "--allow-empty"]
        if full_lines:
            command.append("--match-full-lines")
        result = run(command, stdin=stream.encode())
    if result.returncode != 0:
        raise ToolError("Nassau disagrees with retained expectations:\n" + decode(result.stderr))
    return lines


def generate_repl(oracle, fixture, warnings):
    reference = oracle.reference(fixture)
    expected_exit = int(directive(fixture, "ORACLE-EXIT") or "0")
    if reference.returncode != expected_exit or "Static Errors" in decode(reference.stderr):
        raise ToolError("Poly/ML disagrees with this REPL fixture:\n" + decode(reference.stderr))
    stdin = repl_source(fixture.read_text()).encode()
    actual = run([oracle.nassau], stdin=stdin, cwd=fixture.parent)
    if actual.returncode != 0:
        raise ToolError("Nassau REPL failed:\n" + decode(actual.stderr))
    transcript = normalise_repl(decode(actual.stdout), "nassau> ")
    checks = retained_checks(fixture, "CHECK-REPL", transcript)
    if "(* CHECK-STDOUT:" in fixture.read_text():
        checks += retained_checks(fixture, "CHECK-STDOUT", transcript)
        retained_checks(fixture, "CHECK-STDOUT", decode(reference.stdout))
    return checks


def rewrite(path, block):
    original = path.read_text()
    kept = [line for line in original.split("\n") if not GENERATED.match(line)]
    body = "\n".join(kept).rstrip("\n")
    return body + "\n" + "\n".join(block) + "\n"


def rel(path):
    return str(path.relative_to(ROOT))


def find_fixtures(patterns):
    if not patterns:
        found = glob.glob(str(TESTS / "fixtures" / "**" / "*.sml"), recursive=True)
        found += glob.glob(str(TESTS / "repl" / "*.sml"))
    else:
        found = []
        for pattern in patterns:
            candidates = [pattern, str(ROOT / pattern)]
            matches = [m for c in candidates for m in glob.glob(c, recursive=True)]
            if not matches:
                raise ToolError(f"no files match {pattern!r}")
            found += matches
    paths = []
    for item in sorted({Path(item).resolve() for item in found}):
        path = Path(item)
        if path.suffix == ".sml" and TESTS in path.parents:
            paths.append(path)
    if not paths:
        raise ToolError("no fixtures selected")
    return paths


def main():
    parser = argparse.ArgumentParser(description=(__doc__ or "").split("\n\n")[0])
    parser.add_argument("paths", nargs="*", help="fixture files or globs (default: all)")
    parser.add_argument("--check", action="store_true", help="do not write; exit 1 if stale")
    parser.add_argument("--nassau", default=str(ROOT / "target" / "debug" / "nassau"))
    parser.add_argument("--polyml", default=os.environ.get("POLYML", "poly"))
    parser.add_argument("--no-build", action="store_true", help="skip cargo build")
    args = parser.parse_args()

    if shutil.which(args.polyml) is None:
        raise ToolError(f"Poly/ML not found ({args.polyml}); set POLYML or install polyml")
    if not args.no_build and args.nassau == str(ROOT / "target" / "debug" / "nassau"):
        subprocess.run(["cargo", "build", "--quiet"], cwd=ROOT, check=True)
    oracle = Oracle(args.polyml, args.nassau)

    generators = {
        "run": generate_run,
        "error": generate_error,
        "unsupported": generate_unsupported,
        "parse": generate_parse,
        "core": generate_core,
        "types": generate_types,
        "nodes": generate_nodes,
        "repl": generate_repl,
    }
    warnings, stale, failed = [], [], []
    for fixture in find_fixtures(args.paths):
        kind = classify(fixture)
        if kind == "skip":
            continue
        try:
            block = generators[kind](oracle, fixture, warnings)
            if kind in {"parse", "core", "types", "nodes"}:
                block += generate_runtime(oracle, fixture, warnings)
        except ToolError as error:
            failed.append(f"{rel(fixture)}: {error}")
            continue
        updated = rewrite(fixture, block)
        if updated != fixture.read_text():
            stale.append(rel(fixture))
            if not args.check:
                fixture.write_text(updated)

    for name in stale:
        print(("stale: " if args.check else "updated: ") + name)
    for warning in warnings:
        print("warning: " + warning, file=sys.stderr)
    for failure in failed:
        print("error: " + failure, file=sys.stderr)
    if failed or (args.check and stale):
        sys.exit(1)


if __name__ == "__main__":
    try:
        main()
    except ToolError as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
