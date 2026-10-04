# <span id="section:0"></span>The `OS` structure

---

#### Synopsis

<span id="OS:SIG:SPEC"></span>
<span id="OS:STR:SPEC"></span>

```sml
signature OS
structure OS :> OS
```

The `OS` structure is a container for a collection of structures for interacting with the operating system's file system, directory paths, processes, and I/O subsystem. The types and functions provided by the `OS` substructures are meant to present a model for handling these resources that is largely independent of the operating system.

The structure also declares the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception used to report operating system error conditions.

---

#### Interface

<span id="SIG:OS.syserror:TY:SPEC"></span>
<span id="SIG:OS.SysErr:EXN:SPEC"></span>
<span id="SIG:OS.errorMsg:VAL:SPEC"></span>
<span id="SIG:OS.errorName:VAL:SPEC"></span>
<span id="SIG:OS.syserror:VAL:SPEC"></span>

```sml
structure FileSys : OS_FILE_SYS
structure IO : OS_IO
structure Path : OS_PATH
structure Process : OS_PROCESS
eqtype syserror
exception SysErr of string * syserror option
val errorMsg : syserror -> string
val errorName : syserror -> string
val syserror : string -> syserror option
```

#### Description

<span id="SIG:OS.FileSys:STR"></span>**`structure`**` FileSys `**`:`**` `[`OS_FILE_SYS`](os-file-sys.md#OS_FILE_SYS:SIG:SPEC)  
File system: files and directories and their attributes.

<span id="SIG:OS.IO:STR"></span>**`structure`**` IO `**`:`**` `[`OS_IO`](os-io.md#OS_IO:SIG:SPEC)  
I/O polling.

<span id="SIG:OS.Path:STR"></span>**`structure`**` Path `**`:`**` `[`OS_PATH`](os-path.md#OS_PATH:SIG:SPEC)  
Syntactic manipulation of pathnames.

<span id="SIG:OS.Process:STR"></span>**`structure`**` Process `**`:`**` `[`OS_PROCESS`](os-process.md#OS_PROCESS:SIG:SPEC)  
Process control, exit status, and environment.

<span id="SIG:OS.syserror:TY"></span>**`eqtype`**` syserror`  
The type representing errors that arise when making calls to the run-time or operating system. These values are usually transmitted by the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception.

<span id="SIG:OS.SysErr:EXN"></span>**`exception`**` SysErr `**`of`**` string `**`*`**` syserror option`  
This exception is raised when a call to the runtime system or host operating system results in an error. The first argument is a descriptive string explaining the error, and the second argument optionally specifies the system error condition. The form and content of the description strings are operating system and implementation dependent, but if a `SysErr` exception has the form `SysErr(``s``,`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``e``)`, then we have [`errorMsg`](os.md#SIG:OS.errorMsg:VAL:SPEC)` ``e`` = ``s`. System errors that do not have corresponding [`syserror`](os.md#SIG:OS.syserror:TY:SPEC) value will result in `SysErr` being raised with a second argument of [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

<span id="SIG:OS.errorMsg:VAL"></span>
`errorMsg ``err`` `  
returns a string describing the system error identified by the error code `err`. The form and content of the description strings are operating system and implementation dependent.

<span id="SIG:OS.errorName:VAL"></span>
`errorName ``err`` `
` syserror ``s`` `  
These functions provide conversions between the abstract [`syserror`](os.md#SIG:OS.syserror:TY:SPEC) type, and their operating system dependent string names. The primary purpose of these functions is to provide a mechanism for dealing with error codes that might not have symbolic names defined for them in the operating system specific modules. The former function returns a unique name used for the [`syserror`](os.md#SIG:OS.syserror:TY:SPEC) value, while the latter returns the [`syserror`](os.md#SIG:OS.syserror:TY:SPEC) whose name is `s`, if it exists. If `e` is a [`syserror`](os.md#SIG:OS.syserror:TY:SPEC), then it should be the case that

[SOME](option.md#SIG:OPTION.option:TY:SPEC) `e` = [syserror](os.md#SIG:OS.syserror:VAL:SPEC)([errorName](os.md#SIG:OS.errorName:VAL:SPEC) `e`)


#### Examples

```repl
OS.Process.isSuccess OS.Process.success;;
```

#### See Also

> [`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC), [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC), [`OS.Process`](os.md#SIG:OS.Process:STR:SPEC)
