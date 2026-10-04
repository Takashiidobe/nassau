# <span id="section:0"></span>The `Posix.IO` structure

---

#### Synopsis

<span id="POSIX_IO:SIG:SPEC"></span>
<span id="IO:STR:SPEC"></span>

```sml
signature POSIX_IO
structure IO : POSIX_IO
```

The structure `Posix.IO` specifies functions that provide the primitive POSIX input/output operations, as described in Section 6 of the POSIX standard 1003.1,1996**\[CITE\]**.

---

#### Interface

<span id="SIG:POSIX_IO.file_desc:TY:SPEC"></span>
<span id="SIG:POSIX_IO.pid:TY:SPEC"></span>
<span id="SIG:POSIX_IO.pipe:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.dup:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.dup2:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.close:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.readVec:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.readArr:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.writeVec:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.writeArr:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.whence:TY:SPEC"></span>
<span id="SIG:POSIX_IO.SEEK_SET:TY:SPEC"></span>
<span id="SIG:POSIX_IO.SEEK_CUR:TY:SPEC"></span>
<span id="SIG:POSIX_IO.SEEK_END:TY:SPEC"></span>
<span id="SIG:POSIX_IO.cloexec:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.append:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.nonblock:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.sync:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.open_mode:TY:SPEC"></span>
<span id="SIG:POSIX_IO.O_RDONLY:TY:SPEC"></span>
<span id="SIG:POSIX_IO.O_WRONLY:TY:SPEC"></span>
<span id="SIG:POSIX_IO.O_RDWR:TY:SPEC"></span>
<span id="SIG:POSIX_IO.dupfd:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.getfd:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.setfd:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.getfl:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.setfl:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.lseek:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.fsync:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.lock_type:TY:SPEC"></span>
<span id="SIG:POSIX_IO.F_RDLCK:TY:SPEC"></span>
<span id="SIG:POSIX_IO.F_WRLCK:TY:SPEC"></span>
<span id="SIG:POSIX_IO.F_UNLCK:TY:SPEC"></span>
<span id="SIG:POSIX_IO.flock:TY:SPEC"></span>
<span id="SIG:POSIX_IO.flock:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.ltype:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.whence:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.start:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.len:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.pid:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.getlk:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.setlk:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.setlkw:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.mkBinReader:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.mkTextReader:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.mkBinWriter:VAL:SPEC"></span>
<span id="SIG:POSIX_IO.mkTextWriter:VAL:SPEC"></span>

```sml
eqtype file_desc
eqtype pid
val pipe : unit -> {infd : file_desc, outfd : file_desc}
val dup : file_desc -> file_desc
val dup2 : {old : file_desc, new : file_desc} -> unit
val close : file_desc -> unit
val readVec : file_desc * int -> Word8Vector.vector
val readArr : file_desc * Word8ArraySlice.slice -> int
val writeVec : file_desc * Word8VectorSlice.slice -> int
val writeArr : file_desc * Word8ArraySlice.slice -> int
datatype whence
= SEEK_SET
| SEEK_CUR
| SEEK_END
structure FD : sig
include BIT_FLAGS
val cloexec : flags
end
structure O : sig
include BIT_FLAGS
val append : flags
val nonblock : flags
val sync : flags
end
datatype open_mode
= O_RDONLY
| O_WRONLY
| O_RDWR
val dupfd : {old : file_desc, base : file_desc} -> file_desc
val getfd : file_desc -> FD.flags
val setfd : file_desc * FD.flags -> unit
val getfl : file_desc -> O.flags * open_mode
val setfl : file_desc * O.flags -> unit
val lseek : file_desc * Position.int * whence -> Position.int
val fsync : file_desc -> unit
datatype lock_type
= F_RDLCK
| F_WRLCK
| F_UNLCK
structure FLock : sig
type flock
val flock : {
ltype : lock_type,
whence : whence,
start : Position.int,
len : Position.int,
pid : pid option
} -> flock
val ltype : flock -> lock_type
val whence : flock -> whence
val start : flock -> Position.int
val len : flock -> Position.int
val pid : flock -> pid option
end
val getlk : file_desc * FLock.flock -> FLock.flock
val setlk : file_desc * FLock.flock -> FLock.flock
val setlkw : file_desc * FLock.flock -> FLock.flock
val mkBinReader : {
fd : file_desc,
name : string,
initBlkMode : bool
} -> BinPrimIO.reader
val mkTextReader : {
fd : file_desc,
name : string,
initBlkMode : bool
} -> TextPrimIO.reader
val mkBinWriter : {
fd : file_desc,
name : string,
appendMode : bool,
initBlkMode : bool,
chunkSize : int
} -> BinPrimIO.writer
val mkTextWriter : {
fd : file_desc,
name : string,
appendMode : bool,
initBlkMode : bool,
chunkSize : int
} -> TextPrimIO.writer
```

#### Description

<span id="SIG:POSIX_IO.file_desc:TY"></span>**`eqtype`**` file_desc`  
Open file descriptor.

<span id="SIG:POSIX_IO.pid:TY"></span>**`eqtype`**` pid`  
A process ID, used as an identifier for an operating system process.

<span id="SIG:POSIX_IO.pipe:VAL"></span>**`val`**` pipe `**`:`**` unit `**`->`**` {infd `**`:`**` file_desc, outfd `**`:`**` file_desc}`  
This creates a pipe (channel) and returns two file descriptors that refer to the read (`infd`) and write (`outfd`) ends of the pipe.

<span id="SIG:POSIX_IO.dup:VAL"></span>
`dup ``fd`` `  
returns a new file descriptor that refers to the same open file, with the same file pointer and access mode, as `fd`. The underlying word (see [`Posix.FileSys.fdToWord`](posix-file-sys.md#SIG:POSIX_FILE_SYS.fdToWord:VAL:SPEC)) of the returned file descriptor is the lowest one available. It is equivalent to [`dupfd`](posix-io.md#SIG:POSIX_IO.dupfd:VAL:SPEC)` {``old``=``fd``, ``base``=`[`Posix.FileSys.wordToFD`](posix-file-sys.md#SIG:POSIX_FILE_SYS.wordToFD:VAL:SPEC)` 0w0}`.

<span id="SIG:POSIX_IO.dup2:VAL"></span>
`dup2 {``old``, ``new``} `  
duplicates the open file descriptor `old` as file descriptor `new`.

<span id="SIG:POSIX_IO.close:VAL"></span>
`close ``fd`` `  
closes the file descriptor `fd`.

<span id="SIG:POSIX_IO.readVec:VAL"></span>
`readVec (``fd``, ``n``) `  
reads at most `n` bytes from the file referred to by `fd`. The size of the resulting vector is the number of bytes that were successfully read, which may be less than `n`. This function returns the empty vector if end-of-stream is detected (or if `n` is `0`). It raises the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if `n` \< 0.

<span id="SIG:POSIX_IO.readArr:VAL"></span>
`readArr (``fd``, ``slice``) `  
reads bytes from the file specified by `fd` into the array slice `slice` and returns the number of bytes actually read. The end-of-file condition is marked by returning `0`, although `0` is also returned if the `slice` is empty. This function will raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there is some problem with the underlying system call (_e.g._, the file is closed).

<span id="SIG:POSIX_IO.writeVec:VAL"></span>
`writeVec (``fd``, ``slice``) `
`writeArr (``fd``, ``slice``)`  
These functions write the bytes the vector or array slice `slice` to the open file `fd`. Both functions return the number bytes actually written and will raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there is some problem with the underlying system call (_e.g._, the file is closed or there is insufficient disk space).

<span id="SIG:POSIX_IO.FD:STR"></span>
**`structure`**` FD`  

<span id="SIG:POSIX_IO.FD.cloexec:VAL"></span>**`val`**` cloexec `**`:`**` flags`  
File descriptor flag that, if set, will cause the file descriptor to be closed should the opening process replace itself (through `exec`, etc.). If `cloexec` is not set, the open file descriptor will be inherited by the new process.

<span id="SIG:POSIX_IO.O:STR"></span>
**`structure`**` O`  

<span id="SIG:POSIX_IO.O.append:VAL"></span>**`val`**` append `**`:`**` flags`  
File status flag which forces the file offset to be set to the end of the file prior to each write.

<span id="SIG:POSIX_IO.O.nonblock:VAL"></span>**`val`**` nonblock `**`:`**` flags`  
File status flag used to enable non-blocking I/O.

<span id="SIG:POSIX_IO.O.sync:VAL"></span>**`val`**` sync `**`:`**` flags`  
File status flag enabling writes using \`\`synchronized I/O file integrity completion.''

<span id="SIG:POSIX_IO.open_mode:TY"></span>
**`datatype`**` open_mode`  
Operations allowed on an open file.

`= O_RDONLY`  
Open a file for reading only.

`| O_WRONLY`  
Open a file for writing only.

`| O_RDWR`  
Open a file for reading and writing.

<span id="SIG:POSIX_IO.dupfd:VAL"></span>
`dupfd {``old``, ``base``} `  
returns a new file descriptor bound to `old`. The returned descriptor is greater than or equal to the file descriptor `base` based on the underlying integer mapping defined by [`Posix.FileSys.fdToWord`](posix-file-sys.md#SIG:POSIX_FILE_SYS.fdToWord:VAL:SPEC) and [`Posix.FileSys.wordToFD`](posix-file-sys.md#SIG:POSIX_FILE_SYS.wordToFD:VAL:SPEC). It corresponds to the POSIX `fcntl` function with the `F_DUPFD` command.

<span id="SIG:POSIX_IO.getfd:VAL"></span>
`getfd ``fd`` `  
gets the file descriptor flags associated with `fd`. It corresponds to the POSIX `fcntl` function with the `F_GETFD` command.

<span id="SIG:POSIX_IO.setfd:VAL"></span>
`setfd (``fd``, ``fl``) `  
sets the flags of file descriptor `fd` to `fl`. It corresponds to the POSIX `fcntl` function with the `F_SETFD` command.

<span id="SIG:POSIX_IO.getfl:VAL"></span>
`getfl ``fd`` `  
gets the file status flags for the open file descriptor `fd` and the access mode in which the file was opened. It corresponds to the POSIX `fcntl` function with the `F_GETFL` command.

<span id="SIG:POSIX_IO.setfl:VAL"></span>
`setfl (``fd``, ``fl``) `  
sets the file status flags for the open file descriptor `fd` to `fl`. It corresponds to the POSIX `fcntl` function with the `F_SETFL` command.

<span id="SIG:POSIX_IO.lseek:VAL"></span>
`lseek (``fd``, ``off``, ``wh``) `  
sets the file offset for the open file descriptor `fd` to `off` if `wh` is [`SEEK_SET`](posix-io.md#SIG:POSIX_IO.whence:TY:SPEC); to its current value plus `off` bytes if `wh` is [`SEEK_CUR`](posix-io.md#SIG:POSIX_IO.whence:TY:SPEC); or, to the size of the file plus `off` bytes if `wh` is [`SEEK_END`](posix-io.md#SIG:POSIX_IO.whence:TY:SPEC). Note that `off` may be negative.

<span id="SIG:POSIX_IO.fsync:VAL"></span>
`fsync ``fd`` `  
indicates that all data for the open file descriptor `fd` is to be transferred to the device associated with the descriptor; it is similar to a \`\`flush'' operation.

<span id="SIG:POSIX_IO.lock_type:TY"></span>**`datatype`**` lock_type`
`  = F_RDLCK`
`  | F_WRLCK`
`  | F_UNLCK`  
These constructors denote the kind of lock. [`F_RDLCK`](posix-io.md#SIG:POSIX_IO.lock_type:TY:SPEC) indicates a shared or read lock. [`F_WRLCK`](posix-io.md#SIG:POSIX_IO.lock_type:TY:SPEC) indicates an exclusive or write lock. [`F_WRLCK`](posix-io.md#SIG:POSIX_IO.lock_type:TY:SPEC) indicates a lock is unlocked or inactive.

<span id="SIG:POSIX_IO.FLock:STR"></span>
**`structure`**` FLock`  

<span id="SIG:POSIX_IO.FLock.flock:TY"></span>**`type`**` flock`  
Type representing an advisory lock. It can be considered an abstraction of the record used as the argument to the [`flock`](posix-io.md#SIG:POSIX_IO.FLock.flock:VAL:SPEC) function below.

<span id="SIG:POSIX_IO.FLock.flock:VAL"></span>
`flock {``ltype``, ``whence``, ``start``, ``len``, ``pid``} `  
creates a [`flock`](posix-io.md#SIG:POSIX_IO.FLock.flock:TY:SPEC) value described by the parameters. The `whence` and `start` parameters give the beginning file position as in [`lseek`](posix-io.md#SIG:POSIX_IO.lseek:VAL:SPEC). The `len` value provides the number of bytes to be locked. If the section starts at the beginning of the file and `len`` = 0`, then the entire file is locked. Normally, `pid` will be [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). This value is only used in a [`flock`](posix-io.md#SIG:POSIX_IO.FLock.flock:TY:SPEC) returned by [`getlk`](posix-io.md#SIG:POSIX_IO.getlk:VAL:SPEC).

<span id="SIG:POSIX_IO.FLock.ltype:VAL"></span>**`val`**` ltype `**`:`**` flock `**`->`**` lock_type`
**`val`**` whence `**`:`**` flock `**`->`**` whence`
**`val`**` start `**`:`**` flock `**`->`**` Position.int`
**`val`**` len `**`:`**` flock `**`->`**` Position.int`
**`val`**` pid `**`:`**` flock `**`->`**` pid option`  
These are projection functions for the fields composing a [`flock`](posix-io.md#SIG:POSIX_IO.FLock.flock:TY:SPEC) value.

<span id="SIG:POSIX_IO.getlk:VAL"></span>
`getlk (``fd``, ``fl``) `  
gets the first lock that blocks the lock description `fl` on the open file descriptor `fd`. It corresponds to the POSIX `fcntl` function with the `F_GETLK` command.

<span id="SIG:POSIX_IO.setlk:VAL"></span>
`setlk (``fd``, ``fl``) `  
sets or clears a file segment lock according to the lock description `fl` on the open file descriptor `fd`. An exception is raised immediately if a shared or exclusive lock cannot be set. It corresponds to the POSIX `fcntl` function with the `F_SETLK` command.

<span id="SIG:POSIX_IO.setlkw:VAL"></span>**`val`**` setlkw `**`:`**` file_desc `**`*`**` FLock.flock `**`->`**` FLock.flock`  
This is similar to the [`setlk`](posix-io.md#SIG:POSIX_IO.setlk:VAL:SPEC) function above except that [`setlkw`](posix-io.md#SIG:POSIX_IO.setlkw:VAL:SPEC) waits on blocked locks until they are released. It corresponds to the POSIX `fcntl` function with the `F_SETLKW` command.

<span id="SIG:POSIX_IO.mkBinReader:VAL"></span>**`val`**` mkBinReader `**`:`**` {`
`                      fd `**`:`**` file_desc,`
`                      name `**`:`**` string,`
`                      initBlkMode `**`:`**` bool`
`                    } `**`->`**` BinPrimIO.reader`
**`val`**` mkTextReader `**`:`**` {`
`                       fd `**`:`**` file_desc,`
`                       name `**`:`**` string,`
`                       initBlkMode `**`:`**` bool`
`                     } `**`->`**` TextPrimIO.reader`  
These functions convert an open POSIX file descriptor into a reader. From this, one can then construct an input stream. The functions are comparable to the POSIX function `fdopen`.

The argument fields have the following meanings:

`fd`  
A file descriptor for a file opened for reading.

`name`  
The name associated with the file, used in error messages shown to the user.

`initBlkMode`  
False if the file is currently in non-blocking mode, _i.e._, if the flag `O.nonblock` is set in `#1(getfl fd)`.


<span id="SIG:POSIX_IO.mkBinWriter:VAL"></span>**`val`**` mkBinWriter `**`:`**` {`
`                      fd `**`:`**` file_desc,`
`                      name `**`:`**` string,`
`                      appendMode `**`:`**` bool,`
`                      initBlkMode `**`:`**` bool,`
`                      chunkSize `**`:`**` int`
`                    } `**`->`**` BinPrimIO.writer`
**`val`**` mkTextWriter `**`:`**` {`
`                       fd `**`:`**` file_desc,`
`                       name `**`:`**` string,`
`                       appendMode `**`:`**` bool,`
`                       initBlkMode `**`:`**` bool,`
`                       chunkSize `**`:`**` int`
`                     } `**`->`**` TextPrimIO.writer`  
These functions convert an open POSIX file descriptor into a writer. From this, one can then construct an output stream. The functions are comparable to the POSIX function `fdopen`.

The argument fields have the following meanings:

`fd`  
A file descriptor for a file opened for writing.

`name`  
The name associated with the file, used in error messages shown to the user.

`initBlkMode`  
False if the file is currently in non-blocking mode, _i.e._, if the flag `O.nonblock` is set in `#1(getfl fd)`.

`appendMode`  
True if the file is in append mode, _i.e._, if the flag `O.append` is set in `#1(getfl fd)`.

`chunkSize`  
The recommended size of write operations for efficient writing.


#### Examples

```repl
Posix.IO.pipe ();;
```

#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Error`](posix.md#SIG:POSIX.Error:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC)

#### Discussion

> **Question:**
>
> Why don't mk\*Reader allow a chunkSize parameter? Or why do mk\*Writer allow this?
