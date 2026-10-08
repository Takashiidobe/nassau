import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const bundle = resolve(process.argv[2] ?? 'www/dist/pkg');
const { default: init, BrowserRepl } = await import(pathToFileURL(`${bundle}/nassau.js`));
await init({ module_or_path: await readFile(`${bundle}/nassau_bg.wasm`) });
const decoder = new TextDecoder();
const repl = new BrowserRepl();
const submit = source => {
  const response = repl.submit(source);
  assert.equal(response.diagnostics, '', `${source}\n${response.diagnostics}`);
  return { ...response, text: decoder.decode(Uint8Array.from(response.output)) };
};

assert.equal(submit('val kept = 41;').text, 'val kept = 41 : int\n');
assert.equal(submit('val cell = ref 0;').text, 'val cell = ref 0 : int ref\n');
assert.equal(submit('fun saved () = kept;').text, 'val saved = fn : unit -> int\n');
assert.equal(submit('val kept = 99;').text, 'val kept = 99 : int\n');
assert.equal(submit('saved ();').text, 'val it = 41 : int\n');
assert.match(submit('val broken = (cell := 7; raise Fail "stop");').text, /uncaught exception Fail/);
assert.notEqual(repl.submit('broken;').diagnostics, '');
assert.equal(submit('!cell + saved ();').text, 'val it = 48 : int\n');
assert.equal(submit('Int.toString (~12345) ^ "abc";').text, 'val it = "~12345abc" : string\n');
assert.equal(submit('val eq = ([1,2,3] = [1,2,3]);').text, 'val eq = true : bool\n');
assert.equal(submit('val n = 2.0 + 3.0;').text, 'val n = 5.0 : real\n');
assert.match(submit('1 div 0;').text, /uncaught exception Div/);
assert.equal(submit('saved ();').text, 'val it = 41 : int\n');
assert.equal(submit('fun loop 0 acc = acc | loop n acc = loop (n-1) (acc+1); loop 1000000 0;').text,
  'val loop = fn : int -> int -> int\nval it = 1000000 : int\n');
assert.equal(submit('fun apply f n = f n; fun even 0 = true | even n = apply odd (n-1) and odd 0 = false | odd n = apply even (n-1); even 1000001;').text,
  'val apply = fn : (\'a -> \'b) -> \'a -> \'b\nval even = fn : int -> bool\nval odd = fn : int -> bool\nval it = false : bool\n');
assert.equal(submit('clear;;').clear, true);
assert.equal(submit('saved ();').text, 'val it = 41 : int\n');
assert.equal(submit('reset;;').clear, true);
assert.notEqual(repl.submit('kept;').diagnostics, '');
assert.equal(submit('42;').text, 'val it = 42 : int\n');
assert.equal(submit('Posix.Process.exit (Word8.fromInt 7);').exit, 7);
assert.equal(submit('val ignored = 99;').exit, 7);
assert.equal(submit('reset;;').exit, undefined);
const snippets = await readdir(`${bundle}/snippets`, { recursive: true });
const runtimeFile = snippets.find(name => name.endsWith('/runtime.js'));
assert.ok(runtimeFile);
const { NassauRuntime } = await import(pathToFileURL(`${bundle}/snippets/${runtimeFile}`));
const allocate = NassauRuntime.prototype.allocate;
let runtime;
NassauRuntime.prototype.allocate = function (...args) {
  this.gcInterval = 4096;
  runtime = this;
  return allocate.apply(this, args);
};
const stressed = new BrowserRepl();
const stress = source => {
  const response = stressed.submit(source);
  assert.equal(response.diagnostics, '', response.diagnostics);
  return decoder.decode(Uint8Array.from(response.output));
};
assert.equal(stress('val original = "old" ^ " value"; fun historical () = original; val original = "new";'),
  'val original = "old value" : string\nval historical = fn : unit -> string\nval original = "new" : string\n');
stress('datatype node = End | Node of node ref; val link = ref End; val node = Node link; val _ = link := node;');
stress('fun allocate 0 = () | allocate n = let val cell = ref [n,n+1] in if !cell = [n,n+1] then allocate (n-1) else raise Fail "allocation" end; allocate 100000;');
assert.equal(stress('historical ();'), 'val it = "old value" : string\n');
assert.equal(stress('case !link of Node next => next = link | End => false;'), 'val it = true : bool\n');
assert.ok(runtime.collections > 100, `only ${runtime.collections} collections`);
assert.ok(runtime.top < 2 * 1024 * 1024, `heap grew to ${runtime.top}`);
stress('reset;;');
assert.notEqual(stressed.submit('historical ();').diagnostics, '');
stressed.free();
NassauRuntime.prototype.allocate = allocate;
console.log('browser session fixture passed');

const fixtures = (await readdir('tests/repl')).filter(name => name.endsWith('.sml')).sort();
for (const name of fixtures) {
  const source = await readFile(`tests/repl/${name}`, 'utf8');
  if (source.includes('REPL-COMMANDS') || source.includes('XFAIL')) continue;
  const input = source.split('\n').filter(line => !line.trimStart().startsWith('(*')).join('\n') + '\n';
  const reference = spawnSync(resolve('target/release/nassau'), [], { input, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ifError(reference.error);
  assert.equal(reference.status, 0, `${name}: ${reference.stderr}`);
  const session = new BrowserRepl();
  const response = session.submit(input);
  const text = decoder.decode(Uint8Array.from(response.output));
  const normalize = output => output.replaceAll('nassau> ', '').replace(/^  raised at:.*\n/gm, '').trim();
  assert.equal(normalize(text), normalize(reference.stdout), `${name}: ${response.diagnostics}`);
  assert.equal(Boolean(response.diagnostics), Boolean(reference.stderr.trim()), `${name}: ${response.diagnostics}`);
  session.free();
  console.log(`${name} passed`);
}
repl.free();
