import init, { BrowserRepl } from './pkg/nassau.js';

let repl;
try {
  await init();
  repl = new BrowserRepl();
  postMessage({ type: 'ready' });
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
