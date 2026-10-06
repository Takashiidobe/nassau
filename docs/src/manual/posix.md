# <span id="section:0"></span>The `Posix` structure

---

#### Synopsis

<span id="POSIX:SIG:SPEC"></span>
<span id="Posix:STR:SPEC"></span>

```sml
signature POSIX (* OPTIONAL *)
structure Posix :> POSIX (* OPTIONAL *)
```

This optional structure contains several substructures that are useful for interfacing to POSIX operating systems. For more complete information on the semantics of the types and functions provided in `Posix`, see the POSIX standard 1003.1,1996 document**\[CITE\]**.

---

#### Interface

```sml
structure Error : POSIX_ERROR
structure Signal : POSIX_SIGNAL
structure Process : POSIX_PROCESS
where type signal = Signal.signal
structure ProcEnv : POSIX_PROC_ENV
where type pid = Process.pid
structure FileSys : POSIX_FILE_SYS
where type file_desc = ProcEnv.file_desc
where type uid = ProcEnv.uid
where type gid = ProcEnv.gid
structure IO : POSIX_IO
where type pid = Process.pid
where type file_desc = ProcEnv.file_desc
where type open_mode = FileSys.open_mode
structure SysDB : POSIX_SYS_DB
where type uid = ProcEnv.uid
where type gid = ProcEnv.gid
structure TTY : POSIX_TTY
where type pid = Process.pid
where type file_desc = ProcEnv.file_desc
```

#### Description

<span id="SIG:POSIX.Error:STR"></span>**`structure`**` Error `**`:`**` `[`POSIX_ERROR`](posix-error.md#POSIX_ERROR:SIG:SPEC)  
Errors and their descriptions.

<span id="SIG:POSIX.Signal:STR"></span>**`structure`**` Signal `**`:`**` `[`POSIX_SIGNAL`](posix-signal.md#POSIX_SIGNAL:SIG:SPEC)  
Signal values and their associated numbers.

<span id="SIG:POSIX.Process:STR"></span>**`structure`**` Process `**`:`**` `[`POSIX_PROCESS`](posix-process.md#POSIX_PROCESS:SIG:SPEC)  
Processes: fork, exec, wait, exit, kill, alarm, pause, sleep.

<span id="SIG:POSIX.ProcEnv:STR"></span>**`structure`**` ProcEnv `**`:`**` `[`POSIX_PROC_ENV`](posix-proc-env.md#POSIX_PROC_ENV:SIG:SPEC)  
User and group IDs, process times, environment, etc.

```repl
Posix.ProcEnv.getenv "HOME";;
```

<span id="SIG:POSIX.FileSys:STR"></span>**`structure`**` FileSys `**`:`**` `[`POSIX_FILE_SYS`](posix-file-sys.md#POSIX_FILE_SYS:SIG:SPEC)  
File system: open, chdir, chmod, directories, etc.

<span id="SIG:POSIX.IO:STR"></span>**`structure`**` IO `**`:`**` `[`POSIX_IO`](posix-io.md#POSIX_IO:SIG:SPEC)  
Input/output: read, write, pipe, dup, close, lock, seek, sync, etc.

<span id="SIG:POSIX.SysDB:STR"></span>**`structure`**` SysDB `**`:`**` `[`POSIX_SYS_DB`](posix-sys-db.md#POSIX_SYS_DB:SIG:SPEC)  
Password database, group database, etc.

<span id="SIG:POSIX.TTY:STR"></span>**`structure`**` TTY `**`:`**` `[`POSIX_TTY`](posix-tty.md#POSIX_TTY:SIG:SPEC)  
Terminal (TTY) control: speed, attributes, drain, flush, etc.


#### See Also

> [`Posix.Error`](posix.md#SIG:POSIX.Error:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`Posix.ProcEnv`](posix.md#SIG:POSIX.ProcEnv:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC), [`Posix.Signal`](posix.md#SIG:POSIX.Signal:STR:SPEC), [`Posix.SysDB`](posix.md#SIG:POSIX.SysDB:STR:SPEC), [`Posix.TTY`](posix.md#SIG:POSIX.TTY:STR:SPEC)

#### Discussion

The [`Posix`](posix.md#Posix:STR:SPEC) structure and signatures are optional as a group; _i.e._, they are either all present or all absent. Furthermore, if they are present, then the [`SysWord`](word.md#SysWord:STR:SPEC) structure must also be provided by the implementation, but note that an implementation may provide the [`SysWord`](word.md#SysWord:STR:SPEC) structure without providing the [`Posix`](posix.md#Posix:STR:SPEC) structure.

Many functions in the [`Posix`](posix.md#Posix:STR:SPEC) structure can raise [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) for many reasons. The description of an individual function will usually not describe all of the causes for the exception, or all of the system errors (see [`OS.syserror`](os.md#SIG:OS.syserror:TY:SPEC) and [`Posix.Error.syserror`](posix-error.md#SIG:POSIX_ERROR.syserror:TY:SPEC)) carried by the exception. The programmer will need to consult more detailed POSIX documentation.
