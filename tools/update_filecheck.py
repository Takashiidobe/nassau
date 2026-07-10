#!/usr/bin/env python3
"""Regenerate the FileCheck expectations embedded in Nassau's fixtures.

Every fixture under tests/fixtures and tests/repl carries its expected output
as SML comment lines at the end of the file:

    (* CHECK-EXIT: 18 *)                   exit status of the compiled program
    (* CHECK-STDOUT: hello, world *)        stdout of the compiled program
    (* CHECK-STDERR: ... *)                 stderr of the compiled program
    (* CHECK-ERR: × expected int, ... *)    compiler diagnostic for invalid-* fixtures
    (* CHECK-STDOUT: (val x (+ 1 2)) *)    syntax tree (--dump-ast) for tests/fixtures/parser
    (* CHECK-REPL: val x = 1 : int *)       REPL transcript for tests/repl fixtures

Later lines of the same stream use the -NEXT suffix (and -EMPTY for blank
lines), so the block reads as a full-line match. The test harnesses strip the
comment wrapper and hand the lines to LLVM's FileCheck.

Expectations come from the oracle where SML/NJ defines the behaviour (program
stdout/stderr/exit status, REPL transcripts) and from Nassau itself for
diagnostics, whose wording is Nassau's own. Nassau's actual behaviour is then
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
    r"^\(\* (?:CHECK-(?:EXIT|STDOUT|STDERR|REPL|ERR)(?:-[A-Z]+)?:.*|exit_code:.*) \*\)\s*$"
)
ANSI = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
SMLNJ_NOISE = re.compile(
    r"^(?:Standard ML of New Jersey|\[opening |\[autoloading|\[library |\[scanning |\[parsing )"
)


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


def smlnj_strip_noise(text):
    lines = text.split("\n")
    while lines and SMLNJ_NOISE.match(lines[0]):
        lines.pop(0)
    return "\n".join(lines)


def repl_source(source):
    kept = [line for line in source.split("\n") if not line.lstrip().startswith("(*")]
    return "\n".join(kept).rstrip("\n") + "\n"


def normalise_repl(text, prompt):
    """Strip prompts and trailing blank lines from a REPL transcript."""
    text = smlnj_strip_noise(text)
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
    def __init__(self, smlnj, nassau):
        self.smlnj = smlnj
        self.nassau = nassau

    def smlnj_program(self, fixture):
        """Run a fixture under SML/NJ with the REPL echo of bindings silenced."""
        with tempfile.TemporaryDirectory() as directory:
            wrapper = Path(directory) / "wrapper.sml"
            wrapper.write_text(
                "val _ = Control.Print.out := {say = fn _ => (), flush = fn () => ()};\n"
                f'use "{fixture}";\n'
            )
            result = run([self.smlnj, str(wrapper)], stdin=b"", cwd=fixture.parent)
        return (
            smlnj_strip_noise(decode(result.stdout)),
            decode(result.stderr),
            result.returncode,
        )

    def smlnj_rejects(self, fixture):
        result = run([self.smlnj, str(fixture)], stdin=b"", cwd=fixture.parent)
        return result.returncode != 0

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
    relative = path.relative_to(TESTS)
    parts = relative.parts
    stem = path.stem
    if parts[0] == "repl":
        return "repl"
    if "lexer" in parts:
        # Valid lexer fixtures are token snapshots (.tokens), not FileCheck.
        return "error" if stem.startswith("invalid-") else "skip"
    if "parser" in parts:
        return "error" if stem.startswith("invalid-") else "parse"
    return "error" if stem.startswith("invalid-") else "run"


def generate_run(oracle, fixture, warnings):
    stdout, stderr, code = oracle.smlnj_program(fixture)
    lines = [f"(* CHECK-EXIT: {code} *)"]
    if stdout.strip():
        lines += stream_lines("CHECK-STDOUT", stdout)
    if stderr.strip():
        lines += stream_lines("CHECK-STDERR", stderr)
    actual = oracle.nassau_program(fixture)
    if actual != (stdout, stderr, code):
        warnings.append(
            f"{rel(fixture)}: Nassau disagrees with SML/NJ "
            f"(nassau exit {actual[2]}, stdout {actual[0]!r}; "
            f"SML/NJ exit {code}, stdout {stdout!r})"
        )
    return lines


def generate_error(oracle, fixture, warnings):
    parts = fixture.relative_to(TESTS).parts
    extra = ["--dump-tokens"] if "lexer" in parts else []
    extra += ["--dump-ast"] if "parser" in parts else []
    result = oracle.nassau_compile(fixture, extra)
    if result.returncode == 0:
        raise ToolError("Nassau accepted an invalid-* fixture")
    stderr = ANSI.sub("", decode(result.stderr))
    message = re.search(r"^\s*× (.+)$", stderr, re.M)
    location = re.search(r"\[[^\]\n]*(:\d+:\d+)\]", stderr)
    if not message or not location:
        raise ToolError("unrecognised Nassau diagnostic:\n" + stderr)
    if not oracle.smlnj_rejects(fixture):
        warnings.append(f"{rel(fixture)}: SML/NJ accepts this invalid-* fixture")
    return [
        f"(* CHECK-ERR: × {escape(message.group(1))} *)",
        f"(* CHECK-ERR: {location.group(1)}] *)",
    ]


def generate_parse(oracle, fixture, warnings):
    """Syntax-tree dump from Nassau; SML/NJ only vouches that the file is valid SML."""
    result = oracle.nassau_compile(fixture, ["--dump-ast"])
    if result.returncode != 0:
        raise ToolError("Nassau rejected a valid fixture:\n" + decode(result.stderr))
    if oracle.smlnj_rejects(fixture):
        warnings.append(f"{rel(fixture)}: SML/NJ rejects this valid-* fixture")
    return stream_lines("CHECK-STDOUT", decode(result.stdout))


def generate_repl(oracle, fixture, warnings):
    source = fixture.read_text()
    stdin = repl_source(source).encode()
    reference = run([oracle.smlnj], stdin=stdin, cwd=fixture.parent)
    expected = normalise_repl(decode(reference.stdout), "- ")
    actual = run([oracle.nassau], stdin=stdin, cwd=fixture.parent)
    if normalise_repl(decode(actual.stdout), "nassau> ") != expected:
        warnings.append(f"{rel(fixture)}: Nassau's REPL transcript differs from SML/NJ")
    return stream_lines("CHECK-REPL", expected)


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
    for item in sorted(set(found)):
        path = Path(item).resolve()
        if path.suffix == ".sml" and TESTS in path.parents:
            paths.append(path)
    if not paths:
        raise ToolError("no fixtures selected")
    return paths


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("paths", nargs="*", help="fixture files or globs (default: all)")
    parser.add_argument("--check", action="store_true", help="do not write; exit 1 if stale")
    parser.add_argument("--nassau", default=str(ROOT / "target" / "debug" / "nassau"))
    parser.add_argument("--smlnj", default=os.environ.get("SMLNJ", "smlnj"))
    parser.add_argument("--no-build", action="store_true", help="skip cargo build")
    args = parser.parse_args()

    if shutil.which(args.smlnj) is None:
        raise ToolError(f"SML/NJ not found ({args.smlnj}); set SMLNJ or install smlnj")
    if not args.no_build and args.nassau == str(ROOT / "target" / "debug" / "nassau"):
        subprocess.run(["cargo", "build", "--quiet"], cwd=ROOT, check=True)
    oracle = Oracle(args.smlnj, args.nassau)

    generators = {
        "run": generate_run,
        "error": generate_error,
        "parse": generate_parse,
        "repl": generate_repl,
    }
    warnings, stale, failed = [], [], []
    for fixture in find_fixtures(args.paths):
        kind = classify(fixture)
        if kind == "skip":
            continue
        try:
            block = generators[kind](oracle, fixture, warnings)
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
