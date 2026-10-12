const EBADF = 8;
const ESPIPE = 70;

export class Wasi {
  constructor(env = {}) {
    this.environment = Object.entries(env).map(([name, value]) =>
      new TextEncoder().encode(`${name}=${value}\0`));
    this.output = [];
    this.exit = undefined;
    this.exitSignal = {};
  }

  takeOutput() {
    const bytes = new Uint8Array(this.output.reduce((length, part) => length + part.length, 0));
    let offset = 0;
    for (const part of this.output) { bytes.set(part, offset); offset += part.length; }
    this.output = [];
    return bytes;
  }

  imports(memory) {
    const view = () => new DataView(memory().buffer);
    return {
      proc_exit: code => {
        this.exit = code & 255;
        throw this.exitSignal;
      },
      fd_write: (fd, iovs, count, written) => {
        if (fd !== 1 && fd !== 2) return EBADF;
        const data = view();
        let total = 0;
        for (let i = 0; i < count; i++) {
          const address = data.getUint32(iovs + i * 8, true);
          const length = data.getUint32(iovs + i * 8 + 4, true);
          this.output.push(new Uint8Array(memory().buffer, address, length).slice());
          total += length;
        }
        data.setUint32(written, total, true);
        return 0;
      },
      fd_fdstat_get: (fd, buffer) => {
        if (fd > 2) return EBADF;
        const data = view();
        data.setUint8(buffer, 2);
        data.setUint16(buffer + 2, 0, true);
        data.setBigUint64(buffer + 8, 0n, true);
        data.setBigUint64(buffer + 16, 0n, true);
        return 0;
      },
      fd_seek: () => ESPIPE,
      fd_close: () => 0,
      fd_prestat_get: () => EBADF,
      fd_prestat_dir_name: () => EBADF,
      environ_sizes_get: (count, size) => {
        const data = view();
        data.setUint32(count, this.environment.length, true);
        data.setUint32(size, this.environment.reduce((total, entry) => total + entry.length, 0), true);
        return 0;
      },
      environ_get: (pointers, buffer) => {
        const data = view();
        this.environment.forEach((entry, index) => {
          data.setUint32(pointers + index * 4, buffer, true);
          new Uint8Array(memory().buffer, buffer, entry.length).set(entry);
          buffer += entry.length;
        });
        return 0;
      },
      clock_time_get: (id, precision, out) => {
        const milliseconds = id === 0 ? Date.now() : performance.now();
        view().setBigUint64(out, BigInt(Math.round(milliseconds * 1e6)), true);
        return 0;
      },
      random_get: (address, length) => {
        crypto.getRandomValues(new Uint8Array(memory().buffer, address, length));
        return 0;
      },
    };
  }
}
