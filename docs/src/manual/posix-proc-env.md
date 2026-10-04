# <span id="section:0"></span>The `Posix.ProcEnv` structure

---

#### Synopsis

<span id="POSIX_PROC_ENV:SIG:SPEC"></span>
<span id="ProcEnv:STR:SPEC"></span>

```sml
signature POSIX_PROC_ENV
structure ProcEnv : POSIX_PROC_ENV
```

The structure `Posix.ProcEnv` specifies functions, as described in Section 4 of the POSIX standard 1003.1,1996**\[CITE\]**, which provide primitive POSIX access to the process environment.

---

#### Interface

<span id="SIG:POSIX_PROC_ENV.pid:TY:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.uid:TY:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.gid:TY:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.file_desc:TY:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.uidToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.wordToUid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.gidToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.wordToGid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getpid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getppid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getuid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.geteuid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getgid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getegid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.setuid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.setgid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getgroups:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getlogin:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getpgrp:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.setsid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.setpgid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.uname:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.time:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.times:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.getenv:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.environ:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.ctermid:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.ttyname:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.isatty:VAL:SPEC"></span>
<span id="SIG:POSIX_PROC_ENV.sysconf:VAL:SPEC"></span>

```sml
eqtype pid
eqtype uid
eqtype gid
eqtype file_desc
val uidToWord : uid -> SysWord.word
val wordToUid : SysWord.word -> uid
val gidToWord : gid -> SysWord.word
val wordToGid : SysWord.word -> gid
val getpid : unit -> pid
val getppid : unit -> pid
val getuid : unit -> uid
val geteuid : unit -> uid
val getgid : unit -> gid
val getegid : unit -> gid
val setuid : uid -> unit
val setgid : gid -> unit
val getgroups : unit -> gid list
val getlogin : unit -> string
val getpgrp : unit -> pid
val setsid : unit -> pid
val setpgid : {pid : pid option, pgid : pid option} -> unit
val uname : unit -> (string * string) list
val time : unit -> Time.time
val times : unit -> {
elapsed : Time.time,
utime : Time.time,
stime : Time.time,
cutime : Time.time,
cstime : Time.time
}
val getenv : string -> string option
val environ : unit -> string list
val ctermid : unit -> string
val ttyname : file_desc -> string
val isatty : file_desc -> bool
val sysconf : string -> SysWord.word
```

#### Description

<span id="SIG:POSIX_PROC_ENV.pid:TY"></span>**`eqtype`**` pid`  
A process ID, used as an identifier for an operating system process.

<span id="SIG:POSIX_PROC_ENV.uid:TY"></span>**`eqtype`**` uid`  
User identifier.

<span id="SIG:POSIX_PROC_ENV.gid:TY"></span>**`eqtype`**` gid`  
Group identifier.

<span id="SIG:POSIX_PROC_ENV.file_desc:TY"></span>**`eqtype`**` file_desc`  
Open file descriptor.

<span id="SIG:POSIX_PROC_ENV.uidToWord:VAL"></span>**`val`**` uidToWord `**`:`**` uid `**`->`**` SysWord.word`
**`val`**` wordToUid `**`:`**` SysWord.word `**`->`**` uid`  
These functions convert between an abstract user ID and an underlying unique unsigned integer. Note that [`wordToUid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.wordToUid:VAL:SPEC) does not ensure that it returns a valid [`uid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.uid:TY:SPEC).

<span id="SIG:POSIX_PROC_ENV.gidToWord:VAL"></span>**`val`**` gidToWord `**`:`**` gid `**`->`**` SysWord.word`
**`val`**` wordToGid `**`:`**` SysWord.word `**`->`**` gid`  
These convert between an abstract group ID and an underlying unique unsigned integer. Note that [`wordToGid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.wordToGid:VAL:SPEC) does not ensure that it returns a valid [`gid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.gid:TY:SPEC).

<span id="SIG:POSIX_PROC_ENV.getpid:VAL"></span>**`val`**` getpid `**`:`**` unit `**`->`**` pid`
**`val`**` getppid `**`:`**` unit `**`->`**` pid`  
The process ID and the parent process ID, respectively, of the calling process.

<span id="SIG:POSIX_PROC_ENV.getuid:VAL"></span>**`val`**` getuid `**`:`**` unit `**`->`**` uid`
**`val`**` geteuid `**`:`**` unit `**`->`**` uid`  
The real and effective user IDs, respectively, of the calling process.

<span id="SIG:POSIX_PROC_ENV.getgid:VAL"></span>**`val`**` getgid `**`:`**` unit `**`->`**` gid`
**`val`**` getegid `**`:`**` unit `**`->`**` gid`  
The real and effective group IDs, respectively, of the calling process.

<span id="SIG:POSIX_PROC_ENV.setuid:VAL"></span>
`setuid ``u`` `  
sets the real user ID and effective user ID to `u`.

<span id="SIG:POSIX_PROC_ENV.setgid:VAL"></span>
`setgid ``g`` `  
sets the real group ID and effective group ID to `g`.

<span id="SIG:POSIX_PROC_ENV.getgroups:VAL"></span>**`val`**` getgroups `**`:`**` unit `**`->`**` gid list`  
The list of supplementary group IDs of the calling process.

<span id="SIG:POSIX_PROC_ENV.getlogin:VAL"></span>**`val`**` getlogin `**`:`**` unit `**`->`**` string`  
The user name associated with the calling process, _i.e._, the login name associated with the calling process.

<span id="SIG:POSIX_PROC_ENV.getpgrp:VAL"></span>**`val`**` getpgrp `**`:`**` unit `**`->`**` pid`  
The process group ID of the calling process.

<span id="SIG:POSIX_PROC_ENV.setsid:VAL"></span>**`val`**` setsid `**`:`**` unit `**`->`**` pid`  
This function creates a new session if the calling process is not a process group leader, and returns the process group ID of the calling process.

<span id="SIG:POSIX_PROC_ENV.setpgid:VAL"></span>
`setpgid (`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``pid``, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``pgid``) `
` setpgid (`[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``pgid``) `
` setpgid (`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``pid``, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `
` setpgid (`[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `  
The first (second) usage sets the process group ID of the process with process ID `pid` (of the calling process, respectively) to `pgid`. In the third (fourth) usage, the process with process ID `pid` (the calling process, respectively) becomes a process group leader.

<span id="SIG:POSIX_PROC_ENV.uname:VAL"></span>**`val`**` uname `**`:`**` unit `**`->`**` (string `**`*`**` string) list`  
A list of name-value pairs including, at least, the names: `"sysname"`, `"nodename"`, `"release"`, `"version"`, and `"machine"`. (A POSIX implementation may provide additional values beyond this set.) The respective values are strings that describe the named system component.

<span id="SIG:POSIX_PROC_ENV.time:VAL"></span>**`val`**` time `**`:`**` unit `**`->`**` Time.time`  
The elapsed wall time since the Epoch.

<span id="SIG:POSIX_PROC_ENV.times:VAL"></span>**`val`**` times `**`:`**` unit`
`              `**`->`**` {`
`                elapsed `**`:`**` Time.time,`
`                utime `**`:`**` Time.time,`
`                stime `**`:`**` Time.time,`
`                cutime `**`:`**` Time.time,`
`                cstime `**`:`**` Time.time`
`              }`  
A record containing the wall time (`elapsed`), user time (`utime`), system time (`stime`), user CPU time of terminated child processes (`cutime`), and system CPU time of terminated child processes (`cstime`), for the calling process.

<span id="SIG:POSIX_PROC_ENV.getenv:VAL"></span>
`getenv ``name`` `  
searches the environment list for a string of the form `name=value` and returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(value)` if `name` is present; it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `name` is not present. This is equivalent to [`OS.Process.getEnv`](os-process.md#SIG:OS_PROCESS.getEnv:VAL:SPEC).

<span id="SIG:POSIX_PROC_ENV.environ:VAL"></span>**`val`**` environ `**`:`**` unit `**`->`**` string list`  
The environment of the calling process as a list of strings.

<span id="SIG:POSIX_PROC_ENV.ctermid:VAL"></span>**`val`**` ctermid `**`:`**` unit `**`->`**` string`  
A string that represents the pathname of the controlling terminal for the calling process.

<span id="SIG:POSIX_PROC_ENV.ttyname:VAL"></span>
`ttyname ``fd`` `  
produces a string that represents the pathname of the terminal associated with file descriptor `fd`. It raises [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `fd` does not denote a valid terminal device.

<span id="SIG:POSIX_PROC_ENV.isatty:VAL"></span>
`isatty ``fd`` `  
returns `true` if `fd` is a valid file descriptor associated with a terminal. Note that [`isatty`](posix-proc-env.md#SIG:POSIX_PROC_ENV.isatty:VAL:SPEC) will return `false` if `fd` is a bad file descriptor.

<span id="SIG:POSIX_PROC_ENV.sysconf:VAL"></span>
`sysconf ``s`` `  
returns the integer value for the POSIX configurable system variable `s`. It raises [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `s` does not denote a supported POSIX system variable.

The properties required by POSIX are described below. This list is a minimal set required for POSIX compliance, and an implementation may extend it with additional properties.

`"ARG_MAX"`  
Maximum length of arguments, in bytes, for the functions [`Posix.Process.exec`](posix-process.md#SIG:POSIX_PROCESS.exec:VAL:SPEC), [`Posix.Process.exece`](posix-process.md#SIG:POSIX_PROCESS.exece:VAL:SPEC), and [`Posix.Process.execp`](posix-process.md#SIG:POSIX_PROCESS.execp:VAL:SPEC). This also applies to environment data.

`"CHILD_MAX"`  
Maximum number of concurrent processes associated with a real user ID.

`"CLK_TCK"`  
Number of clock ticks per second.

`"NGROUPS_MAX"`  
Maximum number of supplementary group IDs associated with a process, in addition to the effective group ID.

`"OPEN_MAX"`  
Maximum number of files that one process can have open concurrently.

`"STREAM_MAX"`  
Maximum number of streams that one process can have open concurrently.

`"TZNAME_MAX"`  
Maximum number bytes allowed for a time zone name.

`"JOB_CONTROL"`  
Non-zero if the implementation supports job control.

`"SAVED_IDS"`  
Non-zero if each process has a saved set-user-ID and and saved set-group-ID.

`"VERSION"`  
A version number.

Consult Section 4.8 of POSIX standard 1003.1,1996 **\[CITE\]** for additional information. Note that a property in SML has the same name as the property in C, but without the prefix `"_SC_"`.

#### Examples

```repl
Posix.ProcEnv.getenv "HOME";;
```

#### See Also

> [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.ProcEnv`](posix.md#SIG:POSIX.ProcEnv:STR:SPEC), [`Time`](time.md#Time:STR:SPEC)
