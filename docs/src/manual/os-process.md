# <span id="section:0"></span>The `OS.Process` structure

---

#### Synopsis

<span id="OS_PROCESS:SIG:SPEC"></span>
<span id="Process:STR:SPEC"></span>

```sml
signature OS_PROCESS
structure Process : OS_PROCESS
```

The `OS.Process` structure provides functions for manipulating processes in an operating system independent manner. For discussion of additional relations between this structure and other structures, see Section 11.1.

---

#### Interface

<span id="SIG:OS_PROCESS.status:TY:SPEC"></span>
<span id="SIG:OS_PROCESS.success:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.failure:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.isSuccess:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.system:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.atExit:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.exit:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.terminate:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.getEnv:VAL:SPEC"></span>
<span id="SIG:OS_PROCESS.sleep:VAL:SPEC"></span>

```sml
type status
val success : status
val failure : status
val isSuccess : status -> bool
val system : string -> status
val atExit : (unit -> unit) -> unit
val exit : status -> 'a
val terminate : status -> 'a
val getEnv : string -> string option
val sleep : Time.time -> unit
```

#### Description

<span id="SIG:OS_PROCESS.status:TY"></span>**`type`**` status`  
The `status` type represents various termination conditions for processes. On POSIX-based systems, [`status`](os-process.md#SIG:OS_PROCESS.status:TY:SPEC) will typically be an integral value.

<span id="SIG:OS_PROCESS.success:VAL"></span>

### `success`

```sml
val success : status
```
The unique [`status`](os-process.md#SIG:OS_PROCESS.status:TY:SPEC) value that signifies successful termination of a process.

```repl
OS.Process.success;; (* success status *)
```
<span id="SIG:OS_PROCESS.failure:VAL"></span>

### `failure`

```sml
val failure : status
```
A [`status`](os-process.md#SIG:OS_PROCESS.status:TY:SPEC) value that signifies an error during execution of a process. Note that, in contrast to the success value, there may be other failure values.

```repl
OS.Process.failure;; (* failure status *)
```
<span id="SIG:OS_PROCESS.isSuccess:VAL"></span>

### `isSuccess`

```sml
val isSuccess : status -> bool
```
`isSuccess ``sts`` `  
returns `true` if the status denotes success.

> **Implementation note:**
>
> On implementations supporting the [`Unix`](unix.md#Unix:STR:SPEC) structure, this function returns `true` only when [`Unix.fromStatus`](unix.md#SIG:UNIX.fromStatus:VAL:SPEC)` ``sts` is [`Unix.W_EXITED`](unix.md#SIG:UNIX.exit_status:TY:SPEC). The analogous condition also holds for implementations providing the [`Posix`](posix.md#Posix:STR:SPEC) structure.

```repl
OS.Process.isSuccess OS.Process.success;; (* true *)
```
<span id="SIG:OS_PROCESS.system:VAL"></span>

### `system`

```sml
val system : string -> status
```
`system ``cmd`` `  
passes the command string `cmd` to the operating system's default shell to execute. It returns the termination status resulting from executing the command. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if the command cannot be executed.

Note that, although this function is independent of the operating system, the interpretation of the string `cmd` depends very much on the underlying operating system and shell. On Unix systems, the default shell is "/bin/sh"; on Microsoft Windows systems, the default shell is the Microsoft Windows shell; on MacOS systems, the command is compiled and executed as an Apple script.

```repl
OS.Process.isSuccess (OS.Process.system "echo Nassau");; (* true when the command succeeds *)
```
<span id="SIG:OS_PROCESS.atExit:VAL"></span>

### `atExit`

```sml
val atExit : (unit -> unit) -> unit
```
`atExit ``f`` `  
registers an action `f` to be executed when the current SML program calls [`exit`](os-process.md#SIG:OS_PROCESS.exit:VAL:SPEC). Actions will be executed in the reverse order of registration.

Exceptions raised when `f` is invoked that escape it are trapped and ignored. Calls in `f` to [`atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC) are ignored. Calls to [`exit`](os-process.md#SIG:OS_PROCESS.exit:VAL:SPEC) do not return, but should cause the remainder of the functions registered with [`atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC) to be executed. Calls to [`terminate`](os-process.md#SIG:OS_PROCESS.terminate:VAL:SPEC) (or similar functions such as [`Posix.Process.exit`](posix-process.md#SIG:POSIX_PROCESS.exit:VAL:SPEC)) will terminate the process immediately.

```repl
OS.Process.atExit (fn () => print "leaving
");; (* () *)
```
<span id="SIG:OS_PROCESS.exit:VAL"></span>

### `exit`

```sml
val exit : status -> 'a
```
`exit ``st`` `  
executes all actions registered with [`atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC), flushes and closes all I/O streams opened using the Library, then terminates the SML process with termination status `st`.

> **Implementation note:**
>
> If the argument to `exit` comes from `system` or some other function (such as [`Unix.reap`](unix.md#SIG:UNIX.reap:VAL:SPEC)) returning a [`status`](os-process.md#SIG:OS_PROCESS.status:TY:SPEC) value, then the implementation should attempt to preserve the meaning of the exit code from the subprocess. For example, on a POSIX system, if `Posix.Process.fromStatus ``st` yields `Posix.Process.W_EXITSTATUS ``v`, then `v` should be passed to `Posix.Process.exit` after all necessary cleanup is done.
>
> If `st` does not connote an exit value, `exit` should act as though called with [`failure`](os-process.md#SIG:OS_PROCESS.failure:VAL:SPEC). For example, on a POSIX system, this would occur if `Posix.Process.fromStatus ``st` is `Posix.Process.W_SIGNALED` or `Posix.Process.W_STOPPED`.

```repl
OS.Process.exit;; (* status -> 'a; calling it exits the process *)
```
<span id="SIG:OS_PROCESS.terminate:VAL"></span>

### `terminate`

```sml
val terminate : status -> 'a
```
`terminate ``st`` `  
terminates the SML process with termination status `st`, without executing the actions registered with [`atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC) or flushing open I/O streams.

```repl
OS.Process.terminate;; (* status -> 'a; calling it terminates immediately *)
```
<span id="SIG:OS_PROCESS.getEnv:VAL"></span>

### `getEnv`

```sml
val getEnv : string -> string option
```
`getEnv ``s`` `  
returns the value of the environment variable `s`, if defined. Otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

An environment is associated with each SML process, modeled as a list of pairs of strings, corresponding to name-value pairs. (The way the environment is established depends on the host operating system.) The [`getEnv`](os-process.md#SIG:OS_PROCESS.getEnv:VAL:SPEC) function scans the environment for a pair whose first component equals `s`. If successful, it returns the second component.

```repl
OS.Process.getEnv "PATH";; (* SOME path, or NONE *)
```
<span id="SIG:OS_PROCESS.sleep:VAL"></span>

### `sleep`

```sml
val sleep : Time.time -> unit
```
`sleep ``t`` `  
suspends the calling process for the time specified by `t`. If `t` is zero or negative, then the calling process does not sleep, but returns immediately. No exception is raised.

#### See Also

> [`OS`](os.md#OS:STR:SPEC), [`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC), [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC), [`Posix.ProcEnv`](posix.md#SIG:POSIX.ProcEnv:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

```repl
OS.Process.sleep Time.zeroTime;; (* returns immediately *)
```
