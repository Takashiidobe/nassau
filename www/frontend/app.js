const $ = id => document.getElementById(id);
const transcript = $('transcript');
const decoder = new TextDecoder();
const history = [];
let editor, inputRow, worker, ready = false, busy = false, exited = false, activeEntry;
let historyIndex = 0, draft = '', exampleRequest = 0, promptId = 0;
let completionRequestId = 0, completionCycle = null;
const completionRequests = new Map();
let resumeVimInsert = false;
const storage = {
  get(key) { try { return localStorage.getItem(key); } catch { return null; } },
  set(key, value) { try { localStorage.setItem(key, value); } catch {} }
};
const savedKeymap = storage.get('nassau:keymap');
$('keymap').value = ['vim', 'emacs'].includes(savedKeymap) ? savedKeymap : 'default';

function scroll() { transcript.scrollTop = transcript.scrollHeight; }
function controls() {
  $('stop').hidden = !busy;
  $('examples').disabled = busy;
}
function status(text, kind = '') { $('status').textContent = text; $('status').className = kind; }
function configureEditor(textarea) {
  const cm = CodeMirror.fromTextArea(textarea, {
    mode: 'sml', theme: 'material-darker', keyMap: $('keymap').value,
    indentUnit: 2, tabSize: 2, viewportMargin: Infinity,
    extraKeys: {
      'Ctrl-Enter': run, 'Cmd-Enter': run,
      'Ctrl-Up': () => recall(-1), 'Ctrl-Down': () => recall(1),
      Tab: complete
    }
  });
  cm.on('vim-mode-change', ({ mode, subMode }) => {
    if (cm === editor) $('vim-mode').textContent = `${mode}${subMode ? ' · ' + subMode : ''}`;
  });
  cm.on('change', (cm, change) => {
    if (cm !== editor) return;
    if (change.origin !== 'complete') completionCycle = null;
    scroll();
    if (change.origin === '+input' && endsCommand(cm)) {
      queueMicrotask(() => { if (cm === editor && endsCommand(cm)) run(); });
    }
  });
  cm.getWrapperElement().setAttribute('aria-label', 'Standard ML input');
  return cm;
}
function endsCommand(cm) {
  const cursor = cm.getCursor();
  const before = cm.getRange({ line: 0, ch: 0 }, cursor);
  if (!before.endsWith(';;')) return false;
  if (cm.getRange(cursor, { line: cm.lastLine(), ch: cm.getLine(cm.lastLine()).length }).trim()) return false;
  const token = cm.getTokenAt(cursor, true);
  return token.type !== 'string' && token.type !== 'comment';
}
function newPrompt(source = '') {
  inputRow = document.createElement('div'); inputRow.className = 'input-row';
  const label = document.createElement('label'); label.className = 'prompt'; label.textContent = 'nassau>';
  const shell = document.createElement('div'); shell.className = 'editor-shell';
  const textarea = document.createElement('textarea');
  textarea.id = `source-${++promptId}`; textarea.spellcheck = false; textarea.value = source;
  label.htmlFor = textarea.id; shell.append(textarea); inputRow.append(label, shell);
  transcript.append(inputRow);
  editor = configureEditor(textarea);
  historyIndex = history.length; draft = source;
  if ($('keymap').value === 'vim' && resumeVimInsert) CodeMirror.Vim.handleKey(editor, 'i');
  updateMode(); controls(); editor.focus(); scroll();
}
function updateMode() {
  $('vim-mode').textContent = $('keymap').value === 'vim'
    ? editor.state.vim?.insertMode ? 'insert' : 'normal'
    : '';
}
function recall(direction) {
  if (busy) return;
  if (historyIndex === history.length) draft = editor.getValue();
  historyIndex = Math.max(0, Math.min(history.length, historyIndex + direction));
  editor.setValue(historyIndex === history.length ? draft : history[historyIndex]);
  editor.setCursor(editor.lineCount() - 1); editor.focus(); controls();
}
function complete(cm) {
  const cursor = cm.getCursor();
  if (completionCycle && completionCycle.cm === cm && completionCycle.cursor.line === cursor.line && completionCycle.cursor.ch === cursor.ch) {
    const next = (completionCycle.index + 1) % completionCycle.names.length;
    applyCompletion(cm, completionCycle.from, cursor, completionCycle.names[next]);
    completionCycle.index = next;
    completionCycle.value = completionCycle.names[next];
    completionCycle.cursor = cm.getCursor();
    return;
  }
  const before = cm.getLine(cursor.line).slice(0, cursor.ch);
  const match = before.match(/[A-Za-z0-9_'.]+$/);
  const prefix = match?.[0] ?? '';
  if (!prefix) { cm.replaceSelection('  '); return; }
  const requestId = ++completionRequestId;
  completionRequests.set(requestId, { cm, cursor, from: { line: cursor.line, ch: cursor.ch - prefix.length }, prefix, source: cm.getValue() });
  worker.postMessage({ type: 'complete', requestId, prefix });
}
function applyCompletion(cm, from, cursor, value) {
  cm.replaceRange(value, from, cursor, 'complete');
  cm.setCursor({ line: from.line, ch: from.ch + value.length });
}
function commonPrefix(names) {
  let prefix = names[0] ?? '';
  for (const name of names.slice(1)) {
    let length = 0;
    while (length < prefix.length && prefix[length] === name[length]) length++;
    prefix = prefix.slice(0, length);
  }
  return prefix;
}
function receiveCompletions(data) {
  const request = completionRequests.get(data.requestId);
  completionRequests.delete(data.requestId);
  if (!request || request.cm !== editor || request.cm.getValue() !== request.source) return;
  const cursor = request.cm.getCursor();
  if (cursor.line !== request.cursor.line || cursor.ch !== request.cursor.ch) return;
  const names = [...new Set(data.names)].filter(name => name.startsWith(request.prefix) && name !== request.prefix);
  if (!names.length) { applyCompletion(request.cm, request.from, request.cursor, request.prefix + '  '); return; }
  completionCycle = { cm: request.cm, from: request.from, names, index: -1, cursor: request.cursor, value: request.prefix };
  const prefix = commonPrefix(names);
  if (prefix.length > request.prefix.length) {
    applyCompletion(request.cm, request.from, request.cursor, prefix);
    completionCycle.value = prefix;
    completionCycle.cursor = request.cm.getCursor();
  } else {
    const next = 0;
    applyCompletion(request.cm, request.from, request.cursor, names[next]);
    completionCycle.index = next;
    completionCycle.value = names[next];
    completionCycle.cursor = request.cm.getCursor();
  }
}
function append(parent, text, kind) {
  const pre = document.createElement('pre'); pre.className = kind;
  if (kind === 'output') {
    pre.classList.add('cm-s-material-darker');
    CodeMirror.runMode(text, 'sml', pre, { tabSize: 2 });
  } else pre.textContent = text;
  parent.append(pre); scroll();
}
function notice(text, top = false) {
  const pre = document.createElement('pre'); pre.className = 'notice'; pre.textContent = text + '\n';
  if (top) transcript.prepend(pre);
  else transcript.insertBefore(pre, inputRow?.parentElement === transcript ? inputRow : null);
  scroll();
}
function fail(message) {
  ready = busy = false;
  if (activeEntry) { append(activeEntry, message, 'diagnostics'); activeEntry = null; newPrompt(); }
  else notice(message);
  status('Reset to retry', 'error'); controls();
}
function startWorker() {
  worker?.terminate();
  ready = busy = exited = false; activeEntry = null;
  status('Loading…'); controls();
  try { worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' }); }
  catch (error) { fail(`Could not start Nassau: ${error.message}`); return; }
  const currentWorker = worker;
  worker.onmessage = ({ data }) => {
    if (worker !== currentWorker) return;
    if (data.type === 'ready') { ready = true; status(''); }
    else if (data.type === 'fatal') { fail(data.message); return; }
    else if (data.type === 'completions') { receiveCompletions(data); return; }
    else if (data.type === 'result') {
      const { output, diagnostics, exit, clear } = data.response;
      if (clear) { transcript.replaceChildren(); activeEntry = transcript; }
      const bytes = new Uint8Array(output);
      if (bytes.length) append(activeEntry, decoder.decode(bytes), clear ? 'notice' : 'output');
      if (diagnostics) append(activeEntry, diagnostics, 'diagnostics');
      exited = exit !== undefined && exit !== null;
      if (exited) append(activeEntry, `Process exited with status ${exit}. Reset to continue.\n`, 'notice');
      busy = false; activeEntry = null;
      newPrompt();
      status(exited ? `Exited (${exit})` : diagnostics ? 'Check diagnostics' : '', diagnostics ? 'error' : 'ok');
    }
    controls();
  };
  worker.onerror = event => {
    if (worker !== currentWorker) return;
    event.preventDefault(); fail(event.message || 'Worker failed. Reset to retry.');
  };
  worker.onmessageerror = () => { if (worker === currentWorker) fail('Could not read worker response. Reset to retry.'); };
}
function run() {
  const source = editor.getValue();
  if (!ready || busy || (exited && !/^\s*(?:clear|reset)\s*;;\s*$/.test(source))) return;
  if (!source.trim()) { editor.focus(); return; }
  history.push(source); historyIndex = history.length;
  resumeVimInsert = $('keymap').value === 'vim' && Boolean(editor.state.vim?.insertMode);
  editor.toTextArea();
  const submitted = document.createElement('pre');
  submitted.className = 'submitted cm-s-material-darker';
  CodeMirror.runMode(source.trimEnd(), 'sml', submitted, { tabSize: 2 });
  inputRow.querySelector('.editor-shell').replaceChildren(submitted);
  inputRow.querySelector('.prompt').removeAttribute('for');
  activeEntry = document.createElement('div'); activeEntry.className = 'entry';
  inputRow.replaceWith(activeEntry); activeEntry.append(inputRow);
  busy = true; status('Running…'); controls();
  worker.postMessage({ type: 'submit', source: source + '\n' });
}
editor = configureEditor($('source')); inputRow = editor.getWrapperElement().closest('.input-row');
updateMode();
CodeMirror.Vim.defineEx('write', 'w', run);
CodeMirror.Vim.defineEx('submit', 'sub', run);
$('keymap').onchange = () => {
  editor.setOption('keyMap', $('keymap').value);
  storage.set('nassau:keymap', $('keymap').value); updateMode(); editor.focus();
};
function reset(stopped = false) {
  resumeVimInsert = busy ? resumeVimInsert : $('keymap').value === 'vim' && Boolean(editor.state.vim?.insertMode);
  worker?.terminate();
  if (!busy) editor.toTextArea();
  busy = ready = exited = false; activeEntry = null;
  transcript.replaceChildren(); newPrompt();
  notice(stopped ? 'Stopped. Reset.' : 'Reset', true);
  startWorker();
}
$('reset').onclick = () => reset();
$('stop').onclick = () => reset(true);
$('clear').onclick = () => {
  const keep = busy ? activeEntry : inputRow;
  transcript.replaceChildren(keep);
  notice('Cleared', true);
  if (!busy) editor.refresh();
  scroll();
};
$('examples').onchange = async () => {
  const name = $('examples').value;
  if (!name) return;
  const request = ++exampleRequest;
  try {
    const response = await fetch(`./examples/${name}.sml`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const source = await response.text();
    if (request !== exampleRequest || busy) return;
    editor.setValue(source); historyIndex = history.length; draft = source;
    controls(); editor.focus();
  } catch (error) { notice(`Could not load example: ${error.message}`); }
};
$('share').onclick = async () => {
  const url = new URL(location.href); url.hash = 'code=' + encodeURIComponent(editor.getValue());
  window.history.replaceState(null, '', url);
  try { await navigator.clipboard.writeText(url.href); status('Link copied', 'ok'); }
  catch { status('Link is in the address bar'); }
};
if (location.hash.startsWith('#code=')) {
  try { editor.setValue(decodeURIComponent(location.hash.slice(6))); }
  catch { notice('Could not read the source in this link.'); }
}
startWorker(); editor.focus();
