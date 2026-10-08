const NIL = 1n;
const STACK_TOP = 0x100000;
const EXCEPTIONS = ['Div', 'Overflow', 'Match', 'Bind', 'Fail', 'Subscript', 'Empty', 'Size'];

export class NassauRuntime {
  constructor() {
    this.memory = new WebAssembly.Memory({ initial: 32, maximum: 4096 });
    this.table = new WebAssembly.Table({ element: 'anyfunc', initial: 1 });
    this.stack_pointer = new WebAssembly.Global({ value: 'i32', mutable: true }, STACK_TOP);
    this.top = STACK_TOP;
    this.blocks = new Map();
    this.free = [];
    this.globals = new Set();
    this.codeGlobals = new Map();
    this.identities = [];
    this.hostRoots = [];
    this.frame = 0;
    this.uncaught = 0n;
    this.output = [];
    this.exit = undefined;
    this.allocated = 0;
    this.collections = 0;
    this.gcInterval = 4 * 1024 * 1024;
    this.exitSignal = {};
    this.env = {
      memory: this.memory,
      table: this.table,
      stack_pointer: this.stack_pointer,
      nassau_alloc: (length, kind) => this.allocate(length, kind),
      nassau_print: string => {
        this.output.push(this.read_bytes(Number(string) + 8, this.length(string)));
        return NIL;
      },
      nassau_exception: index => this.builtin(Number(index >> 1n)),
      nassau_raised: () => 8n,
      nassau_uncaught: () => {
        this.uncaught = this.read_word(8);
        this.writeWord(8, 0n);
        return 0;
      },
      nassau_exit: status => {
        this.exit = Number((status >> 1n) & 255n);
        throw this.exitSignal;
      },
      nassau_roots_push: (address, count, code) => {
        const frame = Number(address);
        if (frame < 4096 || frame + Number(count + 3n) * 8 > STACK_TOP) {
          throw new Error('Nassau shadow stack exhausted');
        }
        this.writeWord(frame, BigInt(this.frame));
        this.writeWord(frame + 8, count);
        this.writeWord(frame + 16, code);
        this.frame = frame;
      },
      nassau_roots_pop: address => {
        if (Number(address) !== this.frame) throw new Error('Unbalanced Nassau roots');
        this.frame = Number(this.read_word(this.frame));
      },
      nassau_global_root: address => this.globals.add(Number(address)),
      nassau_code_global: (code, address) => {
        const key = Number(code);
        if (!this.codeGlobals.has(key)) this.codeGlobals.set(key, new Set());
        this.codeGlobals.get(key).add(Number(address));
      },
    };
  }

  read_word(address) {
    return new DataView(this.memory.buffer).getBigInt64(address, true);
  }

  writeWord(address, value) {
    new DataView(this.memory.buffer).setBigInt64(address, value, true);
  }

  read_real(address) {
    return new DataView(this.memory.buffer).getFloat64(address, true);
  }

  read_bytes(address, length) {
    return new Uint8Array(this.memory.buffer, address, length).slice();
  }

  length(value) {
    return Number(this.read_word(Number(value)) >> 8n);
  }

  allocate_static(size, align) {
    if (!Number.isInteger(size) || size < 0 || !Number.isInteger(align) || align < 1 || (align & (align - 1))) {
      throw new Error('Invalid Nassau allocation');
    }
    const address = Math.ceil(this.top / align) * align;
    const end = address + Math.max(size, 8);
    if (end > 4096 * 65536) throw new Error('Nassau heap exhausted');
    const pages = Math.ceil(end / 65536) - this.memory.buffer.byteLength / 65536;
    if (pages > 0) this.memory.grow(pages);
    this.top = end;
    return address;
  }

  allocate(length, kind) {
    const count = Number(length);
    const tag = Number(kind);
    if (!Number.isSafeInteger(count) || count < 0 || ![0, 1, 2, 3, 4].includes(tag)) {
      throw new Error('Invalid Nassau object');
    }
    const size = Math.ceil((8 + (tag === 2 ? count + 1 : count * 8)) / 8) * 8;
    if (this.allocated + size >= this.gcInterval) this.collect();
    let address;
    const index = this.free.findIndex(block => block.size >= size);
    if (index >= 0) {
      const block = this.free[index];
      address = block.address;
      if (block.size === size) this.free.splice(index, 1);
      else { block.address += size; block.size -= size; }
    } else {
      address = this.allocate_static(size, 8);
    }
    new Uint8Array(this.memory.buffer, address, size).fill(0);
    this.writeWord(address, (length << 8n) | kind);
    this.blocks.set(address, size);
    this.allocated += size;
    return BigInt(address);
  }

  builtin(index) {
    if (!EXCEPTIONS[index]) throw new Error('Invalid Nassau exception');
    if (this.identities[index]) return this.identities[index];
    const identity = this.allocate(1n, 4n);
    this.hostRoots.push(identity);
    try {
      const bytes = new TextEncoder().encode(EXCEPTIONS[index]);
      const name = this.allocate(BigInt(bytes.length), 2n);
      new Uint8Array(this.memory.buffer, Number(name) + 8, bytes.length).set(bytes);
      this.writeWord(Number(identity) + 8, name);
      this.identities[index] = identity;
      return identity;
    } finally {
      this.hostRoots.pop();
    }
  }

  collect() {
    const pending = [...this.hostRoots, ...this.identities.filter(Boolean), this.read_word(8), this.uncaught];
    const marked = new Set();
    const codes = new Set();
    const traceCode = code => {
      if (codes.has(code)) return;
      codes.add(code);
      for (const address of this.codeGlobals.get(code) ?? []) pending.push(this.read_word(address));
    };
    for (const address of this.globals) pending.push(this.read_word(address));
    for (let frame = this.frame; frame; frame = Number(this.read_word(frame))) {
      traceCode(Number(this.read_word(frame + 16)));
      const count = Number(this.read_word(frame + 8));
      for (let i = 0; i < count; i++) pending.push(this.read_word(frame + 24 + i * 8));
    }
    while (pending.length) {
      const value = pending.pop();
      if (value === 0n || (value & 1n)) continue;
      const address = Number(value);
      if (!this.blocks.has(address) || marked.has(address)) continue;
      marked.add(address);
      const header = this.read_word(address);
      const kind = Number(header & 255n);
      const length = Number(header >> 8n);
      if (kind === 1) traceCode(Number(this.read_word(address + 8)));
      if (kind === 0 || kind === 1 || kind === 4) {
        for (let i = kind === 1 ? 1 : 0; i < length; i++) {
          pending.push(this.read_word(address + 8 + i * 8));
        }
      }
    }
    for (const [address, size] of this.blocks) {
      if (!marked.has(address)) {
        this.blocks.delete(address);
        this.free.push({ address, size });
      }
    }
    this.free.sort((a, b) => a.address - b.address);
    const merged = [];
    for (const block of this.free) {
      const last = merged.at(-1);
      if (last && last.address + last.size === block.address) last.size += block.size;
      else merged.push({ ...block });
    }
    this.free = merged;
    this.allocated = 0;
    this.collections++;
  }

  instantiate(bytes, names) {
    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, { env: this.env });
    if (this.table.length < names.length + 1) this.table.grow(names.length + 1 - this.table.length);
    for (let index = 0; index < names.length; index++) {
      const name = names[index];
      this.env[name] = instance.exports[name];
      this.table.set(index + 1, instance.exports[name]);
    }
  }

  execute(entry) {
    this.uncaught = 0n;
    this.writeWord(8, 0n);
    this.exit = undefined;
    try {
      return this.env[entry]();
    } catch (error) {
      if (error !== this.exitSignal) throw error;
      return 0;
    } finally {
      this.frame = 0;
      this.stack_pointer.value = STACK_TOP;
    }
  }

  take_uncaught() {
    const exception = this.uncaught;
    this.uncaught = 0n;
    return exception;
  }

  exit_status() {
    return this.exit ?? -1;
  }

  retain_globals(addresses) {
    this.globals = new Set(addresses);
  }

  take_output() {
    const bytes = new Uint8Array(this.output.reduce((length, part) => length + part.length, 0));
    let offset = 0;
    for (const part of this.output) { bytes.set(part, offset); offset += part.length; }
    this.output = [];
    return bytes;
  }

  push_host(value) { this.hostRoots.push(value); }
  pop_host() { this.hostRoots.pop(); }
}
