const STACK_TOP = 0x100000;

export class NassauRuntime {
  constructor(host) {
    this.wasi = host.createWasi();
    const instance = new WebAssembly.Instance(host.module, {
      wasi_snapshot_preview1: this.wasi.imports(() => this.memory),
    });
    this.exports = instance.exports;
    this.memory = this.exports.memory;
    this.exports._initialize();
    this.exports.nassau_repl();
    this.table = new WebAssembly.Table({ element: 'anyfunc', initial: 1 });
    this.stack_pointer = new WebAssembly.Global({ value: 'i32', mutable: true }, STACK_TOP);
    const { exports } = this;
    this.env = {
      memory: this.memory,
      table: this.table,
      stack_pointer: this.stack_pointer,
      sin: Math.sin,
      cos: Math.cos,
      tan: Math.tan,
      asin: Math.asin,
      acos: Math.acos,
      atan: Math.atan,
      atan2: Math.atan2,
      exp: Math.exp,
      log: Math.log,
      log10: Math.log10,
      sinh: Math.sinh,
      cosh: Math.cosh,
      tanh: Math.tanh,
      pow: (x, y) => (x === 1 || (x === -1 && !Number.isFinite(y) && !Number.isNaN(y)) ? 1 : Math.pow(x, y)),
    };
    for (const [name, value] of Object.entries(exports)) {
      if (name.startsWith('nassau_')) this.env[name] = value;
    }
  }

  read_word(address) {
    return new DataView(this.memory.buffer).getBigInt64(address, true);
  }

  read_real(address) {
    return new DataView(this.memory.buffer).getFloat64(address, true);
  }

  read_bytes(address, length) {
    return new Uint8Array(this.memory.buffer, address, length).slice();
  }

  allocate_static(size, align) {
    return Number(this.exports.nassau_static_alloc(BigInt(size), BigInt(align)));
  }

  builtin(index) {
    return this.exports.nassau_exception(BigInt(index) * 2n + 1n);
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
    this.exports.nassau_take_uncaught();
    this.wasi.exit = undefined;
    try {
      return this.env[entry]();
    } catch (error) {
      if (error !== this.wasi.exitSignal) throw error;
      return 0;
    } finally {
      this.exports.nassau_reset_frames();
      this.stack_pointer.value = STACK_TOP;
    }
  }

  take_uncaught() {
    return this.exports.nassau_take_uncaught();
  }

  exit_status() {
    return this.wasi.exit ?? -1;
  }

  retain_globals(addresses) {
    this.exports.nassau_clear_globals();
    for (const address of addresses) this.exports.nassau_global_root(BigInt(address));
  }

  take_output() {
    return this.wasi.takeOutput();
  }

  push_output(bytes) {
    this.wasi.output.push(bytes.slice());
  }

  push_host(value) { this.exports.nassau_host_root_push(value); }
  pop_host() { this.exports.nassau_host_root_pop(); }
}
