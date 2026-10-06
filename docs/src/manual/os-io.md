# <span id="section:0"></span>The `OS.IO` structure

---

#### Synopsis

<span id="OS_IO:SIG:SPEC"></span>
<span id="IO:STR:SPEC"></span>

```sml
signature OS_IO
structure IO : OS_IO
```

The `OS.IO` structure provides a general interface for polling I/O devices. This interface has been modeled after the Unix SVR4 poll interface. A [`poll_desc`](os-io.md#SIG:OS_IO.poll_desc:TY:SPEC), created from an I/O descriptor, can be used to test for various polling conditions.

---

#### Interface

<span id="SIG:OS_IO.iodesc:TY:SPEC"></span>
<span id="SIG:OS_IO.hash:VAL:SPEC"></span>
<span id="SIG:OS_IO.compare:VAL:SPEC"></span>
<span id="SIG:OS_IO.iodesc_kind:TY:SPEC"></span>
<span id="SIG:OS_IO.kind:VAL:SPEC"></span>
<span id="SIG:OS_IO.file:VAL:SPEC"></span>
<span id="SIG:OS_IO.dir:VAL:SPEC"></span>
<span id="SIG:OS_IO.symlink:VAL:SPEC"></span>
<span id="SIG:OS_IO.tty:VAL:SPEC"></span>
<span id="SIG:OS_IO.pipe:VAL:SPEC"></span>
<span id="SIG:OS_IO.socket:VAL:SPEC"></span>
<span id="SIG:OS_IO.device:VAL:SPEC"></span>
<span id="SIG:OS_IO.poll_desc:TY:SPEC"></span>
<span id="SIG:OS_IO.poll_info:TY:SPEC"></span>
<span id="SIG:OS_IO.pollDesc:VAL:SPEC"></span>
<span id="SIG:OS_IO.pollToIODesc:VAL:SPEC"></span>
<span id="SIG:OS_IO.Poll:EXN:SPEC"></span>
<span id="SIG:OS_IO.pollIn:VAL:SPEC"></span>
<span id="SIG:OS_IO.pollOut:VAL:SPEC"></span>
<span id="SIG:OS_IO.pollPri:VAL:SPEC"></span>
<span id="SIG:OS_IO.poll:VAL:SPEC"></span>
<span id="SIG:OS_IO.isIn:VAL:SPEC"></span>
<span id="SIG:OS_IO.isOut:VAL:SPEC"></span>
<span id="SIG:OS_IO.isPri:VAL:SPEC"></span>
<span id="SIG:OS_IO.infoToPollDesc:VAL:SPEC"></span>

```sml
eqtype iodesc
val hash : iodesc -> word
val compare : iodesc * iodesc -> order
eqtype iodesc_kind
val kind : iodesc -> iodesc_kind
structure Kind : sig
val file : iodesc_kind
val dir : iodesc_kind
val symlink : iodesc_kind
val tty : iodesc_kind
val pipe : iodesc_kind
val socket : iodesc_kind
val device : iodesc_kind
end
eqtype poll_desc
type poll_info
val pollDesc : iodesc -> poll_desc option
val pollToIODesc : poll_desc -> iodesc
exception Poll
val pollIn : poll_desc -> poll_desc
val pollOut : poll_desc -> poll_desc
val pollPri : poll_desc -> poll_desc
val poll : poll_desc list * Time.time option -> poll_info list
val isIn : poll_info -> bool
val isOut : poll_info -> bool
val isPri : poll_info -> bool
val infoToPollDesc : poll_info -> poll_desc
```

#### Description

<span id="SIG:OS_IO.iodesc:TY"></span>**`eqtype`**` iodesc`  
An `iodesc` is an abstraction for an opened OS object that supports I/O (_e.g._, a file, console, or socket). In Unix, an `iodesc` corresponds to a file descriptor, while in Microsoft Windows it corresponds to a file handle.

Since [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) values correspond to low-level, OS-specific objects, they are not typically created explicitly by the user, but are generated as a side-effect of the creation of a more high-level abstraction. For example, [`TextIO.openIn`](text-io.md#SIG:TEXT_IO.openIn:VAL:SPEC) creates an [`instream`](imperative-io.md#SIG:IMPERATIVE_IO.instream:TY:SPEC) value, from which the underlying `PrimIO.reader` can be accessed. This latter value may contain the corresponding [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) value.

If the underlying operating system is known, there will usually be mechanisms for converting between [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) values and the type of value used by the operating system. For example, the functions [`Posix.FileSys.fdToIOD`](posix-file-sys.md#SIG:POSIX_FILE_SYS.fdToIOD:VAL:SPEC) and [`Posix.FileSys.iodToFD`](posix-file-sys.md#SIG:POSIX_FILE_SYS.iodToFD:VAL:SPEC) provide this service for POSIX implementations, translating between [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC)s and open file descriptors.

<span id="SIG:OS_IO.hash:VAL"></span>

### `hash`

```sml
val hash : iodesc -> word
```
`hash ``iod`` `  
returns a hash value for the I/O descriptor `iod`.

> **Implementation note:**
>
> [`hash`](os-io.md#SIG:OS_IO.hash:VAL:SPEC) must have the property that values produced are well distributed when taken modulo 2<sup>(`n`)</sup> for any `n`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin in OS.IO.hash iod end;; (* descriptor hash *)
```
<span id="SIG:OS_IO.compare:VAL"></span>

### `compare`

```sml
val compare : iodesc * iodesc -> order
```
`compare (``iod``, ``iod'``) `  
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when `iod` is less than, equal to, or greater than `iod'`, respectively, in some underlying linear ordering on [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) values.

<span id="SIG:OS_IO.iodesc_kind:TY"></span>**`eqtype`**` iodesc_kind`  
This abstract type is used to represent the _kind_ of system object that an [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) represents. The possible values are defined in the [`Kind`](os-io.md#SIG:OS_IO.Kind:STR:SPEC) substructure.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin in OS.IO.compare (iod, iod) end;; (* EQUAL *)
```
<span id="SIG:OS_IO.kind:VAL"></span>

### `kind`

```sml
val kind : iodesc -> iodesc_kind
```
`kind ``iod`` `  
returns the kind of system object that the I/O descriptor `iod` represents. This will raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, `iod` refers to a closed file.

<span id="SIG:OS_IO.Kind:STR"></span>
**`structure`**` Kind`

```repl
OS.IO.kind (Posix.FileSys.fdToIOD Posix.FileSys.stdin);; (* descriptor kind *)
```
<span id="SIG:OS_IO.Kind.file:VAL"></span>

### `Kind.file`

```sml
val file : iodesc_kind
```
These values represent the various kinds of system objects that an I/O descriptor might represent. The following list summarizes the intended meaning of these values:

[`file`](os-io.md#SIG:OS_IO.Kind.file:VAL:SPEC)  
A regular file in the file system. The I/O descriptor associated with a stream produced by one of the [`BinIO`](bin-io.md#BinIO:STR:SPEC) or [`TextIO`](text-io.md#TextIO:STR:SPEC) file opening operations will always have this kind.

```repl
OS.IO.Kind.file;; (* file kind *)
```

<span id="SIG:OS_IO.Kind.dir:VAL"></span>

### `Kind.dir`

```sml
val dir : iodesc_kind
```

```repl
OS.IO.Kind.dir;; (* dir kind *)
```

[`dir`](os-io.md#SIG:OS_IO.Kind.dir:VAL:SPEC)  
A directory in the file system. I/O descriptors associated with file system objects for which [`OS.FileSys.isDir`](os-file-sys.md#SIG:OS_FILE_SYS.isDir:VAL:SPEC) returns `true` will have this kind.

<span id="SIG:OS_IO.Kind.symlink:VAL"></span>

### `Kind.symlink`

```sml
val symlink : iodesc_kind
```

```repl
OS.IO.Kind.symlink;; (* symlink kind *)
```

[`symlink`](os-io.md#SIG:OS_IO.Kind.symlink:VAL:SPEC)  
A symbolic link or file system alias. I/O descriptors associated with file system objects for which [`OS.FileSys.isLink`](os-file-sys.md#SIG:OS_FILE_SYS.isLink:VAL:SPEC) returns `true` will have this kind.

<span id="SIG:OS_IO.Kind.tty:VAL"></span>

### `Kind.tty`

```sml
val tty : iodesc_kind
```

```repl
OS.IO.Kind.tty;; (* tty kind *)
```

[`tty`](os-io.md#SIG:OS_IO.Kind.tty:VAL:SPEC)  
A terminal console.

<span id="SIG:OS_IO.Kind.pipe:VAL"></span>

### `Kind.pipe`

```sml
val pipe : iodesc_kind
```

```repl
OS.IO.Kind.pipe;; (* pipe kind *)
```

[`pipe`](os-io.md#SIG:OS_IO.Kind.pipe:VAL:SPEC)  
A pipe to another system process.

<span id="SIG:OS_IO.Kind.socket:VAL"></span>

### `Kind.socket`

```sml
val socket : iodesc_kind
```

```repl
OS.IO.Kind.socket;; (* socket kind *)
```

[`socket`](os-io.md#SIG:OS_IO.Kind.socket:VAL:SPEC)  
A network socket.

<span id="SIG:OS_IO.Kind.device:VAL"></span>

### `Kind.device`

```sml
val device : iodesc_kind
```

```repl
OS.IO.Kind.device;; (* device kind *)
```

[`device`](os-io.md#SIG:OS_IO.Kind.device:VAL:SPEC)  
A logical or physical hardware device.

Note that a given implementation may define other [`iodesc`](os-io.md#SIG:OS_IO.iodesc:TY:SPEC) values not covered by these definitions.

<span id="SIG:OS_IO.poll_desc:TY"></span>**`eqtype`**` poll_desc`  
An abstract representation of a polling operation on an I/O descriptor.

<span id="SIG:OS_IO.poll_info:TY"></span>**`type`**` poll_info`  
An abstract representation of the per-descriptor information returned by the `poll` operation.

<span id="SIG:OS_IO.pollDesc:VAL"></span>

### `pollDesc`

```sml
val pollDesc : iodesc -> poll_desc option
```
`pollDesc ``iod`` `  
create a polling operation on the given descriptor; [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned when no polling is supported by the I/O device.

```repl
OS.IO.pollDesc (Posix.FileSys.fdToIOD Posix.FileSys.stdin);; (* SOME poll descriptor when supported *)
```
<span id="SIG:OS_IO.pollToIODesc:VAL"></span>

### `pollToIODesc`

```sml
val pollToIODesc : poll_desc -> iodesc
```
`pollToIODesc ``pd`` `  
return the I/O descriptor that is being polled using `pd`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin in OS.IO.pollToIODesc (valOf (OS.IO.pollDesc iod)) end;; (* standard input descriptor *)
```
<span id="SIG:OS_IO.pollIn:VAL"></span>

### `pollIn`

```sml
val pollIn : poll_desc -> poll_desc
```
`pollIn ``pd`` `
` pollOut ``pd`` `
` pollPri ``pd`` `  
These return a poll descriptor that has input (respectively, output, high-priority) polling added to the poll descriptor `pd`. It raises [`Poll`](os-io.md#SIG:OS_IO.Poll:EXN:SPEC) if input (respectively, output, high-priority events) is not appropriate for the underlying I/O device.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin in OS.IO.pollIn (valOf (OS.IO.pollDesc iod)) end;; (* input poll descriptor *)
```
<span id="SIG:OS_IO.poll:VAL"></span>

### `poll`

```sml
val poll : poll_desc list * Time.time option -> poll_info list
```
`poll (``l``, ``timeout``) `  
polls a collection of I/O devices for the conditions specified by the list of poll descriptors `l`. The argument `timeout` specifies the timeout where:

- [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) means wait indefinitely.
- [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(Time.zeroTime)` means do not block.
- [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(t)` means timeout after time `t`.

This returns a list of [`poll_info`](os-io.md#SIG:OS_IO.poll_info:TY:SPEC) values corresponding to those descriptors in `l` whose conditions are enabled. The returned list respects the order of the argument list, and a value in the returned list will reflect a (nonempty) subset of the conditions specified in the corresponding argument descriptor. The `poll` function will raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if, for example, one of the file descriptors refers to a closed file.

```repl
OS.IO.poll ([], SOME Time.zeroTime);; (* [] *)
```
<span id="SIG:OS_IO.isIn:VAL"></span>

### `isIn`

```sml
val isIn : poll_info -> bool
```
returns `true` when input readiness is present in `info`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin; val pd = valOf (OS.IO.pollDesc iod) in case OS.IO.poll ([OS.IO.pollIn pd], SOME Time.zeroTime) of info :: _ => OS.IO.isIn info | [] => false end;; (* whether input is ready *)
```

<span id="SIG:OS_IO.isOut:VAL"></span>

### `isOut`

```sml
val isOut : poll_info -> bool
```
returns `true` when output readiness is present in `info`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdout; val pd = valOf (OS.IO.pollDesc iod) in case OS.IO.poll ([OS.IO.pollOut pd], SOME Time.zeroTime) of info :: _ => OS.IO.isOut info | [] => false end;; (* whether output is ready *)
```

<span id="SIG:OS_IO.isPri:VAL"></span>

### `isPri`

```sml
val isPri : poll_info -> bool
```
returns `true` when priority information is present in `info`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin; val pd = valOf (OS.IO.pollDesc iod) in case OS.IO.poll ([OS.IO.pollPri pd], SOME Time.zeroTime) of info :: _ => OS.IO.isPri info | [] => false end;; (* whether priority data is ready *)
```

<span id="SIG:OS_IO.infoToPollDesc:VAL"></span>

### `infoToPollDesc`

```sml
val infoToPollDesc : poll_info -> poll_desc
```
`infoToPollDesc ``pi`` `  
returns the underlying poll descriptor from poll information `pi`.

```repl
let val iod = Posix.FileSys.fdToIOD Posix.FileSys.stdin; val pd = valOf (OS.IO.pollDesc iod) in case OS.IO.poll ([pd], SOME Time.zeroTime) of info :: _ => OS.IO.infoToPollDesc info | [] => pd end;; (* poll descriptor *)
```

#### See Also

> [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC), [`OS`](os.md#OS:STR:SPEC)

#### Discussion

> **Question:**
>
> Why are the file attributes only available for open files?
