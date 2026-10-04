# <span id="section:0"></span>The `Posix.Process` structure

---

#### Synopsis

<span id="POSIX_PROCESS:SIG:SPEC"></span>
<span id="Process:STR:SPEC"></span>

```sml
signature POSIX_PROCESS
structure Process : POSIX_PROCESS
```

The structure `Posix.Process` describes the primitive POSIX operations dealing with processes, as described in Section 3 of the POSIX standard 1003.1,1996**\[CITE\]**.

---

#### Interface

<span id="SIG:POSIX_PROCESS.signal:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.pid:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.wordToPid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.pidToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.fork:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.exec:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.exece:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.execp:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.waitpid_arg:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_ANY_CHILD:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_CHILD:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_SAME_GROUP:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_GROUP:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.exit_status:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_EXITED:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_EXITSTATUS:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_SIGNALED:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.W_STOPPED:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.fromStatus:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.untraced:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.wait:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.waitpid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.waitpid_nh:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.exit:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.killpid_arg:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.K_PROC:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.K_SAME_GROUP:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.K_GROUP:TY:SPEC"></span>
<span id="SIG:POSIX_PROCESS.kill:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.alarm:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.pause:VAL:SPEC"></span>
<span id="SIG:POSIX_PROCESS.sleep:VAL:SPEC"></span>

```sml
eqtype signal
eqtype pid
val wordToPid : SysWord.word -> pid
val pidToWord : pid -> SysWord.word
val fork : unit -> pid option
val exec : string * string list -> 'a
val exece : string * string list * string list -> 'a
val execp : string * string list -> 'a
datatype waitpid_arg
= W_ANY_CHILD
| W_CHILD of pid
| W_SAME_GROUP
| W_GROUP of pid
datatype exit_status
= W_EXITED
| W_EXITSTATUS of Word8.word
| W_SIGNALED of signal
| W_STOPPED of signal
val fromStatus : OS.Process.status -> exit_status
structure W : sig
include BIT_FLAGS
val untraced : flags
end
val wait : unit -> pid * exit_status
val waitpid : waitpid_arg * W.flags list -> pid * exit_status
val waitpid_nh : waitpid_arg * W.flags list -> (pid * exit_status) option
val exit : Word8.word -> 'a
datatype killpid_arg
= K_PROC of pid
| K_SAME_GROUP
| K_GROUP of pid
val kill : killpid_arg * signal -> unit
val alarm : Time.time -> Time.time
val pause : unit -> unit
val sleep : Time.time -> Time.time
```

#### Description

<span id="SIG:POSIX_PROCESS.signal:TY"></span>**`eqtype`**` signal`  
A POSIX signal, an asynchronous notification of an event.

<span id="SIG:POSIX_PROCESS.pid:TY"></span>**`eqtype`**` pid`  
A process ID, used as an identifier for an operating system process.

<span id="SIG:POSIX_PROCESS.wordToPid:VAL"></span>**`val`**` wordToPid `**`:`**` SysWord.word `**`->`**` pid`
**`val`**` pidToWord `**`:`**` pid `**`->`**` SysWord.word`  
These functions convert between a process ID and the integer representation used by the operating system. Note that there is no validation that a [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC) value generated using [`wordToPid`](posix-process.md#SIG:POSIX_PROCESS.wordToPid:VAL:SPEC) is legal on the given system or that it corresponds to a currently running process.

<span id="SIG:POSIX_PROCESS.fork:VAL"></span>**`val`**` fork `**`:`**` unit `**`->`**` pid option`  
This creates a new process. The new child process is a copy of the calling parent process. After execution of [`fork`](posix-process.md#SIG:POSIX_PROCESS.fork:VAL:SPEC), both the parent and child process execute independently, but share various system resources. Upon successful completion, [`fork`](posix-process.md#SIG:POSIX_PROCESS.fork:VAL:SPEC) returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) in the child process, and the [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC) of the child in the parent process. It raises [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) on failure.

<span id="SIG:POSIX_PROCESS.exec:VAL"></span>
`exec (``path``, ``args``) `
`exece (``path``, ``args``, ``env``)`
`execp (``file``, ``args``)`  
These functions replace the current process image with a new process image. There is no return from a successful call, as the calling process image is overlaid by the new process image. In the first two forms, the `path` argument specifies the pathname of the executable file. In the last form, if `file` contains a slash character, it is treated as the pathname for the executable file; otherwise, an executable file with name `file` is searched for in the directories specified by the environment variable **PATH**.

Normally, the new image is given the same environment as the calling program. The `env` argument in [`exece`](posix-process.md#SIG:POSIX_PROCESS.exece:VAL:SPEC) allows the program to specify a new environment.

The `args` argument is a list of string arguments to be passed to the new program. By convention, the first item in `args` is some form of the filename of the new program, usually the last arc in the path or filename.

<span id="SIG:POSIX_PROCESS.waitpid_arg:TY"></span>
**`datatype`**` waitpid_arg`  
`= W_ANY_CHILD`  
Any child process

`| W_CHILD `**`of`**` pid`  
The child process with the given [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC)

`| W_SAME_GROUP`  
Any child process in the same process group as the calling process

`| W_GROUP `**`of`**` pid`  
Any child process whose process group ID is given by [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC).

<span id="SIG:POSIX_PROCESS.exit_status:TY"></span>**`datatype`**` exit_status`
`  = W_EXITED`
`  | W_EXITSTATUS `**`of`**` Word8.word`
`  | W_SIGNALED `**`of`**` signal`
`  | W_STOPPED `**`of`**` signal`  
These values represent the ways in which a process might stop. They correspond to, respectively, terminate successfully, terminate with the given value, terminate upon receipt of the given signal, and stop upon receipt of the given signal. The value carried by `W_EXITSTATUS` must never be zero.

If an implementation provides both the [`Posix`](posix.md#Posix:STR:SPEC) and [`Unix`](unix.md#Unix:STR:SPEC) structures, then the datatypes [`Posix.Process.exit_status`](posix-process.md#SIG:POSIX_PROCESS.exit_status:TY:SPEC) and [`Unix.exit_status`](unix.md#SIG:UNIX.exit_status:TY:SPEC) must be the same.

<span id="SIG:POSIX_PROCESS.fromStatus:VAL"></span>
`fromStatus ``sts`` `  
returns a concrete view of the given status.

<span id="SIG:POSIX_PROCESS.W:STR"></span>
**`structure`**` W`  

<span id="SIG:POSIX_PROCESS.W.untraced:VAL"></span>**`val`**` untraced `**`:`**` flags`  
In systems supporting job control, this flag requests the status of child processes that are stopped.

<span id="SIG:POSIX_PROCESS.wait:VAL"></span>**`val`**` wait `**`:`**` unit `**`->`**` pid `**`*`**` exit_status`  
This function allows a calling process to obtain status information on any of its child processes. Execution of [`wait`](posix-process.md#SIG:POSIX_PROCESS.wait:VAL:SPEC) suspends execution until status information on one of its child processes is available. If status information is available prior to the execution of [`wait`](posix-process.md#SIG:POSIX_PROCESS.wait:VAL:SPEC), return is immediate. [`wait`](posix-process.md#SIG:POSIX_PROCESS.wait:VAL:SPEC) returns the process ID of the child and its exit status.

<span id="SIG:POSIX_PROCESS.waitpid:VAL"></span>
`waitpid (``procs``, ``l``) `  
is identical to [`wait`](posix-process.md#SIG:POSIX_PROCESS.wait:VAL:SPEC) except that the status is reported only for child processes specified by `procs`. A set of flags `l` may be used to modify the behavior of [`waitpid`](posix-process.md#SIG:POSIX_PROCESS.waitpid:VAL:SPEC).

<span id="SIG:POSIX_PROCESS.waitpid_nh:VAL"></span>
`waitpid_nh (``procs``, ``l``) `  
is identical to [`waitpid`](posix-process.md#SIG:POSIX_PROCESS.waitpid:VAL:SPEC), except that the call does not suspend if status information for one of the children specified by `procs` is not immediately available.

> **Rationale:**
>
> In C, [`waitpid_nh`](posix-process.md#SIG:POSIX_PROCESS.waitpid_nh:VAL:SPEC) is handled by [`waitpid`](posix-process.md#SIG:POSIX_PROCESS.waitpid:VAL:SPEC), using an additional flag to indicate no hanging. In SML, the semantics of [`waitpid_nh`](posix-process.md#SIG:POSIX_PROCESS.waitpid_nh:VAL:SPEC) indicated a different return type from that of [`waitpid`](posix-process.md#SIG:POSIX_PROCESS.waitpid:VAL:SPEC), hence the split into two functions.


<span id="SIG:POSIX_PROCESS.exit:VAL"></span>
`exit ``i`` `  
terminates the calling process. If the parent process is executing a [`wait`](posix-process.md#SIG:POSIX_PROCESS.wait:VAL:SPEC) related call, the exit status `i` is made available to it. [`exit`](posix-process.md#SIG:POSIX_PROCESS.exit:VAL:SPEC) does not return to the caller.

Calling [`exit`](posix-process.md#SIG:POSIX_PROCESS.exit:VAL:SPEC) does not flush or close any open [`IO`](io.md#IO:STR:SPEC) streams, nor does it call [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC). It does close any open POSIX files, and performs the actions associated with the C version of [`exit`](posix-process.md#SIG:POSIX_PROCESS.exit:VAL:SPEC).

<span id="SIG:POSIX_PROCESS.killpid_arg:TY"></span>
**`datatype`**` killpid_arg`  
`= K_PROC `**`of`**` pid`  
The process with ID [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC).

`| K_SAME_GROUP`  
All processes in the same process group as the calling process.

`| K_GROUP `**`of`**` pid`  
All processes in the process group specified by [`pid`](posix-process.md#SIG:POSIX_PROCESS.pid:TY:SPEC).

<span id="SIG:POSIX_PROCESS.kill:VAL"></span>
`kill (``procs``, ``sig``) `  
sends the signal `sig` to the process or group of processes specified by `procs`.

<span id="SIG:POSIX_PROCESS.alarm:VAL"></span>
`alarm ``t`` `  
causes the system to send an alarm signal ([`alrm`](posix-signal.md#SIG:POSIX_SIGNAL.alrm:VAL:SPEC)) to the calling process after `t` seconds have elapsed. If there is a previous alarm request with time remaining, the [`alarm`](posix-process.md#SIG:POSIX_PROCESS.alarm:VAL:SPEC) function returns a nonzero value corresponding to the number of seconds remaining on the previous request. Zero time is returned if there are no outstanding calls.

<span id="SIG:POSIX_PROCESS.pause:VAL"></span>**`val`**` pause `**`:`**` unit `**`->`**` unit`  
This suspends the calling process until the delivery of a signal that is either caught or that terminates the process.

<span id="SIG:POSIX_PROCESS.sleep:VAL"></span>
`sleep ``t`` `  
causes the current process to be suspended from execution until either `t` seconds have elapsed, or until the receipt of a signal that is either caught or that terminates the process.

#### Examples

```repl
Posix.Process.pidToWord (Posix.ProcEnv.getpid ());;
```

#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`OS.Process`](os.md#SIG:OS.Process:STR:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Signal`](posix.md#SIG:POSIX.Signal:STR:SPEC)
