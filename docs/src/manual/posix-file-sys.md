# <span id="section:0"></span>The `Posix.FileSys` structure

---

#### Synopsis

<span id="POSIX_FILE_SYS:SIG:SPEC"></span>
<span id="FileSys:STR:SPEC"></span>

```sml
signature POSIX_FILE_SYS
structure FileSys : POSIX_FILE_SYS
```

The structure `Posix.FileSys` provides access to file system operations as described in Section 5 of the POSIX standard 1003.1,1996**\[CITE\]**.

---

#### Interface

<span id="SIG:POSIX_FILE_SYS.uid:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.gid:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.file_desc:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fdToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.wordToFD:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fdToIOD:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.iodToFD:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.dirstream:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.opendir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.readdir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.rewinddir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.closedir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.chdir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.getcwd:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.stdin:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.stdout:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.stderr:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.mode:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.irwxu:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.irusr:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.iwusr:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ixusr:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.irwxg:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.irgrp:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.iwgrp:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ixgrp:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.irwxo:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.iroth:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.iwoth:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ixoth:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isuid:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isgid:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.append:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.excl:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.noctty:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.nonblock:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.sync:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.trunc:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.open_mode:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.O_RDONLY:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.O_WRONLY:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.O_RDWR:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.openf:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.createf:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.creat:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.umask:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.link:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.mkdir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.mkfifo:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.unlink:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.rmdir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.rename:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.symlink:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.readlink:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.dev:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.wordToDev:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.devToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ino:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.wordToIno:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.inoToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.stat:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isDir:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isChr:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isBlk:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isReg:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isFIFO:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isLink:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.isSock:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.mode:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ino:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.dev:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.nlink:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.uid:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.gid:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.size:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.atime:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.mtime:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ctime:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.stat:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.lstat:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fstat:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.access_mode:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.A_READ:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.A_WRITE:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.A_EXEC:TY:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.access:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.chmod:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fchmod:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.chown:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fchown:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.utime:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.ftruncate:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.pathconf:VAL:SPEC"></span>
<span id="SIG:POSIX_FILE_SYS.fpathconf:VAL:SPEC"></span>

```sml
eqtype uid
eqtype gid
eqtype file_desc
val fdToWord : file_desc -> SysWord.word
val wordToFD : SysWord.word -> file_desc
val fdToIOD : file_desc -> OS.IO.iodesc
val iodToFD : OS.IO.iodesc -> file_desc option
type dirstream
val opendir : string -> dirstream
val readdir : dirstream -> string option
val rewinddir : dirstream -> unit
val closedir : dirstream -> unit
val chdir : string -> unit
val getcwd : unit -> string
val stdin : file_desc
val stdout : file_desc
val stderr : file_desc
structure S : sig
eqtype mode
include BIT_FLAGS
where type flags = mode
val irwxu : mode
val irusr : mode
val iwusr : mode
val ixusr : mode
val irwxg : mode
val irgrp : mode
val iwgrp : mode
val ixgrp : mode
val irwxo : mode
val iroth : mode
val iwoth : mode
val ixoth : mode
val isuid : mode
val isgid : mode
end
structure O : sig
include BIT_FLAGS
val append : flags
val excl : flags
val noctty : flags
val nonblock : flags
val sync : flags
val trunc : flags
end
datatype open_mode
= O_RDONLY
| O_WRONLY
| O_RDWR
val openf : string * open_mode * O.flags -> file_desc
val createf : string * open_mode * O.flags * S.mode -> file_desc
val creat : string * S.mode -> file_desc
val umask : S.mode -> S.mode
val link : {old : string, new : string} -> unit
val mkdir : string * S.mode -> unit
val mkfifo : string * S.mode -> unit
val unlink : string -> unit
val rmdir : string -> unit
val rename : {old : string, new : string} -> unit
val symlink : {old : string, new : string} -> unit
val readlink : string -> string
eqtype dev
val wordToDev : SysWord.word -> dev
val devToWord : dev -> SysWord.word
eqtype ino
val wordToIno : SysWord.word -> ino
val inoToWord : ino -> SysWord.word
structure ST : sig
type stat
val isDir : stat -> bool
val isChr : stat -> bool
val isBlk : stat -> bool
val isReg : stat -> bool
val isFIFO : stat -> bool
val isLink : stat -> bool
val isSock : stat -> bool
val mode : stat -> S.mode
val ino : stat -> ino
val dev : stat -> dev
val nlink : stat -> int
val uid : stat -> uid
val gid : stat -> gid
val size : stat -> Position.int
val atime : stat -> Time.time
val mtime : stat -> Time.time
val ctime : stat -> Time.time
end
val stat : string -> ST.stat
val lstat : string -> ST.stat
val fstat : file_desc -> ST.stat
datatype access_mode = A_READ | A_WRITE | A_EXEC
val access : string * access_mode list -> bool
val chmod : string * S.mode -> unit
val fchmod : file_desc * S.mode -> unit
val chown : string * uid * gid -> unit
val fchown : file_desc * uid * gid -> unit
val utime : string
* {actime : Time.time, modtime : Time.time} option -> unit
val ftruncate : file_desc * Position.int -> unit
val pathconf : string * string -> SysWord.word option
val fpathconf : file_desc * string -> SysWord.word option
```

#### Description

<span id="SIG:POSIX_FILE_SYS.uid:TY"></span>**`eqtype`**` uid`  
User identifier; identical to [`Posix.ProcEnv.uid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.uid:TY:SPEC).

<span id="SIG:POSIX_FILE_SYS.gid:TY"></span>**`eqtype`**` gid`  
Group identifier; identical to [`Posix.ProcEnv.gid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.gid:TY:SPEC).

<span id="SIG:POSIX_FILE_SYS.file_desc:TY"></span>**`eqtype`**` file_desc`  
Open file descriptor.

<span id="SIG:POSIX_FILE_SYS.fdToWord:VAL"></span>

### `fdToWord`

```sml
val fdToWord : file_desc -> SysWord.word
```

```repl
Posix.FileSys.fdToWord Posix.FileSys.stdin;; (* descriptor word *)
```
<span id="SIG:POSIX_FILE_SYS.wordToFD:VAL"></span>

### `wordToFD`

```sml
val wordToFD : SysWord.word -> file_desc
```

These functions convert between an abstract open file descriptor and the integer representation used by the operating system. These calls should be avoided where possible, for the SML implementation may be able to garbage collect (_i.e._, automatically close) any [`file_desc`](posix-file-sys.md#SIG:POSIX_FILE_SYS.file_desc:TY:SPEC) value that is not accessible, but it cannot do this for any [`file_desc`](posix-file-sys.md#SIG:POSIX_FILE_SYS.file_desc:TY:SPEC) that has ever been made concrete by [`fdToWord`](posix-file-sys.md#SIG:POSIX_FILE_SYS.fdToWord:VAL:SPEC). Also, there is no validation that the file descriptor created by `wordToFD` corresponds to an actually open file.

```repl
Posix.FileSys.wordToFD (Posix.FileSys.fdToWord Posix.FileSys.stdin);; (* standard input descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.fdToIOD:VAL"></span>

### `fdToIOD`

```sml
val fdToIOD : file_desc -> OS.IO.iodesc
```

```repl
Posix.FileSys.fdToIOD Posix.FileSys.stdin;; (* OS I/O descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.iodToFD:VAL"></span>

### `iodToFD`

```sml
val iodToFD : OS.IO.iodesc -> file_desc option
```

These convert between a POSIX open file descriptor and the handle used by the OS subsystem. The function [`iodToFD`](posix-file-sys.md#SIG:POSIX_FILE_SYS.iodToFD:VAL:SPEC) returns an [`option`](option.md#SIG:OPTION.option:TY:SPEC) type because, on certain systems, some open I/O devices are not associated with an underlying open file descriptor.

<span id="SIG:POSIX_FILE_SYS.dirstream:TY"></span>**`type`**` dirstream`  
A directory stream opened for reading. A directory stream is an ordered sequence of all the directory entries in a particular directory. This type is identical to [`OS.FileSys.dirstream`](os-file-sys.md#SIG:OS_FILE_SYS.dirstream:TY:SPEC).

```repl
Posix.FileSys.iodToFD (Posix.FileSys.fdToIOD Posix.FileSys.stdin);; (* SOME descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.opendir:VAL"></span>

### `opendir`

```sml
val opendir : string -> dirstream
```
`opendir ``dirName`` `  
opens the directory designated by the `dirName` parameter and associates a directory stream with it. The directory stream is positioned at the first entry.

```repl
val d = Posix.FileSys.opendir "."; Posix.FileSys.closedir d;; (* directory stream opened and closed *)
```
<span id="SIG:POSIX_FILE_SYS.readdir:VAL"></span>

### `readdir`

```sml
val readdir : dirstream -> string option
```
`readdir ``dir`` `  
returns and removes one filename from the directory stream `dir`. When the directory stream is empty (that is, when all entries have been read from the stream), [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. Entries for `"."` (current directory) and `".."` (parent directory) are never returned.

> **Rationale:**
>
> The reason for filtering out the current and parent directory entries is that it makes recursive walks of a directory tree easier.

```repl
val d = Posix.FileSys.opendir "."; val entry = Posix.FileSys.readdir d; Posix.FileSys.closedir d; entry;; (* first entry or NONE *)
```
<span id="SIG:POSIX_FILE_SYS.rewinddir:VAL"></span>

### `rewinddir`

```sml
val rewinddir : dirstream -> unit
```
`rewinddir ``d`` `  
repositions the directory stream `d` for reading at the beginning.

```repl
val d = Posix.FileSys.opendir "."; Posix.FileSys.rewinddir d; Posix.FileSys.closedir d;; (* () *)
```
<span id="SIG:POSIX_FILE_SYS.closedir:VAL"></span>

### `closedir`

```sml
val closedir : dirstream -> unit
```
`closedir ``d`` `  
closes the directory stream `d`. Closing a previously closed [`dirstream`](posix-file-sys.md#SIG:POSIX_FILE_SYS.dirstream:TY:SPEC) does not raise an exception.

```repl
Posix.FileSys.closedir (Posix.FileSys.opendir ".");; (* () *)
```
<span id="SIG:POSIX_FILE_SYS.chdir:VAL"></span>

### `chdir`

```sml
val chdir : string -> unit
```
`chdir ``s`` `  
changes the current working directory to `s`.

```repl
Posix.FileSys.chdir ".";; (* () *)
```
<span id="SIG:POSIX_FILE_SYS.getcwd:VAL"></span>

### `getcwd`

```sml
val getcwd : unit -> string
```
The absolute pathname of the current working directory.

```repl
Posix.FileSys.getcwd ();; (* current directory *)
```
<span id="SIG:POSIX_FILE_SYS.stdin:VAL"></span>

### `stdin`

```sml
val stdin : file_desc
```

```repl
Posix.FileSys.stdin;; (* standard input descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.stdout:VAL"></span>

### `stdout`

```sml
val stdout : file_desc
```

```repl
Posix.FileSys.stdout;; (* standard output descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.stderr:VAL"></span>

### `stderr`

```sml
val stderr : file_desc
```

The standard input, output, and error file descriptors.

<span id="SIG:POSIX_FILE_SYS.S:STR"></span>
**`structure`**` S`  

<span id="SIG:POSIX_FILE_SYS.S.mode:TY"></span>**`eqtype`**` mode`  
A file [`mode`](posix-file-sys.md#SIG:POSIX_FILE_SYS.S.mode:TY:SPEC) is a set of (read, write, execute) permissions for the owner of the file, members of the file's group, and others.

```repl
Posix.FileSys.stderr;; (* standard error descriptor *)
```
<span id="SIG:POSIX_FILE_SYS.S.irwxu:VAL"></span>

### `irwxu`

```sml
val irwxu : mode
```
Read, write, and execute permission for \`\`user'' (the file's owner).

```repl
Posix.FileSys.S.irwxu;; (* owner read, write, and execute bits *)
```
<span id="SIG:POSIX_FILE_SYS.S.irusr:VAL"></span>

### `irusr`

```sml
val irusr : mode
```
Read permission for \`\`user'' (the file's owner).

```repl
Posix.FileSys.S.irusr;; (* owner read bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.iwusr:VAL"></span>

### `iwusr`

```sml
val iwusr : mode
```
Write permission for \`\`user'' (the file's owner).

```repl
Posix.FileSys.S.iwusr;; (* owner write bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.ixusr:VAL"></span>

### `ixusr`

```sml
val ixusr : mode
```
Execute permission for \`\`user'' (the file's owner).

```repl
Posix.FileSys.S.ixusr;; (* owner execute bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.irwxg:VAL"></span>

### `irwxg`

```sml
val irwxg : mode
```
Read, write, and execute permission for members of the file's group.

```repl
Posix.FileSys.S.irwxg;; (* group read, write, and execute bits *)
```
<span id="SIG:POSIX_FILE_SYS.S.irgrp:VAL"></span>

### `irgrp`

```sml
val irgrp : mode
```
Read permission for members of the file's group.

```repl
Posix.FileSys.S.irgrp;; (* group read bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.iwgrp:VAL"></span>

### `iwgrp`

```sml
val iwgrp : mode
```
Write permission for members of the file's group.

```repl
Posix.FileSys.S.iwgrp;; (* group write bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.ixgrp:VAL"></span>

### `ixgrp`

```sml
val ixgrp : mode
```
Execute permission for members of the file's group.

```repl
Posix.FileSys.S.ixgrp;; (* group execute bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.irwxo:VAL"></span>

### `irwxo`

```sml
val irwxo : mode
```
Read, write, and execute permission for \`\`others'' (all users).

```repl
Posix.FileSys.S.irwxo;; (* other read, write, and execute bits *)
```
<span id="SIG:POSIX_FILE_SYS.S.iroth:VAL"></span>

### `iroth`

```sml
val iroth : mode
```
Read permission for \`\`others'' (all users).

```repl
Posix.FileSys.S.iroth;; (* other read bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.iwoth:VAL"></span>

### `iwoth`

```sml
val iwoth : mode
```
Write permission for \`\`others'' (all users).

```repl
Posix.FileSys.S.iwoth;; (* other write bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.ixoth:VAL"></span>

### `ixoth`

```sml
val ixoth : mode
```
Execute permission for \`\`others'' (all users).

```repl
Posix.FileSys.S.ixoth;; (* other execute bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.isuid:VAL"></span>

### `isuid`

```sml
val isuid : mode
```
Set-user-id mode, indicating that the effective user ID of any user executing the file should be made the same as that of the owner of the file.

```repl
Posix.FileSys.S.isuid;; (* set-user-ID bit *)
```
<span id="SIG:POSIX_FILE_SYS.S.isgid:VAL"></span>

### `isgid`

```sml
val isgid : mode
end
```
Set-group-id mode, indicating that the effective group ID of any user executing the file should be made the same as the group of the file.

<span id="SIG:POSIX_FILE_SYS.O:STR"></span>
**`structure`**` O`  
The structure [`Posix.FileSys.O`](posix-file-sys.md#SIG:POSIX_FILE_SYS.O:STR:SPEC) contains file status flags used in calls to [`openf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.openf:VAL:SPEC).

```repl
Posix.FileSys.S.isgid;; (* set-group-ID bit *)
```
<span id="SIG:POSIX_FILE_SYS.O.append:VAL"></span>

### `append`

```sml
val append : flags
```
If set, the file pointer is set to the end of the file prior to each write.

```repl
Posix.FileSys.O.append;; (* append flag *)
```
<span id="SIG:POSIX_FILE_SYS.O.excl:VAL"></span>

### `excl`

```sml
val excl : flags
```
This flag causes the open to fail if the file already exists.

```repl
Posix.FileSys.O.excl;; (* exclusive-create flag *)
```
<span id="SIG:POSIX_FILE_SYS.O.noctty:VAL"></span>

### `noctty`

```sml
val noctty : flags
```
If the path parameter identifies a terminal device, this flag assures that the terminal device does not become the controlling terminal for the process.

```repl
Posix.FileSys.O.noctty;; (* do not acquire controlling terminal *)
```
<span id="SIG:POSIX_FILE_SYS.O.nonblock:VAL"></span>

### `nonblock`

```sml
val nonblock : flags
```
Open, read, and write operations on the file will be nonblocking.

```repl
Posix.FileSys.O.nonblock;; (* nonblocking flag *)
```
<span id="SIG:POSIX_FILE_SYS.O.sync:VAL"></span>

### `sync`

```sml
val sync : flags
```
If set, updates and writes to regular files and block devices are synchronous updates. On return from a function that performs a synchronous update ([`writeVec`](posix-io.md#SIG:POSIX_IO.writeVec:VAL:SPEC), [`writeArr`](posix-io.md#SIG:POSIX_IO.writeArr:VAL:SPEC), [`ftruncate`](posix-file-sys.md#SIG:POSIX_FILE_SYS.ftruncate:VAL:SPEC), [`openf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.openf:VAL:SPEC) with [`trunc`](posix-file-sys.md#SIG:POSIX_FILE_SYS.O.trunc:VAL:SPEC)), the calling process is assured that all data for the file has been written to permanent storage, even if the file is also open for deferred update.

```repl
Posix.FileSys.O.sync;; (* synchronized I/O flag *)
```
<span id="SIG:POSIX_FILE_SYS.O.trunc:VAL"></span>

### `trunc`

```sml
val trunc : flags
end
```
This causes the file to be truncated (to zero length) upon opening.

<span id="SIG:POSIX_FILE_SYS.open_mode:TY"></span>
**`datatype`**` open_mode`  
Operations allowed on an open file.

`= O_RDONLY`  
Open a file for reading only.

`| O_WRONLY`  
Open a file for writing only.

`| O_RDWR`  
Open a file for reading and writing.

```repl
Posix.FileSys.O.trunc;; (* truncate file when opening *)
```
<span id="SIG:POSIX_FILE_SYS.openf:VAL"></span>

### `openf`

```sml
val openf : string * open_mode * O.flags -> file_desc
```
`openf (``s``, ``om``, ``f``) `

```repl
let val fd = Posix.FileSys.openf ("/etc/hosts", Posix.FileSys.O_RDONLY, Posix.FileSys.O.flags []) in Posix.IO.close fd end;; (* () *)
```
<span id="SIG:POSIX_FILE_SYS.createf:VAL"></span>

### `createf`

```sml
val createf : string * open_mode * O.flags * S.mode -> file_desc
```

`createf (``s``, ``om``, ``f``, ``m``)`  
These calls open a file named `s` for reading, writing, or both (depending on the open mode `om`). The flags `f` specify the state of the open file. If the file does not exist, [`openf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.openf:VAL:SPEC) raises the [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception whereas [`createf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.createf:VAL:SPEC) creates the file, setting its protection mode to `m` (as modified by the [`umask`](posix-file-sys.md#SIG:POSIX_FILE_SYS.umask:VAL:SPEC)).

Note that, in C, the roles of [`openf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.openf:VAL:SPEC) and [`createf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.createf:VAL:SPEC) are combined in the function `open`. The first acts like `open` without the `O_CREAT` flag; the second acts like `open` with the `O_CREAT` flag and the specified permission mode. Also, the [`createf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.createf:VAL:SPEC) function should not be confused with the [`creat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.creat:VAL:SPEC) function below, which behaves like its C namesake.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.createf (p, Posix.FileSys.O_WRONLY, Posix.FileSys.O.flags [], Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd in Posix.FileSys.unlink p end;; (* temporary file created and removed *)
```
<span id="SIG:POSIX_FILE_SYS.creat:VAL"></span>

### `creat`

```sml
val creat : string * S.mode -> file_desc
```
`creat (``s``, ``m``) `  
opens a file `s` for writing. If the file exists, this call truncates the file to zero length. If the file does not exist, it creates the file, setting its protection mode to `m` (as modified by the [`umask`](posix-file-sys.md#SIG:POSIX_FILE_SYS.umask:VAL:SPEC)). This is equivalent to the expression:

[createf](posix-file-sys.md#SIG:POSIX_FILE_SYS.createf:VAL:SPEC)(`s`,[O_WRONLY](posix-file-sys.md#SIG:POSIX_FILE_SYS.open_mode:TY:SPEC),[O.trunc](posix-file-sys.md#SIG:POSIX_FILE_SYS.O.trunc:VAL:SPEC),`m`)

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; Posix.FileSys.unlink p;; (* temporary file created and removed *)
```
<span id="SIG:POSIX_FILE_SYS.umask:VAL"></span>

### `umask`

```sml
val umask : S.mode -> S.mode
```
`umask ``cmask`` `  
sets the file mode creation mask of the process to `cmask` and returns the previous value of the mask.

Whenever a file is created (by [`openf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.openf:VAL:SPEC), [`creat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.creat:VAL:SPEC), [`mkdir`](posix-file-sys.md#SIG:POSIX_FILE_SYS.mkdir:VAL:SPEC), etc.), all file permission set in the file mode creation mask are removed from the mode of the created file. This clearing allows users to restrict the default access to their files.

The mask is inherited by child processes.

```repl
let val old = Posix.FileSys.umask (Posix.FileSys.S.flags []) in Posix.FileSys.umask old end;; (* previous mask restored *)
```
<span id="SIG:POSIX_FILE_SYS.link:VAL"></span>

### `link`

```sml
val link : {old : string, new : string} -> unit
```
`link {``old``, ``new``} `  
creates an additional hard link (directory entry) for an existing file. Both the old and the new link share equal access rights to the underlying object.

Both `old` and `new` must reside on the same file system. A hard link to a directory cannot be created.

Upon successful completion, [`link`](posix-file-sys.md#SIG:POSIX_FILE_SYS.link:VAL:SPEC) updates the file status change time of the `old` file, and updates the file status change and modification times of the directory containing the `new` entry. (See [`Posix.FileSys.ST`](posix-file-sys.md#SIG:POSIX_FILE_SYS.ST:STR:SPEC).)

```repl
let val a = OS.FileSys.tmpName (); val b = a ^ "-link"; val fd = Posix.FileSys.creat (a, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val _ = Posix.FileSys.link {old = a, new = b}; val _ = Posix.FileSys.unlink b in Posix.FileSys.unlink a end;; (* hard link created and removed *)
```
<span id="SIG:POSIX_FILE_SYS.mkdir:VAL"></span>

### `mkdir`

```sml
val mkdir : string * S.mode -> unit
```
`mkdir (``s``, ``m``) `  
creates a new directory named `s` with protection mode `m` (as modified by the [`umask`](posix-file-sys.md#SIG:POSIX_FILE_SYS.umask:VAL:SPEC)).

```repl
let val p = OS.FileSys.tmpName () in Posix.FileSys.mkdir (p, Posix.FileSys.S.irwxu); Posix.FileSys.rmdir p end;; (* temporary directory created and removed *)
```
<span id="SIG:POSIX_FILE_SYS.mkfifo:VAL"></span>

### `mkfifo`

```sml
val mkfifo : string * S.mode -> unit
```
`mkfifo (``s``, ``m``) `  
makes a FIFO special file (or named pipe) `s`, with protection mode `m` (as modified by the [`umask`](posix-file-sys.md#SIG:POSIX_FILE_SYS.umask:VAL:SPEC)).

```repl
let val p = OS.FileSys.tmpName () in Posix.FileSys.mkfifo (p, Posix.FileSys.S.irwxu); Posix.FileSys.unlink p end;; (* FIFO created and removed *)
```
<span id="SIG:POSIX_FILE_SYS.unlink:VAL"></span>

### `unlink`

```sml
val unlink : string -> unit
```
`unlink ``path`` `  
removes the directory entry specified by `path` and, if the entry is a hard link, decrements the link count of the file referenced by the link.

When all links to a file are removed and no process has the file open or mapped, all resources associated with the file are reclaimed, and the file is no longer accessible. If one or more processes have the file open or mapped when the last link is removed, the link is removed before [`unlink`](posix-file-sys.md#SIG:POSIX_FILE_SYS.unlink:VAL:SPEC) returns, but the removal of the file contents is postponed until all open or map references to the file are removed. If the `path` parameter names a symbolic link, the symbolic link itself is removed.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd in Posix.FileSys.unlink p end;; (* temporary file removed *)
```
<span id="SIG:POSIX_FILE_SYS.rmdir:VAL"></span>

### `rmdir`

```sml
val rmdir : string -> unit
```
`rmdir ``s`` `  
removes a directory `s`, which must be empty.

```repl
let val p = OS.FileSys.tmpName () in Posix.FileSys.mkdir (p, Posix.FileSys.S.irwxu); Posix.FileSys.rmdir p end;; (* () *)
```
<span id="SIG:POSIX_FILE_SYS.rename:VAL"></span>

### `rename`

```sml
val rename : {old : string, new : string} -> unit
```
`rename {``old``, ``new``} `  
changes the name of a file system object from `old` to `new`.

```repl
let val a = OS.FileSys.tmpName (); val b = a ^ "-renamed"; val fd = Posix.FileSys.creat (a, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val _ = Posix.FileSys.rename {old = a, new = b} in Posix.FileSys.unlink b end;; (* temporary file renamed and removed *)
```
<span id="SIG:POSIX_FILE_SYS.symlink:VAL"></span>

### `symlink`

```sml
val symlink : {old : string, new : string} -> unit
```
`symlink {``old``, ``new``} `  
creates a symbolic link `new`. Any component of a pathname resolving to `new` will be replaced by the text `old`. Note that `old` may be a relative or absolute pathname, and might not be the pathname of any existing file.

```repl
let val p = OS.FileSys.tmpName (); val _ = Posix.FileSys.symlink {old = "target", new = p}; val target = Posix.FileSys.readlink p; val _ = Posix.FileSys.unlink p in target end;; (* "target" *)
```
<span id="SIG:POSIX_FILE_SYS.readlink:VAL"></span>

### `readlink`

```sml
val readlink : string -> string
```
`readlink ``s`` `  
reads the value of a symbolic link `s`.

<span id="SIG:POSIX_FILE_SYS.dev:TY"></span>**`eqtype`**` dev`  
Device identifier. The device identifier and the file serial number (_inode_ or [`ino`](posix-file-sys.md#SIG:POSIX_FILE_SYS.ino:TY:SPEC)) uniquely identify a file.

```repl
let val p = OS.FileSys.tmpName (); val _ = Posix.FileSys.symlink {old = "target", new = p}; val target = Posix.FileSys.readlink p; val _ = Posix.FileSys.unlink p in target end;; (* "target" *)
```
<span id="SIG:POSIX_FILE_SYS.wordToDev:VAL"></span>

### `wordToDev`

```sml
val wordToDev : SysWord.word -> dev
```

```repl
Posix.FileSys.wordToDev (Posix.FileSys.devToWord (Posix.FileSys.ST.dev (Posix.FileSys.stat ".")));; (* device identifier *)
```
<span id="SIG:POSIX_FILE_SYS.devToWord:VAL"></span>

### `devToWord`

```sml
val devToWord : dev -> SysWord.word
```

These functions convert between [`dev`](posix-file-sys.md#SIG:POSIX_FILE_SYS.dev:TY:SPEC) values and words by which the operating system identifies a device. There is no verification that a value created by `wordToDev` corresponds to a to a valid device identifier.

<span id="SIG:POSIX_FILE_SYS.ino:TY"></span>**`eqtype`**` ino`  
File serial number (_inode_).

```repl
Posix.FileSys.devToWord (Posix.FileSys.ST.dev (Posix.FileSys.stat "."));; (* device word *)
```
<span id="SIG:POSIX_FILE_SYS.wordToIno:VAL"></span>

### `wordToIno`

```sml
val wordToIno : SysWord.word -> ino
```

```repl
Posix.FileSys.wordToIno (Posix.FileSys.inoToWord (Posix.FileSys.ST.ino (Posix.FileSys.stat ".")));; (* inode identifier *)
```
<span id="SIG:POSIX_FILE_SYS.inoToWord:VAL"></span>

### `inoToWord`

```sml
val inoToWord : ino -> SysWord.word
```

These functions convert between [`ino`](posix-file-sys.md#SIG:POSIX_FILE_SYS.ino:TY:SPEC) values and words by which the operating system identifies an inode. There is no verification that a value created by `wordToIno` corresponds to a to a valid inode.

<span id="SIG:POSIX_FILE_SYS.ST:STR"></span>
**`structure`**` ST`  

<span id="SIG:POSIX_FILE_SYS.ST.stat:TY"></span>**`type`**` stat`  
This type models status information concerning a file.

```repl
Posix.FileSys.inoToWord (Posix.FileSys.ST.ino (Posix.FileSys.stat "."));; (* inode word *)
```
<span id="SIG:POSIX_FILE_SYS.ST.isDir:VAL"></span>

### `isDir`

```sml
val isDir : stat -> bool
```

```repl
Posix.FileSys.ST.isDir (Posix.FileSys.stat ".");; (* true *)
```
<span id="SIG:POSIX_FILE_SYS.isChr:VAL"></span>

### `isChr`

```sml
val isChr : stat -> bool
```

```repl
Posix.FileSys.ST.isChr (Posix.FileSys.stat ".");; (* false for an ordinary directory *)
```
<span id="SIG:POSIX_FILE_SYS.isBlk:VAL"></span>

### `isBlk`

```sml
val isBlk : stat -> bool
```

```repl
Posix.FileSys.ST.isBlk (Posix.FileSys.stat ".");; (* false for an ordinary directory *)
```
<span id="SIG:POSIX_FILE_SYS.isReg:VAL"></span>

### `isReg`

```sml
val isReg : stat -> bool
```

```repl
Posix.FileSys.ST.isReg (Posix.FileSys.stat ".");; (* false for an ordinary directory *)
```
<span id="SIG:POSIX_FILE_SYS.isFIFO:VAL"></span>

### `isFIFO`

```sml
val isFIFO : stat -> bool
```

```repl
Posix.FileSys.ST.isFIFO (Posix.FileSys.stat ".");; (* false for an ordinary directory *)
```
<span id="SIG:POSIX_FILE_SYS.isLink:VAL"></span>

### `isLink`

```sml
val isLink : stat -> bool
```

```repl
Posix.FileSys.ST.isLink (Posix.FileSys.lstat ".");; (* whether current directory is a symbolic link *)
```
<span id="SIG:POSIX_FILE_SYS.isSock:VAL"></span>

### `isSock`

```sml
val isSock : stat -> bool
```

These functions return `true` if the file described by the parameter is, respectively, a _directory_, a _character special device_, a _block special device_, a _regular file_, a _FIFO_, a _symbolic link_, or a _socket_.

```repl
Posix.FileSys.ST.isSock (Posix.FileSys.stat ".");; (* false for an ordinary directory *)
```
<span id="SIG:POSIX_FILE_SYS.ST.mode:VAL"></span>

### `mode`

```sml
val mode : stat -> S.mode
```
`mode ``st`` `  
returns the protection mode of the file described by `st`.

```repl
Posix.FileSys.ST.mode (Posix.FileSys.stat ".");; (* directory protection mode *)
```
<span id="SIG:POSIX_FILE_SYS.ST.ino:VAL"></span>

### `ino`

```sml
val ino : stat -> ino
```

```repl
Posix.FileSys.ST.ino (Posix.FileSys.stat ".");; (* inode number *)
```
<span id="SIG:POSIX_FILE_SYS.dev:VAL"></span>

### `dev`

```sml
val dev : stat -> dev
```

These return the file serial number (inode) and the device identifier, respectively, of the corresponding file.

```repl
Posix.FileSys.ST.dev (Posix.FileSys.stat ".");; (* device identifier *)
```
<span id="SIG:POSIX_FILE_SYS.ST.nlink:VAL"></span>

### `nlink`

```sml
val nlink : stat -> int
```
`nlink ``st`` `  
returns the number of hard links to the file described by `st`.

```repl
Posix.FileSys.ST.nlink (Posix.FileSys.stat ".");; (* hard-link count *)
```
<span id="SIG:POSIX_FILE_SYS.ST.uid:VAL"></span>

### `uid`

```sml
val uid : stat -> uid
```

```repl
Posix.FileSys.ST.uid (Posix.FileSys.stat ".");; (* owner ID *)
```
<span id="SIG:POSIX_FILE_SYS.gid:VAL"></span>

### `gid`

```sml
val gid : stat -> gid
```

These return the owner and group ID of the file.

```repl
Posix.FileSys.ST.gid (Posix.FileSys.stat ".");; (* group ID *)
```
<span id="SIG:POSIX_FILE_SYS.ST.size:VAL"></span>

### `size`

```sml
val size : stat -> Position.int
```
`size ``st`` `  
returns the size (number of bytes) of the file described by `st`.

```repl
Posix.FileSys.ST.size (Posix.FileSys.stat ".");; (* directory size in bytes *)
```
<span id="SIG:POSIX_FILE_SYS.ST.atime:VAL"></span>

### `atime`

```sml
val atime : stat -> Time.time
```

```repl
Posix.FileSys.ST.atime (Posix.FileSys.stat ".");; (* access time *)
```
<span id="SIG:POSIX_FILE_SYS.mtime:VAL"></span>

### `mtime`

```sml
val mtime : stat -> Time.time
```

```repl
Posix.FileSys.ST.mtime (Posix.FileSys.stat ".");; (* modification time *)
```
<span id="SIG:POSIX_FILE_SYS.ctime:VAL"></span>

### `ctime`

```sml
val ctime : stat -> Time.time
```

These functions return, respectively, the last access time, the last modification time or the last status change time of the file.

```repl
Posix.FileSys.ST.ctime (Posix.FileSys.stat ".");; (* status-change time *)
```
<span id="SIG:POSIX_FILE_SYS.stat:VAL"></span>

### `stat`

```sml
val stat : string -> ST.stat
```

```repl
Posix.FileSys.stat ".";; (* status for current directory *)
```
<span id="SIG:POSIX_FILE_SYS.lstat:VAL"></span>

### `lstat`

```sml
val lstat : string -> ST.stat
```

```repl
Posix.FileSys.lstat ".";; (* status of the path itself *)
```
<span id="SIG:POSIX_FILE_SYS.fstat:VAL"></span>

### `fstat`

```sml
val fstat : file_desc -> ST.stat
```

These return information on a file system object. For [`stat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.stat:VAL:SPEC) and [`lstat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.lstat:VAL:SPEC), the object is specified by its pathname. Note that an empty string causes an exception. For [`fstat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.fstat:VAL:SPEC), an open file descriptor is supplied.

[`lstat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.lstat:VAL:SPEC) differs from [`stat`](posix-file-sys.md#SIG:POSIX_FILE_SYS.stat:VAL:SPEC) in that, if the pathname argument is a symbolic link, the information concerns the link itself, not the file to which the link points.

<span id="SIG:POSIX_FILE_SYS.access_mode:TY"></span>**`datatype`**` access_mode = A_READ | A_WRITE | A_EXEC`  
This type is identical to [`OS.FileSys.access_mode`](os-file-sys.md#SIG:OS_FILE_SYS.access_mode:TY:SPEC).

```repl
Posix.FileSys.fstat Posix.FileSys.stdin;; (* status for standard input *)
```
<span id="SIG:POSIX_FILE_SYS.access:VAL"></span>

### `access`

```sml
val access : string * access_mode list -> bool
```
`access (``s``, ``l``) `  
checks for accessibility of file `s`. If `l` is the empty list, it checks for the existence of the file; if `l` contains [`A_READ`](posix-file-sys.md#SIG:POSIX_FILE_SYS.access_mode:TY:SPEC), it checks for the readability of `s` based on the real user and group IDs of the process; and so on.

The value returned depends only the appropriate privileges of the process and the permissions of the file. A directory may be indicated as writable by [`access`](posix-file-sys.md#SIG:POSIX_FILE_SYS.access:VAL:SPEC), but an attempt to open it for writing will fail (although files may be created there). A file's permissions may indicate that it is executable, but the [`exec`](posix-process.md#SIG:POSIX_PROCESS.exec:VAL:SPEC) can fail if the file is not in the proper format. Conversely, if the process has appropriate privileges, [`access`](posix-file-sys.md#SIG:POSIX_FILE_SYS.access:VAL:SPEC) will return `true` if none of the appropriate file permissions are set.

```repl
Posix.FileSys.access (".", [Posix.FileSys.A_READ]);; (* true when readable *)
```
<span id="SIG:POSIX_FILE_SYS.chmod:VAL"></span>

### `chmod`

```sml
val chmod : string * S.mode -> unit
```
`chmod (``s``, ``mode``) `  
changes the permissions of `s` to `mode`.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val _ = Posix.FileSys.chmod (p, Posix.FileSys.S.irusr) in Posix.FileSys.unlink p end;; (* temporary file mode changed and file removed *)
```
<span id="SIG:POSIX_FILE_SYS.fchmod:VAL"></span>

### `fchmod`

```sml
val fchmod : file_desc * S.mode -> unit
```
`fchmod (``fd``, ``mode``) `  
changes the permissions of the file opened as `fd` to `mode`.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.FileSys.fchmod (fd, Posix.FileSys.S.irusr); val _ = Posix.IO.close fd in Posix.FileSys.unlink p end;; (* temporary file mode changed and file removed *)
```
<span id="SIG:POSIX_FILE_SYS.chown:VAL"></span>

### `chown`

```sml
val chown : string * uid * gid -> unit
```
`chown (``s``, ``uid``, ``gid``) `  
changes the owner and group of file `s` to `uid` and `gid`, respectively.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val _ = Posix.FileSys.chown (p, Posix.ProcEnv.geteuid (), Posix.ProcEnv.getegid ()) in Posix.FileSys.unlink p end;; (* ownership set to current user and group *)
```
<span id="SIG:POSIX_FILE_SYS.fchown:VAL"></span>

### `fchown`

```sml
val fchown : file_desc * uid * gid -> unit
```
`fchown (``fd``, ``uid``, ``gid``) `  
changes the owner and group of the file opened as `fd` to `uid` and `gid`, respectively.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.FileSys.fchown (fd, Posix.ProcEnv.geteuid (), Posix.ProcEnv.getegid ()); val _ = Posix.IO.close fd in Posix.FileSys.unlink p end;; (* ownership set to current user and group *)
```
<span id="SIG:POSIX_FILE_SYS.utime:VAL"></span>

### `utime`

```sml
val utime : string
* {actime : Time.time, modtime : Time.time} option -> unit
```
`utime (``f``, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`{``actime``,``modtime``}) `  
sets the access and modification times of the file `f` to `actime` and `modtime`, respectively.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val now = Time.now (); val _ = Posix.FileSys.utime (p, SOME {actime = now, modtime = now}) in Posix.FileSys.unlink p end;; (* temporary file timestamps set and file removed *)
```
`utime (``f``, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `  
sets the access and modification times of a file to the current time.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.IO.close fd; val now = Time.now (); val _ = Posix.FileSys.utime (p, SOME {actime = now, modtime = now}); val _ = Posix.FileSys.utime (p, NONE) in Posix.FileSys.unlink p end;; (* timestamps set, then reset to now *)
```
<span id="SIG:POSIX_FILE_SYS.ftruncate:VAL"></span>

### `ftruncate`

```sml
val ftruncate : file_desc * Position.int -> unit
```
`ftruncate (``fd``, ``n``) `  
changes the length of a file opened as `fd` to `n` bytes. If the new length is less than the previous length, all data beyond `n` bytes is discarded. If the new length is greater than the previous length, the file is extended to its new length by the necessary number of zero bytes.

```repl
let val p = OS.FileSys.tmpName (); val fd = Posix.FileSys.creat (p, Posix.FileSys.S.irwxu); val _ = Posix.FileSys.ftruncate (fd, 0); val _ = Posix.IO.close fd in Posix.FileSys.unlink p end;; (* temporary file truncated to zero bytes *)
```
<span id="SIG:POSIX_FILE_SYS.pathconf:VAL"></span>

### `pathconf`

```sml
val pathconf : string * string -> SysWord.word option
```
`pathconf (``s``, ``p``) `

```repl
Posix.FileSys.pathconf (".", "NAME_MAX");; (* maximum filename length, when supported *)
```
<span id="SIG:POSIX_FILE_SYS.fpathconf:VAL"></span>

### `fpathconf`

```sml
val fpathconf : file_desc * string -> SysWord.word option
```

`fpathconf (``fd``, ``p``)`  
These functions return the value of property `p` of the file system underlying the file specified by `s` or `fd`. For integer-valued properties, if the value is unbounded, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. If the value is bounded, [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``v``)` is returned, where `v` is the value. For boolean-value properties, if the value is true, [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(1)` is returned; otherwise, [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(0)` or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. The [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception is raised if something goes wrong, including when `p` is not a valid property or when the implementation does not associate the property with the file.

In the case of [`pathconf`](posix-file-sys.md#SIG:POSIX_FILE_SYS.pathconf:VAL:SPEC), read, write, or execute permission of the named file is not required, but all directories in the path leading to the file must be searchable.

The properties required by POSIX are described below. A given implementation may support additional properties.

`"CHOWN_RESTRICTED"`  
True if the use of [`chown`](posix-file-sys.md#SIG:POSIX_FILE_SYS.chown:VAL:SPEC) on any files (other than directories) in the specified directory is restricted to processes with appropriate privileges. This property only applies to directories.

`"LINK_MAX"`  
The maximum value of a file's link count as returned by the [`ST.nlink`](posix-file-sys.md#SIG:POSIX_FILE_SYS.ST.nlink:VAL:SPEC) function.

`"MAX_CANON"`  
The maximum number of bytes that can be stored in an input queue. This property only applies to terminal devices.

`"MAX_INPUT"`  
The maximum number of bytes allowed in an input queue before being read by a process. This property only applies to terminal devices.

`"NAME_MAX"`  
The maximum number of bytes in a filename. This value may be as small as 13, but is never larger than 255. This property only applies to directories and its value applies to filenames within the directory.

`"NO_TRUNC"`  
True if supplying a filename longer than allowed by `"NAME_MAX"` causes an error; false if long filenames are truncated. This property only applies to directories.

`"PATH_MAX"`  
The maximum number of bytes in a pathname. This value is never larger than 65,535 and is the maximum length of a relative pathname when the specified directory is the working directory. This property only applies to directories.

`"PIPE_BUF"`  
Maximum number of bytes guaranteed to be written atomically. This is applicable only to a FIFO. The value returned applies to the referenced object. If the path or file descriptor parameter refers to a directory, the value returned applies to any FIFO that exists or can be created within the directory.

`"VDISABLE"`  
If defined, the integer code `ord(c)` of the character `c` which can be used to disable the terminal special characters specified in [`Posix.TTY.V`](posix-tty.md#SIG:POSIX_TTY.V:STR:SPEC). This property only applies to terminal devices.

`"ASYNC_IO"`  
True if asynchronous input or output operations may be performed on the file.

`"SYNC_IO"`  
True if synchronous input or output operations may be performed on the file.

`"PRIO_IO"`  
True if prioritized input or output operations may be performed on the file.

> **Implementation note:**
>
> An implementation can call the operating system's `pathconf` or `fpathconf` functions, which return an integer. If the returned value is -1 and `errno` has been set, an exception is raised. Otherwise, a returned value of -1 should be mapped to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), and other values should be wrapped in [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) and returned.

> **Rationale:**
>
> The encoding of boolean values as `int option`, with false having two values, is an unpleasant choice. It would be preferable to split these two functions into four, with one pair handling integer-valued properties, with the present return type, and the other pair handling boolean-valued properties, returning values of type [`bool`](bool.md#SIG:BOOL.bool:TY:SPEC). Unfortunately, the nature of the POSIX `pathconf` and `fpathconf` functions would make this a nightmare for the implementor.
>
> First, the specification of these functions provides a non-negative integer return value for both booleans and numbers. System include files provide no inherent information as to the type of a property. Although the basic properties specified by POSIX have fixed types, each system is allowed to add its own non-standard properties. Thus, for an SML implementation to make the distinction, it would have to rely on somehow gleaning the information from, _e.g._, system-specific manual pages.
>
> In addition, the POSIX specification is unclear on how boolean values are encoded. Some systems return 0 for false; others appear to return -1 without setting `errno`. Technically, the latter value may be interpreted as meaning that the property value is unknown or unspecified. From the programmer's point of view, this means that the property is not usable.
>
> This situation probably precludes automatically generating these functions on a per system basis. Given this, the current return types and values appear to be the only reasonable choice.


#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`Posix.ProcEnv`](posix.md#SIG:POSIX.ProcEnv:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

```repl
Posix.FileSys.pathconf (".", "NAME_MAX");; (* maximum filename length, when supported *)
```
