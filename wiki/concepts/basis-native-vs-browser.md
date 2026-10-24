# Basis on native and in the browser

Native and browser builds compile the same Basis sources (`basis/sources`)
with the same Cranelift emitter, and link the same `runtime/` Rust. Native
links it directly; the browser builds it for `wasm32-wasip1` and supplies the
WASI imports from `www/frontend/wasi.js`. See [browser-codegen](browser-codegen.md)
for the mechanism. This page records where behaviour is the same and where
it is not.

## Identical

Modules that never reach a `Prim.os*` or `Prim.math*` extern are the same
compiled code on both targets:

- `General`, `Bool`, `Option`, `Char`, `String`, `Substring`, `StringCvt`, `Byte`
- `Int`, `LargeInt`, `Position`, `Word8`, `Real`, `LargeReal`, `IEEEReal`
- `Time`, except `Time.now`
- `List`, `ListPair`, `Vector`, `Array`, `Array2` and the slice modules
- the `CharVector`, `CharArray`, `Word8Vector` and `Word8Array` families
- the `IO`, `STREAM_IO` and `IMPERATIVE_IO` signatures and the `ImperativeIO` functor
- `OS.Path`, and the `OS.Process` exit, `atExit` and `terminate` logic

`tests/fixtures/runtime/browser-session.mjs` runs every `tests/repl/*.sml`
fixture in the browser and compares the output with native.

## Same code, different host

These go through the `nassau_os_*` externs in `runtime/os.rs`. Native uses the
host OS. The browser uses an in-memory filesystem preopened at `/`.

- `TextIO` file open, read, write and close
- `OS.FileSys`: remove, rename, directory operations, `chDir`, `getDir`,
  `fullPath`, `fileSize`, `modTime`, `setTime`, `access`, `fileId`, `tmpName`
- `OS.Process.getEnv`, `OS.Process.sleep` and `Time.now`

`textio-files`, `filesys`, `filesys-meta` and the `os-process-*` fixtures run on
both targets.

## Math

`Math` functions other than `sqrt` call the `nassau_math_*` exports in
`runtime/math.rs`, which wrap the pure-Rust `libm` crate. Native and browser run
the same code, so results are bit-identical, and `sqrt` is inline on both.
`basis/math-digits.sml` prints 15 fractional digits and runs on both targets.
`libm` differs from glibc by 1 ulp for some inputs (`cosh 10`, `pow (10, 2.5)`),
so that fixture is `POLYML-SKIP`.

## Differences

| Area | Native | Browser |
|---|---|---|
| `TextIO.stdIn` | Process stdin | Blocking `stdin>` prompt over `SharedArrayBuffer` and `Atomics.wait`. The page needs the COOP/COEP headers in `www/netlify.toml`. |
| `OS.Process.system` | Runs `sh -c` | Raises `SysErr` with ENOSYS. |
| `FileSys.isLink`, `readLink` | Real symlinks | No symlinks in `wasi.js`: `isLink` is always false and `readLink` fails. |
| Filesystem and environment | Host disk and environment | Both start empty and nothing persists across sessions. |
| `OS.Process.sleep` | Thread sleep | `Atomics.wait` in the worker, a busy-wait without `SharedArrayBuffer`. |

## Untested

- Only the queued-input path of browser stdin is tested, in Node. The prompt in
  a real browser is `nassau-i4g`.

## Not implemented on either target

No file in `basis/` defines these:

- `Word` (only `Word8`), `Word31`, `Word32`, `Word64`, `LargeWord`
- `Date`, `Timer`, `CommandLine`, `BinIO`, `Text`
- `Unix`, `Socket`, and `NetHostDB` beyond its signature
- `Posix` beyond `Posix.Process.exit`

Optional-component coverage is tracked by `nassau-617.5`, and the remaining
`TextIO` and `StreamIO` work by `nassau-bda`.

## Testing

Nextest has a `native` profile (also the default) and a `wasm` profile. `wasm`
runs program fixtures against the browser bundle in Node
(`tests/wasm.rs`, `tests/wasm/run-fixture.mjs`) after `./www/build.sh`. It skips
fixtures marked `WASM-SKIP`: uncaught-exception fixtures (the REPL keeps going
where a program exits 1), FileCheck regex fixtures, `basis/process.sml`, and four
fixtures that trap on deep non-tail recursion (`nassau-gph`).

## Adding an OS service

Add a row to `EXTERN_FNS` in `src/core.rs`, a function in `runtime/os.rs`, and
an entry in `runtime::symbols()` for the JIT. The browser picks up every
`nassau_*` export automatically, but `wasi.js` must implement any WASI call the
service needs. Cover both targets with a fixture in `tests/fixtures/basis` and
add it to the list in `browser-session.mjs`.
