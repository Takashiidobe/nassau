const SUCCESS = 0;
const EBADF = 8;
const EEXIST = 20;
const EINVAL = 28;
const EISDIR = 31;
const ENOENT = 44;
const ENOTDIR = 54;
const ENOSYS = 52;
const ENOTEMPTY = 55;
const ESPIPE = 70;

const FILETYPE_CHARACTER_DEVICE = 2;
const FILETYPE_DIRECTORY = 3;
const FILETYPE_REGULAR_FILE = 4;

const OFLAGS_CREAT = 1;
const OFLAGS_DIRECTORY = 2;
const OFLAGS_EXCL = 4;
const OFLAGS_TRUNC = 8;
const FDFLAGS_APPEND = 1;

const ROOT_FD = 3;

const nowNanoseconds = () => BigInt(Math.round(Date.now() * 1e6));
const FSTFLAGS_MTIM = 4;
const FSTFLAGS_MTIM_NOW = 8;

class Directory {
  constructor(parent) {
    this.entries = new Map();
    this.parent = parent ?? this;
    this.mtime = nowNanoseconds();
  }
}

class File {
  constructor(data = new Uint8Array(0)) {
    this.data = data;
    this.size = data.length;
    this.mtime = nowNanoseconds();
  }

  write(offset, bytes) {
    const end = offset + bytes.length;
    if (end > this.data.length) {
      const grown = new Uint8Array(Math.max(end, this.data.length * 2, 64));
      grown.set(this.data.subarray(0, this.size));
      this.data = grown;
    }
    if (offset > this.size) this.data.fill(0, this.size, offset);
    this.data.set(bytes, offset);
    this.size = Math.max(this.size, end);
    this.mtime = nowNanoseconds();
  }

  truncate(size) {
    if (size > this.size) this.write(this.size, new Uint8Array(size - this.size));
    this.size = size;
    this.mtime = nowNanoseconds();
  }
}

export class Wasi {
  constructor(env = {}, files = {}) {
    this.environment = Object.entries(env).map(([name, value]) =>
      new TextEncoder().encode(`${name}=${value}\0`));
    this.output = [];
    this.stdin = () => new Uint8Array(0);
    this.exit = undefined;
    this.exitSignal = {};
    this.root = new Directory();
    this.descriptors = new Map([[ROOT_FD, { node: this.root }]]);
    this.nextDescriptor = ROOT_FD + 1;
    for (const [path, contents] of Object.entries(files)) {
      this.writeFile(path, typeof contents === 'string' ? new TextEncoder().encode(contents) : contents);
    }
  }

  sleep(milliseconds) {
    if (typeof SharedArrayBuffer === 'function') {
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
      return;
    }
    const end = performance.now() + milliseconds;
    while (performance.now() < end);
  }

  writeFile(path, bytes) {
    const parts = path.split('/').filter(Boolean);
    const name = parts.pop();
    let directory = this.root;
    for (const part of parts) {
      if (!directory.entries.has(part)) directory.entries.set(part, new Directory(directory));
      directory = directory.entries.get(part);
    }
    directory.entries.set(name, new File(bytes.slice()));
  }

  readFile(path) {
    const node = this.resolve(this.root, path);
    return node instanceof File ? node.data.slice(0, node.size) : undefined;
  }

  resolve(start, path) {
    let node = start;
    for (const part of path.split('/')) {
      if (part === '' || part === '.') continue;
      if (!(node instanceof Directory)) return undefined;
      if (part === '..') { node = node.parent; continue; }
      node = node.entries.get(part);
      if (!node) return undefined;
    }
    return node;
  }

  parent(start, path) {
    const parts = path.split('/').filter(part => part !== '' && part !== '.');
    const name = parts.pop();
    const directory = this.resolve(start, parts.join('/'));
    return { directory, name };
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
    const bytes = () => new Uint8Array(memory().buffer);
    const text = (address, length) => new TextDecoder().decode(bytes().subarray(address, address + length));
    const entry = fd => this.descriptors.get(fd);
    const filestat = (address, node) => {
      const data = view();
      for (let offset = 0; offset < 64; offset += 8) data.setBigUint64(address + offset, 0n, true);
      data.setUint8(address + 16, node instanceof Directory ? FILETYPE_DIRECTORY : FILETYPE_REGULAR_FILE);
      data.setBigUint64(address + 24, 1n, true);
      data.setBigUint64(address + 32, BigInt(node instanceof File ? node.size : 0), true);
      for (const offset of [40, 48, 56]) data.setBigUint64(address + offset, node.mtime, true);
    };
    const gather = (iovs, count) => {
      const data = view();
      const parts = [];
      for (let i = 0; i < count; i++) {
        const address = data.getUint32(iovs + i * 8, true);
        const length = data.getUint32(iovs + i * 8 + 4, true);
        parts.push(bytes().subarray(address, address + length));
      }
      return parts;
    };
    return {
      proc_exit: code => {
        this.exit = code & 255;
        throw this.exitSignal;
      },
      fd_write: (fd, iovs, count, written) => {
        let total = 0;
        if (fd === 1 || fd === 2) {
          for (const part of gather(iovs, count)) {
            this.output.push(part.slice());
            total += part.length;
          }
        } else {
          const open = entry(fd);
          if (!open || !(open.node instanceof File)) return open ? EISDIR : EBADF;
          for (const part of gather(iovs, count)) {
            if (open.append) open.offset = open.node.size;
            open.node.write(open.offset, part);
            open.offset += part.length;
            total += part.length;
          }
        }
        view().setUint32(written, total, true);
        return SUCCESS;
      },
      fd_read: (fd, iovs, count, read) => {
        let total = 0;
        if (fd === 0) {
          const parts = gather(iovs, count);
          const wanted = parts.reduce((length, part) => length + part.length, 0);
          let chunk = wanted ? this.stdin(wanted).subarray(0, wanted) : new Uint8Array(0);
          for (const part of parts) {
            const length = Math.min(part.length, chunk.length);
            part.set(chunk.subarray(0, length));
            chunk = chunk.subarray(length);
            total += length;
          }
        } else {
          const open = entry(fd);
          if (!open || !(open.node instanceof File)) return open ? EISDIR : EBADF;
          for (const part of gather(iovs, count)) {
            const available = Math.max(0, open.node.size - open.offset);
            const length = Math.min(part.length, available);
            part.set(open.node.data.subarray(open.offset, open.offset + length));
            open.offset += length;
            total += length;
          }
        }
        view().setUint32(read, total, true);
        return SUCCESS;
      },
      fd_seek: (fd, offset, whence, result) => {
        const open = entry(fd);
        if (!open) return fd <= 2 ? ESPIPE : EBADF;
        if (!(open.node instanceof File)) return EISDIR;
        const base = [open.offset, open.node.size, open.node.size][Number(whence)] ?? 0;
        const target = whence === 0 ? Number(offset) : base + Number(offset);
        if (target < 0) return EINVAL;
        open.offset = target;
        view().setBigUint64(result, BigInt(target), true);
        return SUCCESS;
      },
      fd_tell: (fd, result) => {
        const open = entry(fd);
        if (!open) return fd <= 2 ? ESPIPE : EBADF;
        view().setBigUint64(result, BigInt(open.offset ?? 0), true);
        return SUCCESS;
      },
      fd_close: fd => {
        if (fd <= 2) return SUCCESS;
        return this.descriptors.delete(fd) ? SUCCESS : EBADF;
      },
      fd_sync: () => SUCCESS,
      fd_datasync: () => SUCCESS,
      fd_advise: () => SUCCESS,
      fd_allocate: () => SUCCESS,
      fd_fdstat_get: (fd, buffer) => {
        const open = entry(fd);
        if (fd > 2 && !open) return EBADF;
        const data = view();
        const type = fd <= 2
          ? FILETYPE_CHARACTER_DEVICE
          : open.node instanceof Directory ? FILETYPE_DIRECTORY : FILETYPE_REGULAR_FILE;
        data.setUint8(buffer, type);
        data.setUint16(buffer + 2, open?.append ? FDFLAGS_APPEND : 0, true);
        data.setBigUint64(buffer + 8, 0x1fffffffn, true);
        data.setBigUint64(buffer + 16, 0x1fffffffn, true);
        return SUCCESS;
      },
      fd_fdstat_set_flags: () => SUCCESS,
      fd_filestat_get: (fd, buffer) => {
        const open = entry(fd);
        if (!open) return fd <= 2 ? SUCCESS : EBADF;
        filestat(buffer, open.node);
        return SUCCESS;
      },
      fd_filestat_set_size: (fd, size) => {
        const open = entry(fd);
        if (!open || !(open.node instanceof File)) return EBADF;
        open.node.truncate(Number(size));
        return SUCCESS;
      },
      fd_filestat_set_times: (fd, atime, mtime, flags) => {
        const open = entry(fd);
        if (!open) return EBADF;
        if (flags & FSTFLAGS_MTIM_NOW) open.node.mtime = nowNanoseconds();
        else if (flags & FSTFLAGS_MTIM) open.node.mtime = mtime;
        return SUCCESS;
      },
      fd_prestat_get: (fd, buffer) => {
        if (fd !== ROOT_FD) return EBADF;
        const data = view();
        data.setUint8(buffer, 0);
        data.setUint32(buffer + 4, 1, true);
        return SUCCESS;
      },
      fd_prestat_dir_name: (fd, path, length) => {
        if (fd !== ROOT_FD) return EBADF;
        bytes().set(new TextEncoder().encode('/').subarray(0, length), path);
        return SUCCESS;
      },
      fd_readdir: (fd, buffer, length, cookie, used) => {
        const open = entry(fd);
        if (!open) return EBADF;
        if (!(open.node instanceof Directory)) return ENOTDIR;
        const names = ['.', '..', ...open.node.entries.keys()];
        const out = new Uint8Array(length);
        let written = 0;
        for (let index = Number(cookie); index < names.length && written < length; index++) {
          const name = new TextEncoder().encode(names[index]);
          const record = new Uint8Array(24 + name.length);
          const fields = new DataView(record.buffer);
          fields.setBigUint64(0, BigInt(index + 1), true);
          fields.setUint32(16, name.length, true);
          const child = open.node.entries.get(names[index]);
          fields.setUint8(20, child instanceof File ? FILETYPE_REGULAR_FILE : FILETYPE_DIRECTORY);
          record.set(name, 24);
          const part = record.subarray(0, Math.min(record.length, length - written));
          out.set(part, written);
          written += part.length;
        }
        bytes().set(out.subarray(0, written), buffer);
        view().setUint32(used, written, true);
        return SUCCESS;
      },
      path_open: (dirfd, dirflags, pathAddress, pathLength, oflags, rightsBase, rightsInheriting, fdflags, opened) => {
        const directory = entry(dirfd);
        if (!directory || !(directory.node instanceof Directory)) return directory ? ENOTDIR : EBADF;
        const path = text(pathAddress, pathLength);
        const existing = this.resolve(directory.node, path);
        let node = existing;
        if (existing) {
          if (oflags & OFLAGS_EXCL && oflags & OFLAGS_CREAT) return EEXIST;
          if (oflags & OFLAGS_DIRECTORY && !(existing instanceof Directory)) return ENOTDIR;
          if (existing instanceof File && oflags & OFLAGS_TRUNC) existing.truncate(0);
        } else {
          if (!(oflags & OFLAGS_CREAT)) return ENOENT;
          const { directory: parent, name } = this.parent(directory.node, path);
          if (!(parent instanceof Directory) || !name) return ENOENT;
          node = new File();
          parent.entries.set(name, node);
        }
        const fd = this.nextDescriptor++;
        this.descriptors.set(fd, { node, offset: 0, append: Boolean(fdflags & FDFLAGS_APPEND) });
        view().setUint32(opened, fd, true);
        return SUCCESS;
      },
      path_filestat_get: (dirfd, flags, pathAddress, pathLength, buffer) => {
        const directory = entry(dirfd);
        if (!directory) return EBADF;
        const node = this.resolve(directory.node, text(pathAddress, pathLength));
        if (!node) return ENOENT;
        filestat(buffer, node);
        return SUCCESS;
      },
      path_unlink_file: (dirfd, pathAddress, pathLength) => {
        const directory = entry(dirfd);
        if (!directory) return EBADF;
        const { directory: parent, name } = this.parent(directory.node, text(pathAddress, pathLength));
        const node = parent?.entries?.get(name);
        if (!node) return ENOENT;
        if (node instanceof Directory) return EISDIR;
        parent.entries.delete(name);
        return SUCCESS;
      },
      path_create_directory: (dirfd, pathAddress, pathLength) => {
        const directory = entry(dirfd);
        if (!directory) return EBADF;
        const { directory: parent, name } = this.parent(directory.node, text(pathAddress, pathLength));
        if (!(parent instanceof Directory) || !name) return ENOENT;
        if (parent.entries.has(name)) return EEXIST;
        parent.entries.set(name, new Directory(parent));
        return SUCCESS;
      },
      path_remove_directory: (dirfd, pathAddress, pathLength) => {
        const directory = entry(dirfd);
        if (!directory) return EBADF;
        const { directory: parent, name } = this.parent(directory.node, text(pathAddress, pathLength));
        const node = parent?.entries?.get(name);
        if (!node) return ENOENT;
        if (!(node instanceof Directory)) return ENOTDIR;
        if (node.entries.size) return ENOTEMPTY;
        parent.entries.delete(name);
        return SUCCESS;
      },
      path_readlink: (dirfd, pathAddress, pathLength) => {
        const directory = entry(dirfd);
        if (!directory) return EBADF;
        return this.resolve(directory.node, text(pathAddress, pathLength)) ? EINVAL : ENOENT;
      },
      path_rename: (oldfd, oldAddress, oldLength, newfd, newAddress, newLength) => {
        const from = entry(oldfd);
        const to = entry(newfd);
        if (!from || !to) return EBADF;
        const source = this.parent(from.node, text(oldAddress, oldLength));
        const target = this.parent(to.node, text(newAddress, newLength));
        const node = source.directory?.entries?.get(source.name);
        if (!node) return ENOENT;
        if (!(target.directory instanceof Directory) || !target.name) return ENOENT;
        const existing = target.directory.entries.get(target.name);
        if (existing) {
          if (existing === node) return SUCCESS;
          if (existing instanceof Directory) {
            if (!(node instanceof Directory)) return EISDIR;
            if (existing.entries.size) return ENOTEMPTY;
          } else if (node instanceof Directory) {
            return ENOTDIR;
          }
        }
        source.directory.entries.delete(source.name);
        target.directory.entries.set(target.name, node);
        if (node instanceof Directory) node.parent = target.directory;
        return SUCCESS;
      },
      environ_sizes_get: (count, size) => {
        const data = view();
        data.setUint32(count, this.environment.length, true);
        data.setUint32(size, this.environment.reduce((total, item) => total + item.length, 0), true);
        return SUCCESS;
      },
      environ_get: (pointers, buffer) => {
        const data = view();
        this.environment.forEach((item, index) => {
          data.setUint32(pointers + index * 4, buffer, true);
          bytes().set(item, buffer);
          buffer += item.length;
        });
        return SUCCESS;
      },
      clock_time_get: (id, precision, out) => {
        const milliseconds = id === 0 ? Date.now() : performance.now();
        view().setBigUint64(out, BigInt(Math.round(milliseconds * 1e6)), true);
        return SUCCESS;
      },
      poll_oneoff: (subscriptions, events, count, produced) => {
        const data = view();
        let wait = Infinity;
        for (let index = 0; index < count; index++) {
          const base = subscriptions + index * 48;
          if (data.getUint8(base + 8) !== 0) return ENOSYS;
          const timeout = Number(data.getBigUint64(base + 24, true));
          const absolute = data.getUint16(base + 40, true) & 1;
          const now = (data.getUint32(base + 16, true) === 0 ? Date.now() : performance.now()) * 1e6;
          wait = Math.min(wait, absolute ? timeout - now : timeout);
        }
        this.sleep(Math.max(0, wait) / 1e6);
        for (let index = 0; index < count; index++) {
          const event = events + index * 32;
          for (let offset = 0; offset < 32; offset += 8) data.setBigUint64(event + offset, 0n, true);
          data.setBigUint64(event, data.getBigUint64(subscriptions + index * 48, true), true);
        }
        data.setUint32(produced, count, true);
        return SUCCESS;
      },
      random_get: (address, length) => {
        crypto.getRandomValues(new Uint8Array(memory().buffer, address, length));
        return SUCCESS;
      },
    };
  }
}
