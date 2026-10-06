# <span id="section:0"></span>The `Unix` structure

---

#### Synopsis

<span id="UNIX:SIG:SPEC"></span>
<span id="Unix:STR:SPEC"></span>

```sml
signature UNIX (* OPTIONAL *)
structure Unix :> UNIX (* OPTIONAL *)
```

The `Unix` structure provides several high-level functions for creating and communicating with separate processes, in analogy with the `popen` interface provided in the Unix operating system. This module provides a more flexible interface than that provided by the [`OS.Process.system`](os-process.md#SIG:OS_PROCESS.system:VAL:SPEC) function. Using this module, a program can invoke a separate process and obtain input and output streams connected to the standard output and input streams, respectively, of the other process.

---

#### Interface

<span id="SIG:UNIX.proc:TY:SPEC"></span>
<span id="SIG:UNIX.signal:TY:SPEC"></span>
<span id="SIG:UNIX.exit_status:TY:SPEC"></span>
<span id="SIG:UNIX.W_EXITED:TY:SPEC"></span>
<span id="SIG:UNIX.W_EXITSTATUS:TY:SPEC"></span>
<span id="SIG:UNIX.W_SIGNALED:TY:SPEC"></span>
<span id="SIG:UNIX.W_STOPPED:TY:SPEC"></span>
<span id="SIG:UNIX.fromStatus:VAL:SPEC"></span>
<span id="SIG:UNIX.executeInEnv:VAL:SPEC"></span>
<span id="SIG:UNIX.execute:VAL:SPEC"></span>
<span id="SIG:UNIX.textInstreamOf:VAL:SPEC"></span>
<span id="SIG:UNIX.binInstreamOf:VAL:SPEC"></span>
<span id="SIG:UNIX.textOutstreamOf:VAL:SPEC"></span>
<span id="SIG:UNIX.binOutstreamOf:VAL:SPEC"></span>
<span id="SIG:UNIX.streamsOf:VAL:SPEC"></span>
<span id="SIG:UNIX.reap:VAL:SPEC"></span>
<span id="SIG:UNIX.kill:VAL:SPEC"></span>
<span id="SIG:UNIX.exit:VAL:SPEC"></span>

```sml
type ('a,'b) proc
type signal
datatype exit_status
= W_EXITED
| W_EXITSTATUS of Word8.word
| W_SIGNALED of signal
| W_STOPPED of signal
val fromStatus : OS.Process.status -> exit_status
val executeInEnv : string * string list * string list -> ('a, 'b) proc
val execute : string * string list -> ('a, 'b) proc
val textInstreamOf : (TextIO.instream, 'a) proc -> TextIO.instream
val binInstreamOf : (BinIO.instream, 'a) proc -> BinIO.instream
val textOutstreamOf : ('a, TextIO.outstream) proc -> TextIO.outstream
val binOutstreamOf : ('a, BinIO.outstream) proc -> BinIO.outstream
val streamsOf : (TextIO.instream, TextIO.outstream) proc -> TextIO.instream * TextIO.outstream
val reap : ('a, 'b) proc -> OS.Process.status
val kill : ('a, 'b) proc * signal -> unit
val exit : Word8.word -> 'a
```

#### Description

<span id="SIG:UNIX.proc:TY"></span>**`type`**` (`_`'a`_`,`_`'b`_`) proc`  
A type representing a handle for an operating system process.

<span id="SIG:UNIX.signal:TY"></span>**`type`**` signal`  
A Unix-like signal which can be sent to another process. Note that signal values must be obtained from some other structure. For example, an implementation providing the [`Posix`](posix.md#Posix:STR:SPEC) module would probably equate the [`signal`](unix.md#SIG:UNIX.signal:TY:SPEC) and [`Posix.Signal.signal`](posix-signal.md#SIG:POSIX_SIGNAL.signal:TY:SPEC) types.

<span id="SIG:UNIX.exit_status:TY"></span>**`datatype`**` exit_status`
`  = W_EXITED`
`  | W_EXITSTATUS `**`of`**` Word8.word`
`  | W_SIGNALED `**`of`**` signal`
`  | W_STOPPED `**`of`**` signal`  
These values represent the ways in which a Unix process might stop. They correspond to, respectively, successful termination, termination with the given exit value, termination upon receipt of the given signal, and stopping upon receipt of the given signal. The value carried by [`W_EXITSTATUS`](unix.md#SIG:UNIX.exit_status:TY:SPEC) will be non-zero.

If an implementation provides both the [`Posix`](posix.md#Posix:STR:SPEC) and [`Unix`](unix.md#Unix:STR:SPEC) structures, then [`Posix.Process.exit_status`](posix-process.md#SIG:POSIX_PROCESS.exit_status:TY:SPEC) and [`exit_status`](unix.md#SIG:UNIX.exit_status:TY:SPEC) must be the same type.

<span id="SIG:UNIX.fromStatus:VAL"></span>

### `fromStatus`

```sml
val fromStatus : OS.Process.status -> exit_status
```

`fromStatus ``sts`` `  
returns a concrete view of the given status.



```repl
Unix.fromStatus OS.Process.success;; (* W_EXITED *)
```

<span id="SIG:UNIX.executeInEnv:VAL"></span>

### `executeInEnv`

```sml
val executeInEnv : string * string list * string list -> ('a, 'b) proc
```

`executeInEnv (``cmd``, ``args``, ``env``) `  
asks the operating system to execute the program named by the string `cmd` with the argument list `args` and the environment `env`. The program is run as a child process of the calling program; the return value of this function is an abstract [`proc`](unix.md#SIG:UNIX.proc:TY:SPEC) value naming the child process. Strings in the `env` list typically have the form `"name=value"` (see [`OS.Process.getEnv`](os-process.md#SIG:OS_PROCESS.getEnv:VAL:SPEC)).

The `executeInEnv` function raises the [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if it fails. Reasons for failure include insufficient memory, too many processes, and the case where `cmd` does not name an executable file. If the child process fails to execute the command (_i.e._, the `execve` call fails), then it should exit with a status code of 126.



```repl
Unix.reap (Unix.executeInEnv ("/bin/true", ["true"], []));; (* OS.Process.success *)
```

<span id="SIG:UNIX.execute:VAL"></span>

### `execute`

```sml
val execute : string * string list -> ('a, 'b) proc
```

`execute (``cmd``, ``args``) `  
asks the operating system to execute the program named by the string `cmd` with the argument list `args`. The program is run as a child process of the calling program and it inherits the calling process's environment; the return value of this function is an abstract [`proc`](unix.md#SIG:UNIX.proc:TY:SPEC) value naming the child process. The failure semantics of this function are the same as for [`executeInEnv`](unix.md#SIG:UNIX.executeInEnv:VAL:SPEC).

For implementations providing the [`Posix`](posix.md#Posix:STR:SPEC) modules, this function is equivalent to

          fun execute (cmd, args) =
            executeInEnv (`cmd`, `args`, [Posix.ProcEnv.environ](posix-proc-env.md#SIG:POSIX_PROC_ENV.environ:VAL:SPEC) ())




```repl
Unix.reap (Unix.execute ("/bin/true", ["true"]));; (* OS.Process.success *)
```

<span id="SIG:UNIX.textInstreamOf:VAL"></span>

### `textInstreamOf`

```sml
val textInstreamOf : (TextIO.instream, 'b) proc -> TextIO.instream
```

### `binInstreamOf`

```sml
val binInstreamOf : (BinIO.instream, 'b) proc -> BinIO.instream
```

`textInstreamOf ``pr`` `
` binInstreamOf ``pr`` `  
These return a text or binary [`instream`](imperative-io.md#SIG:IMPERATIVE_IO.instream:TY:SPEC) connected to the standard output stream of the process `pr`.

Note that multiple calls to these functions on the same [`proc`](unix.md#SIG:UNIX.proc:TY:SPEC) value will result in multiple streams that all share the same underlying open file descriptor, which can lead to unpredictable effects because of the state inherent in file descriptors.



```repl
let val p = Unix.execute ("/bin/echo", ["echo", "hello"]) val line = TextIO.inputLine (Unix.textInstreamOf p) in Unix.reap p; line end;; (* SOME "hello\n" *)
```

```repl
let val p = Unix.execute ("/bin/echo", ["echo", "hello"]) val bytes = BinIO.inputAll (Unix.binInstreamOf p) in Unix.reap p; bytes end;; (* bytes for "hello\n" *)
```

<span id="SIG:UNIX.textOutstreamOf:VAL"></span>

### `textOutstreamOf`

```sml
val textOutstreamOf : ('a, TextIO.outstream) proc -> TextIO.outstream
```

### `binOutstreamOf`

```sml
val binOutstreamOf : ('a, BinIO.outstream) proc -> BinIO.outstream
```

`textOutstreamOf ``pr`` `
` binOutstreamOf ``pr`` `  
These return a text or binary [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) connected to the standard input stream of the process `pr`.

Note that multiple calls to these functions on the same [`proc`](unix.md#SIG:UNIX.proc:TY:SPEC) value will result in multiple streams that all share the same underlying open file descriptor, which can lead to unpredictable effects due to buffering.



```repl
let val p = Unix.execute ("/bin/cat", ["cat"]) val os = Unix.textOutstreamOf p in TextIO.output (os, "hello\n"); TextIO.closeOut os; let val line = TextIO.inputLine (Unix.textInstreamOf p) in Unix.reap p; line end end;; (* SOME "hello\n" *)
```

```repl
let val p = Unix.execute ("/bin/cat", ["cat"]) val os = Unix.binOutstreamOf p in BinIO.output (os, Word8Vector.fromList [0w104, 0w105]); BinIO.closeOut os; let val bytes = BinIO.inputAll (Unix.binInstreamOf p) in Unix.reap p; bytes end end;; (* bytes for "hi" *)
```

<span id="SIG:UNIX.streamsOf:VAL"></span>

### `streamsOf`

```sml
val streamsOf : (TextIO.instream, TextIO.outstream) proc -> TextIO.instream * TextIO.outstream
```

`streamsOf ``pr`` `  
returns a pair of input and output text streams associated with `pr`. This function is equivalent to `(textInstream ``pr``, textOutstream ``pr``)` and is provided for backward compatibility.



```repl
let val p = Unix.execute ("/bin/echo", ["echo", "hello"]) val (ins, outs) = Unix.streamsOf p val line = TextIO.inputLine ins in Unix.reap p; line end;; (* SOME "hello\n" *)
```

<span id="SIG:UNIX.reap:VAL"></span>

### `reap`

```sml
val reap : ('a, 'b) proc -> OS.Process.status
```

`reap ``pr`` `  
closes the input and output streams associated with `pr`, and then suspends the current process until the system process corresponding to `pr` terminates. It returns the exit status given by `pr` when it terminated. If `reap` is applied again to `pr`, it should immediately return the previous exit status.

> **Implementation note:**
>
> Typically, one cannot rely on the underlying operating system to provide the exit status of a terminated process after it has done so once. Thus, the exit status probably needs to be cached. Also note that `reap` should not return until the process being monitored has terminated. In particular, implementations should be careful not to return if the process has only been suspended.




```repl
Unix.reap (Unix.execute ("/bin/true", ["true"]));; (* OS.Process.success *)
```

<span id="SIG:UNIX.kill:VAL"></span>

### `kill`

```sml
val kill : ('a, 'b) proc * signal -> unit
```

`kill (``pr``,``s``) `  
sends the signal `s` to the process `pr`.



```repl
let val p = Unix.execute ("/bin/sleep", ["sleep", "30"]) in Unix.kill (p, Posix.Signal.term); Unix.reap p end;; (* child terminated by signal *)
```

<span id="SIG:UNIX.exit:VAL"></span>

### `exit`

```sml
val exit : Word8.word -> 'a
```

`exit ``st`` `  
executes all actions registered with [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC), flushes and closes all I/O streams opened using the Library, then terminates the SML process with termination status `st`.



```repl
val terminate = Unix.exit;; (* function that terminates the SML process when called *)
```

#### Examples

```repl
Unix.reap (Unix.execute ("/bin/true", ["true"]));; (* OS.Process.success *)
```

#### See Also

> [`BinIO`](bin-io.md#BinIO:STR:SPEC), [`OS.Process`](os.md#SIG:OS.Process:STR:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.ProcEnv`](posix.md#SIG:POSIX.ProcEnv:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC), [`Posix.Signal`](posix.md#SIG:POSIX.Signal:STR:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC)

#### Discussion

Note that the interpretation of the string `cmd` in the [`execute`](unix.md#SIG:UNIX.execute:VAL:SPEC) and [`executeInEnv`](unix.md#SIG:UNIX.executeInEnv:VAL:SPEC) functions depends very much on the underlying operating system. Typically, the `cmd` argument will be a full pathname.

The semantics of Unix necessitates that processes that have terminated need to be reaped. If this is not done, information concerning the dead process continues to reside in system tables. Thus, a program using [`execute`](unix.md#SIG:UNIX.execute:VAL:SPEC) or [`executeInEnv`](unix.md#SIG:UNIX.executeInEnv:VAL:SPEC) should invoke [`reap`](unix.md#SIG:UNIX.reap:VAL:SPEC) on any subprocess it creates.

> **Implementation note:**
>
> Although the flavor of this module is heavily influenced by Unix, and the module is simple to implement given the [`Posix`](posix.md#Posix:STR:SPEC) subsystem, the functions are specified at a sufficiently high-level that implementations, including non-Unix ones, could provide this module without having to supply all of the [`Posix`](posix.md#Posix:STR:SPEC) modules.
