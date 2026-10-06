# <span id="section:0"></span>The `OS.FileSys` structure

---

#### Synopsis

<span id="OS_FILE_SYS:SIG:SPEC"></span>
<span id="FileSys:STR:SPEC"></span>

```sml
signature OS_FILE_SYS
structure FileSys : OS_FILE_SYS
```

The `OS.FileSys` structure provides facilities for accessing and operating on the file system. These functions are designed to be portable across operating systems. They raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) with an argument in case of errors.

Except for [`fullPath`](os-file-sys.md#SIG:OS_FILE_SYS.fullPath:VAL:SPEC) and [`realPath`](os-file-sys.md#SIG:OS_FILE_SYS.realPath:VAL:SPEC), functions taking a string argument will raise the [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if the argument string is empty.

It is expected that all functions taking a pathname as an argument (_e.g._, [`modTime`](os-file-sys.md#SIG:OS_FILE_SYS.modTime:VAL:SPEC) or [`OS.Process.system`](os-process.md#SIG:OS_PROCESS.system:VAL:SPEC)) will resolve any components corresponding to symbolic links. The obvious exceptions to this rule are [`isLink`](os-file-sys.md#SIG:OS_FILE_SYS.isLink:VAL:SPEC) and [`readLink`](os-file-sys.md#SIG:OS_FILE_SYS.readLink:VAL:SPEC), where only symbolic links appearing as directory components of the pathname are resolved.

> **Question:**
>
> We need a general discussion of dirstreams, working directory, directory structure, etc. The introduction should say something about the model of a file system that these functions use; what features they support and what examples of features that require an OS-specific library. We should also note that the particular semantics, especially concerning errors, is OS dependent.

---

#### Interface

<span id="SIG:OS_FILE_SYS.dirstream:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.openDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.readDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.rewindDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.closeDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.chDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.getDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.mkDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.rmDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.isDir:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.isLink:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.readLink:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.fullPath:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.realPath:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.modTime:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.fileSize:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.setTime:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.remove:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.rename:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.access_mode:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.A_READ:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.A_WRITE:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.A_EXEC:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.access:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.tmpName:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.file_id:TY:SPEC"></span>
<span id="SIG:OS_FILE_SYS.fileId:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.hash:VAL:SPEC"></span>
<span id="SIG:OS_FILE_SYS.compare:VAL:SPEC"></span>

```sml
type dirstream
val openDir : string -> dirstream
val readDir : dirstream -> string option
val rewindDir : dirstream -> unit
val closeDir : dirstream -> unit
val chDir : string -> unit
val getDir : unit -> string
val mkDir : string -> unit
val rmDir : string -> unit
val isDir : string -> bool
val isLink : string -> bool
val readLink : string -> string
val fullPath : string -> string
val realPath : string -> string
val modTime : string -> Time.time
val fileSize : string -> Position.int
val setTime : string * Time.time option -> unit
val remove : string -> unit
val rename : {old : string, new : string} -> unit
datatype access_mode = A_READ | A_WRITE | A_EXEC
val access : string * access_mode list -> bool
val tmpName : unit -> string
eqtype file_id
val fileId : string -> file_id
val hash : file_id -> word
val compare : file_id * file_id -> order
```

#### Description

<span id="SIG:OS_FILE_SYS.openDir:VAL"></span>

### `openDir`

```sml
val openDir : string -> dirstream
```
`openDir ``path`` `  
opens the directory specified by `path` and returns a directory stream for use with [`readDir`](os-file-sys.md#SIG:OS_FILE_SYS.readDir:VAL:SPEC), [`rewindDir`](os-file-sys.md#SIG:OS_FILE_SYS.rewindDir:VAL:SPEC), and [`closeDir`](os-file-sys.md#SIG:OS_FILE_SYS.closeDir:VAL:SPEC). The stream reads the directory entries off the file system in some unspecified order. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, the directory does not exist or is not accessible.

```repl
val d = OS.FileSys.openDir "."; OS.FileSys.closeDir d;; (* directory opened and closed *)
```
<span id="SIG:OS_FILE_SYS.readDir:VAL"></span>

### `readDir`

```sml
val readDir : dirstream -> string option
```
`readDir ``dir`` `  
returns and removes one filename from the directory stream `dir`. When the directory stream is empty (that is, when all entries have been read from the stream), [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. [`readDir`](os-file-sys.md#SIG:OS_FILE_SYS.readDir:VAL:SPEC) filters out the names corresponding to the current and parent arcs.

```repl
val d = OS.FileSys.openDir "."; val entry = OS.FileSys.readDir d; OS.FileSys.closeDir d; entry;; (* first entry, or NONE *)
```
<span id="SIG:OS_FILE_SYS.rewindDir:VAL"></span>

### `rewindDir`

```sml
val rewindDir : dirstream -> unit
```
`rewindDir ``dir`` `  
resets the directory stream `dir`, as if it had just been opened. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) in case of an operating system error, though, since the directory stream has already been opened, an error should not be likely.

```repl
val d = OS.FileSys.openDir "."; OS.FileSys.rewindDir d; OS.FileSys.closeDir d;; (* () *)
```
<span id="SIG:OS_FILE_SYS.closeDir:VAL"></span>

### `closeDir`

```sml
val closeDir : dirstream -> unit
```
`closeDir ``dir`` `  
closes the directory stream `dir`, releasing any system resources associated with it. Any subsequent read or rewind on the stream will raise exception [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC). Closing a closed directory stream, however, has no effect.

```repl
OS.FileSys.closeDir (OS.FileSys.openDir ".");; (* () *)
```
<span id="SIG:OS_FILE_SYS.chDir:VAL"></span>

### `chDir`

```sml
val chDir : string -> unit
```
`chDir ``s`` `  
changes the current working directory to `s`. This affects future calls to all functions that access the file system. These include the input/output functions such as [`TextIO.openIn`](text-io.md#SIG:TEXT_IO.openIn:VAL:SPEC) and [`TextIO.openOut`](text-io.md#SIG:TEXT_IO.openOut:VAL:SPEC), and functions defined in this structure. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, the directory does not exist or is not readable.

The [`chDir`](os-file-sys.md#SIG:OS_FILE_SYS.chDir:VAL:SPEC) function will also change the current volume (on systems with volumes) if one is specified. This function does not allow the user to change the current working directory of another volume than the current volume, even on systems where this concept is otherwise supported.

```repl
OS.FileSys.chDir ".";; (* () *)
```
<span id="SIG:OS_FILE_SYS.getDir:VAL"></span>

### `getDir`

```sml
val getDir : unit -> string
```
An absolute canonical pathname of the current working directory. This includes the current volume for systems supporting volumes.

```repl
OS.FileSys.getDir ();; (* current directory *)
```
<span id="SIG:OS_FILE_SYS.mkDir:VAL"></span>

### `mkDir`

```sml
val mkDir : string -> unit
```
`mkDir ``s`` `  
creates a directory `s` on the file system. If `s` has multiple arcs, each of the ancestor directories will need to be created first, if it does not already exist. `mkDir` raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, the directory in which `s` is to be created does not exist or is not writable.

```repl
val p = OS.FileSys.tmpName (); OS.FileSys.mkDir p; OS.FileSys.rmDir p;; (* temporary directory created and removed *)
```
<span id="SIG:OS_FILE_SYS.rmDir:VAL"></span>

### `rmDir`

```sml
val rmDir : string -> unit
```
`rmDir ``s`` `  
removes directory `s` from the file system. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `s` does not exist or if the directory in which `s` resides is not writable, or if the directory is not empty.

```repl
val p = OS.FileSys.tmpName (); OS.FileSys.mkDir p; OS.FileSys.rmDir p;; (* () *)
```
<span id="SIG:OS_FILE_SYS.isDir:VAL"></span>

### `isDir`

```sml
val isDir : string -> bool
```
`isDir ``s`` `  
tests whether `s` is a directory. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `s` does not exist or if the directory in which `s` resides is not accessible.

```repl
OS.FileSys.isDir ".";; (* true *)
```
<span id="SIG:OS_FILE_SYS.isLink:VAL"></span>

### `isLink`

```sml
val isLink : string -> bool
```
`isLink ``s`` `  
returns `true` if `s` names a symbolic link. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `s` does not exist or there is an access violation. On operating systems without symbolic links, it will always return `false` unless an exception is raised first.

```repl
OS.FileSys.isLink ".";; (* whether the current directory is a symbolic link *)
```
<span id="SIG:OS_FILE_SYS.readLink:VAL"></span>

### `readLink`

```sml
val readLink : string -> string
```
`readLink ``s`` `  
returns the contents of the symbolic link `s`. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `s` does not exist or is not a symbolic link, or there is an access violation. On operating systems without symbolic links, it raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) unconditionally.

The precise form of the returned string, in particular, whether it corresponds to an absolute or relative path, is system-dependent.

```repl
if OS.FileSys.isLink "/tmp" then OS.FileSys.readLink "/tmp" else "not a link";; (* link target when /tmp is a symbolic link *)
```
<span id="SIG:OS_FILE_SYS.fullPath:VAL"></span>

### `fullPath`

```sml
val fullPath : string -> string
```
`fullPath ``path`` `  
returns an absolute canonical path that names the same file system object as `path`. The resulting path will have a volume prefix (on systems supporting volumes), all occurrences of the current, parent, and empty arcs will have been expanded or removed, and any symbolic links will have been fully expanded. An empty `path` is treated as `"."`. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, a directory on the path, or the file or directory named, does not exist or is not accessible or if there is a link loop.

```repl
OS.FileSys.fullPath ".";; (* absolute path *)
```
<span id="SIG:OS_FILE_SYS.realPath:VAL"></span>

### `realPath`

```sml
val realPath : string -> string
```
`realPath ``path`` `  
returns a canonical path that names the same file system object as `path`. If `path` is an absolute path, then `realPath` acts like `fullPath`. If `path` is relative and on the same volume as the current working directory, then it returns a path that is relative to the current working directory, but in which the symbolic links have been expanded. Otherwise, it raises [`OS.Path.Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC).

> **Implementation note:**
>
> This function can be implemented as follows:
>
>         fun realPath p = if [OS.Path.isAbsolute](os-path.md#SIG:OS_PATH.isAbsolute:VAL:SPEC) p
>           then fullPath p
>           else [OS.Path.mkRelative](os-path.md#SIG:OS_PATH.mkRelative:VAL:SPEC){
>                       path=fullPath p, relativeTo=fullPath(getDir())
>                     }
>

```repl
OS.FileSys.realPath ".";; (* canonical absolute path *)
```
<span id="SIG:OS_FILE_SYS.modTime:VAL"></span>

### `modTime`

```sml
val modTime : string -> Time.time
```
`modTime ``path`` `  
returns the modification time of file `path`. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `path` does not exist or if the directory in which `path` resides is not accessible.

```repl
OS.FileSys.modTime ".";; (* modification time *)
```
<span id="SIG:OS_FILE_SYS.fileSize:VAL"></span>

### `fileSize`

```sml
val fileSize : string -> Position.int
```
`fileSize ``path`` `  
returns the size of file `path` in bytes. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `path` does not exist or if the directory in which `path` resides is not accessible.

```repl
OS.FileSys.fileSize ".";; (* size *)
```
<span id="SIG:OS_FILE_SYS.setTime:VAL"></span>

### `setTime`

```sml
val setTime : string * Time.time option -> unit
```
`setTime (``path``, ``opt``) `  
sets the modification and access time of file `path`. If `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``t``)`, then the time `t` is used; otherwise the current time (_i.e._, [`Time.now`](time.md#SIG:TIME.now:VAL:SPEC)`()`) is used. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `path` does not exist, the directory in which `path` resides is not accessible, or the user does not have the appropriate permission.

```repl
val p = OS.FileSys.tmpName (); val out = TextIO.openOut p; TextIO.closeOut out; OS.FileSys.setTime (p, NONE); OS.FileSys.remove p;; (* temporary file timestamp set, then file removed *)
```
<span id="SIG:OS_FILE_SYS.remove:VAL"></span>

### `remove`

```sml
val remove : string -> unit
```
`remove ``path`` `  
deletes the file `path` from the file system. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, `path` does not exist or is not writable, if the directory in which `path` resides is not writable, or `file` is a directory. Use the [`rmDir`](os-file-sys.md#SIG:OS_FILE_SYS.rmDir:VAL:SPEC) function to delete directories.

If one removes a file that has been opened for reading or writing, the behavior of subsequent reads and writes is undefined. For example, removing the file may close all existing streams or generate an exception. The Unix idiom of opening a file and then removing it is not portable.

```repl
val p = OS.FileSys.tmpName (); val out = TextIO.openOut p; TextIO.closeOut out; OS.FileSys.remove p;; (* temporary file removed *)
```
<span id="SIG:OS_FILE_SYS.rename:VAL"></span>

### `rename`

```sml
val rename : {old : string, new : string} -> unit
```
`rename {``old``, ``new``} `  
changes the name of file `old` to `new`. If `new` and `old` refer to the same file, `rename` does nothing. If a file called `new` exists, it is removed. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `old` does not exist, or if one of the directories in which `old` or `new` reside is not writable. This may also fail if `old` refers to an open file, or if `old` and `new` are on different file systems, _i.e._, if a copy is required.

```repl
val a = OS.FileSys.tmpName (); val b = a ^ "-renamed"; val out = TextIO.openOut a; TextIO.closeOut out; OS.FileSys.rename {old = a, new = b}; OS.FileSys.remove b;; (* temporary file renamed and removed *)
```
<span id="SIG:OS_FILE_SYS.access:VAL"></span>

### `access`

```sml
val access : string * access_mode list -> bool
```
`access (``path``, ``accs``) `  
tests the access permissions of file `path`, expanding symbolic links as necessary. If the list `accs` of required access modes is empty, it tests whether `path` exists. If `accs` contains [`A_READ`](os-file-sys.md#SIG:OS_FILE_SYS.access_mode:TY:SPEC), [`A_WRITE`](os-file-sys.md#SIG:OS_FILE_SYS.access_mode:TY:SPEC), or [`A_EXEC`](os-file-sys.md#SIG:OS_FILE_SYS.access_mode:TY:SPEC), respectively, it tests whether the user process has read, write, or execute permission for the file, testing their conjunction if more than one are present. Note that `access` is also implicitly testing the user's access to the parent directories of the file. The function will only raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) for errors unrelated to resolving the pathname and the related permissions, such as being interrupted by a signal during the system call.

> **Implementation note:**
>
> On systems that do not support a notion of execution permissions, the [`access`](os-file-sys.md#SIG:OS_FILE_SYS.access:VAL:SPEC) should accept but ignore the [`A_EXEC`](os-file-sys.md#SIG:OS_FILE_SYS.access_mode:TY:SPEC) value.

```repl
OS.FileSys.access (".", [OS.FileSys.A_READ]);; (* true when readable *)
```
<span id="SIG:OS_FILE_SYS.tmpName:VAL"></span>

### `tmpName`

```sml
val tmpName : unit -> string
```
This creates a new empty file with a unique name and returns the full pathname of the file. The named file will be readable and writable by the creating process, but, if the host operating systems supports it, not accessible by other users. This function can be used to create a temporary file that will not collide with other applications.

This function raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if it cannot create the unique file or filename.

<span id="SIG:OS_FILE_SYS.file_id:TY"></span>**`eqtype`**` file_id`  
A unique identifier associated with a file system object. A value of this type is not persistent across changes in the file system (_e.g._, mount/unmount) but it is better than pathnames for uniquely identifying files. A [`file_id`](os-file-sys.md#SIG:OS_FILE_SYS.file_id:TY:SPEC) value should not be confused with the open file identifier [`OS.IO.iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC).

```repl
OS.FileSys.tmpName ();; (* unique temporary path *)
```
<span id="SIG:OS_FILE_SYS.fileId:VAL"></span>

### `fileId`

```sml
val fileId : string -> file_id
```
`fileId ``path`` `  
returns the unique [`file_id`](os-file-sys.md#SIG:OS_FILE_SYS.file_id:TY:SPEC) value associated with the file system object designated by the pathname `path`. In particular, if `fileId`` ``p`` = ``fileId`` ``p'`, then the paths `p` and `p'` refer to the same file system object. Note that if `p` is a symbolic link, then `fileId`` ``p`` = ``fileId``(`[`readLink`](os-file-sys.md#SIG:OS_FILE_SYS.readLink:VAL:SPEC)` ``p``) `.

```repl
OS.FileSys.fileId ".";; (* file identifier *)
```
<span id="SIG:OS_FILE_SYS.hash:VAL"></span>

### `hash`

```sml
val hash : file_id -> word
```
`hash ``fid`` `  
returns a hash value associated with `fid`.

> **Implementation note:**
>
> `hash` must have the property that values produced are well distributed when taken modulo 2<sup>(`n`)</sup> for any `n`.

```repl
OS.FileSys.hash (OS.FileSys.fileId ".");; (* hash word *)
```
<span id="SIG:OS_FILE_SYS.compare:VAL"></span>

### `compare`

```sml
val compare : file_id * file_id -> order
```
`compare (``fid``, ``fid'``) `  
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when `fid` is less than, equal to, or greater than `fid'`, respectively, in some underlying linear ordering on [`file_id`](os-file-sys.md#SIG:OS_FILE_SYS.file_id:TY:SPEC) values.

```repl
OS.FileSys.compare (OS.FileSys.fileId ".", OS.FileSys.fileId ".");; (* EQUAL *)
```

#### See Also

> [`BinIO`](bin-io.md#BinIO:STR:SPEC), [`OS`](os.md#OS:STR:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC)

#### Discussion

For functions dealing with file attributes, such as [`fileSize`](os-file-sys.md#SIG:OS_FILE_SYS.fileSize:VAL:SPEC) or [`rename`](os-file-sys.md#SIG:OS_FILE_SYS.rename:VAL:SPEC), the arguments can be directories as well as ordinary files.
