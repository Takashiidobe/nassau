import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const bundle = resolve(process.argv[2] ?? 'www/dist/pkg');
const { default: init, BrowserRepl } = await import(pathToFileURL(`${bundle}/nassau.js`));
await init({ module_or_path: await readFile(`${bundle}/nassau_bg.wasm`) });
const { Wasi } = await import(pathToFileURL(`${bundle}/../wasi.js`));
const module = await WebAssembly.compile(await readFile(`${bundle}/nassau_runtime.wasm`));
const host = env => ({ module, createWasi: () => new Wasi(env) });
const decoder = new TextDecoder();
const repl = new BrowserRepl(host({}));
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
const stressed = new BrowserRepl(host({ NASSAU_GC_STRESS: '20' }));
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
stress('reset;;');
assert.notEqual(stressed.submit('historical ();').diagnostics, '');
stressed.free();
for (const name of ['textio-files', 'filesys', 'filesys-meta', 'math-digits']) {
  const fixture = await readFile(`tests/fixtures/basis/${name}.sml`, 'utf8');
  const expectedLines = [...fixture.matchAll(/\(\* CHECK-STDOUT(?:-NEXT)?: (.*) \*\)/g)].map(match => match[1]);
  assert.ok(expectedLines.length > 10);
  const files = new BrowserRepl(host({ PATH: '/usr/bin' }));
  const filesResponse = files.submit(fixture.replace(/\(\*[\s\S]*?\*\)/g, ''));
  assert.equal(filesResponse.diagnostics, '');
  const filesOutput = decoder.decode(Uint8Array.from(filesResponse.output)).split('\n');
  let cursor = 0;
  for (const line of expectedLines) {
    const found = filesOutput.indexOf(line, cursor);
    assert.notEqual(found, -1, `${name}: browser output is missing "${line}"`);
    cursor = found + 1;
  }
  files.free();
}
const processes = new BrowserRepl(host({}));
const slept = processes.submit(`val started = Time.now ();
val _ = OS.Process.sleep (Time.fromMilliseconds 150);
val elapsed = Time.toMilliseconds (Time.- (Time.now (), started));
val _ = print (Bool.toString (elapsed >= 100 andalso elapsed < 2000) ^ "\\n");
val system = (OS.Process.system "echo hi"; "ran") handle OS.SysErr (message, _) => message;
val _ = OS.Process.atExit (fn () => print "hook\\n");
val _ = OS.Process.exit OS.Process.success;`);
assert.equal(slept.diagnostics, '');
assert.equal(slept.exit, 0);
const sleptOutput = decoder.decode(Uint8Array.from(slept.output));
assert.match(sleptOutput, /\ntrue\n/);
assert.match(sleptOutput, /val system = "[^"]+" : string\nhook\n$/);
assert.ok(!sleptOutput.includes('"ran"'));
processes.free();
for (const name of ['os-process-signature', 'os-process-nested-exit', 'os-process-hook-terminate', 'os-process-cleanup']) {
  const fixture = await readFile(`tests/fixtures/runtime/${name}.sml`, 'utf8');
  const session = new BrowserRepl(host({}));
  const response = session.submit(fixture.replace(/\(\*[\s\S]*?\*\)/g, '') + '\nval _ = OS.Process.exit OS.Process.success;');
  assert.equal(response.diagnostics, '', name);
  if (name !== 'os-process-signature' && name !== 'os-process-cleanup') {
    assert.equal(response.exit, 1, name);
  }
  const output = decoder.decode(Uint8Array.from(response.output));
  const lines = output.split('\n');
  let cursor = 0;
  for (const match of fixture.matchAll(/\(\* CHECK-STDOUT(?:-NEXT)?: (.*) \*\)/g)) {
    const found = lines.indexOf(match[1], cursor);
    assert.notEqual(found, -1, `${name}: missing "${match[1]}"`);
    cursor = found + 1;
  }
  assert.ok(!output.includes('new hook\n'), name);
  assert.ok(!output.includes('skipped\n'), name);
  assert.ok(!output.includes('returned from'), name);
  session.free();
}
const typed = ['first line\nsecond', ' half\n'];
const input = new BrowserRepl({
  module,
  createWasi: () => Object.assign(new Wasi(), {
    stdin: () => (typed.length ? new TextEncoder().encode(typed.shift()) : new Uint8Array(0)),
  }),
});
const read = input.submit(`val a = TextIO.inputLine TextIO.stdIn;
val b = TextIO.inputLine TextIO.stdIn;
val c = TextIO.inputLine TextIO.stdIn;`);
assert.equal(read.diagnostics, '');
assert.equal(decoder.decode(Uint8Array.from(read.output)),
  'val a = SOME "first line\\n" : string option\nval b = SOME "second half\\n" : string option\nval c = NONE : string option\n');
input.free();
console.log('browser session fixture passed');

const fixtures = (await readdir('tests/repl')).filter(name => name.endsWith('.sml')).sort();
for (const name of fixtures) {
  const source = await readFile(`tests/repl/${name}`, 'utf8');
  if (source.includes('REPL-COMMANDS') || source.includes('XFAIL')) continue;
  const input = source.split('\n').filter(line => !line.trimStart().startsWith('(*')).join('\n') + '\n';
  const reference = spawnSync(resolve('target/release/nassau'), [], { input, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ifError(reference.error);
  assert.equal(reference.status, 0, `${name}: ${reference.stderr}`);
  const session = new BrowserRepl(host({}));
  const response = session.submit(input);
  const text = decoder.decode(Uint8Array.from(response.output));
  const normalize = output => output.replaceAll('nassau> ', '').replace(/^  raised at:.*\n/gm, '').trim();
  assert.equal(normalize(text), normalize(reference.stdout), `${name}: ${response.diagnostics}`);
  assert.equal(Boolean(response.diagnostics), Boolean(reference.stderr.trim()), `${name}: ${response.diagnostics}`);
  session.free();
  console.log(`${name} passed`);
}
repl.free();
