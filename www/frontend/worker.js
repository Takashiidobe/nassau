import init, { BrowserRepl } from './pkg/nassau.js';
import { Wasi } from './wasi.js';

const CAPACITY = 65536;
const shared = self.crossOriginIsolated ? new SharedArrayBuffer(8 + CAPACITY) : null;
const control = shared && new Int32Array(shared, 0, 2);
let wasi, leftover = new Uint8Array(0);

function readStdin(wanted) {
  if (!shared) return new Uint8Array(0);
  if (!leftover.length) {
    const output = wasi.takeOutput();
    if (output.length) postMessage({ type: 'output', bytes: output }, [output.buffer]);
    Atomics.store(control, 0, 0);
    postMessage({ type: 'stdin' });
    Atomics.wait(control, 0, 0);
    leftover = new Uint8Array(shared, 8, control[1]).slice();
  }
  const chunk = leftover.subarray(0, wanted);
  leftover = leftover.subarray(chunk.length);
  return chunk;
}

function createWasi() {
  wasi = new Wasi();
  wasi.stdin = readStdin;
  leftover = new Uint8Array(0);
  return wasi;
}

let repl;
try {
  const [, runtime] = await Promise.all([
    init(),
    WebAssembly.compileStreaming(fetch(new URL('./pkg/nassau_runtime.wasm', import.meta.url))),
  ]);
  repl = new BrowserRepl({ module: runtime, createWasi });
  postMessage({ type: 'ready', stdin: shared });
} catch (error) {
  postMessage({ type: 'fatal', message: `Could not load Nassau: ${error.message ?? error}` });
}

onmessage = ({ data }) => {
  if (!repl) return;
  if (data.type === 'complete') {
    try {
      postMessage({ type: 'completions', requestId: data.requestId, names: repl.completions(data.prefix) });
    } catch (error) {
      postMessage({ type: 'completions', requestId: data.requestId, names: [], error: error.message ?? String(error) });
    }
    return;
  }
  if (data.type !== 'submit') return;
  const start = performance.now();
  try {
    const response = repl.submit(data.source);
    postMessage({ type: 'result', response, elapsed: performance.now() - start });
  } catch (error) {
    repl = null;
    postMessage({ type: 'fatal', message: `Execution stopped: ${error.message ?? error}. Reset the session to continue.` });
  }
};
