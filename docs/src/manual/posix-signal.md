# <span id="section:0"></span>The `Posix.Signal` structure

---

#### Synopsis

<span id="POSIX_SIGNAL:SIG:SPEC"></span>
<span id="Signal:STR:SPEC"></span>

```sml
signature POSIX_SIGNAL
structure Signal : POSIX_SIGNAL
```

The structure `Posix.Signal` defines the symbolic names of all the POSIX signals (see Section 3.3 of the POSIX standard 1003.1,1996**\[CITE\]**), and provides conversion functions between them and their underlying representations.

---

#### Interface

<span id="SIG:POSIX_SIGNAL.signal:TY:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.toWord:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.fromWord:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.abrt:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.alrm:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.bus:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.fpe:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.hup:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.ill:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.int:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.kill:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.pipe:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.quit:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.segv:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.term:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.usr1:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.usr2:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.chld:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.cont:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.stop:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.tstp:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.ttin:VAL:SPEC"></span>
<span id="SIG:POSIX_SIGNAL.ttou:VAL:SPEC"></span>

```sml
eqtype signal
val toWord : signal -> SysWord.word
val fromWord : SysWord.word -> signal
val abrt : signal
val alrm : signal
val bus : signal
val fpe : signal
val hup : signal
val ill : signal
val int : signal
val kill : signal
val pipe : signal
val quit : signal
val segv : signal
val term : signal
val usr1 : signal
val usr2 : signal
val chld : signal
val cont : signal
val stop : signal
val tstp : signal
val ttin : signal
val ttou : signal
```

#### Description

<span id="SIG:POSIX_SIGNAL.signal:TY"></span>**`eqtype`**` signal`  
A POSIX signal, an asynchronous notification of an event.

<span id="SIG:POSIX_SIGNAL.toWord:VAL"></span>

### `toWord`

```sml
val toWord : signal -> SysWord.word
```

```repl
Posix.Signal.toWord Posix.Signal.int;; (* signal number as a word *)
```
<span id="SIG:POSIX_SIGNAL.fromWord:VAL"></span>

### `fromWord`

```sml
val fromWord : SysWord.word -> signal
```

```repl
Posix.Signal.fromWord (Posix.Signal.toWord Posix.Signal.int);; (* Posix.Signal.int *)
```

These convert between a signal identifier and its underlying integer representation. Note that [`fromWord`](posix-signal.md#SIG:POSIX_SIGNAL.fromWord:VAL:SPEC) does not check that the result corresponds to a valid POSIX signal.

#### See Also

> [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

#### Discussion

The values defined in this structure represent the standard POSIX signals. The following table provides a brief description of their meanings.

---

**SML name**

**Description**

<span id="SIG:POSIX_SIGNAL.abrt:VAL"></span>

### `abrt`

```sml
val abrt : signal
```

```repl
Posix.Signal.abrt;; (* POSIX signal value *)
```

[`abrt`](posix-signal.md#SIG:POSIX_SIGNAL.abrt:VAL:SPEC)

End process (abort).

<span id="SIG:POSIX_SIGNAL.alrm:VAL"></span>

### `alrm`

```sml
val alrm : signal
```

```repl
Posix.Signal.alrm;; (* POSIX signal value *)
```

[`alrm`](posix-signal.md#SIG:POSIX_SIGNAL.alrm:VAL:SPEC)

Alarm clock.

<span id="SIG:POSIX_SIGNAL.bus:VAL"></span>

### `bus`

```sml
val bus : signal
```

```repl
Posix.Signal.bus;; (* POSIX signal value *)
```

[`bus`](posix-signal.md#SIG:POSIX_SIGNAL.bus:VAL:SPEC)

Bus error.

<span id="SIG:POSIX_SIGNAL.fpe:VAL"></span>

### `fpe`

```sml
val fpe : signal
```

```repl
Posix.Signal.fpe;; (* POSIX signal value *)
```

[`fpe`](posix-signal.md#SIG:POSIX_SIGNAL.fpe:VAL:SPEC)

Floating-point exception.

<span id="SIG:POSIX_SIGNAL.hup:VAL"></span>

### `hup`

```sml
val hup : signal
```

```repl
Posix.Signal.hup;; (* POSIX signal value *)
```

[`hup`](posix-signal.md#SIG:POSIX_SIGNAL.hup:VAL:SPEC)

Hangup.

<span id="SIG:POSIX_SIGNAL.ill:VAL"></span>

### `ill`

```sml
val ill : signal
```

```repl
Posix.Signal.ill;; (* POSIX signal value *)
```

[`ill`](posix-signal.md#SIG:POSIX_SIGNAL.ill:VAL:SPEC)

Illegal instruction.

<span id="SIG:POSIX_SIGNAL.int:VAL"></span>

### `int`

```sml
val int : signal
```

```repl
Posix.Signal.int;; (* POSIX signal value *)
```

[`int`](posix-signal.md#SIG:POSIX_SIGNAL.int:VAL:SPEC)

Interrupt.

<span id="SIG:POSIX_SIGNAL.kill:VAL"></span>

### `kill`

```sml
val kill : signal
```

```repl
Posix.Signal.kill;; (* POSIX signal value *)
```

[`kill`](posix-signal.md#SIG:POSIX_SIGNAL.kill:VAL:SPEC)

Kill. (It cannot be caught or ignored.)

<span id="SIG:POSIX_SIGNAL.pipe:VAL"></span>

### `pipe`

```sml
val pipe : signal
```

```repl
Posix.Signal.pipe;; (* POSIX signal value *)
```

[`pipe`](posix-signal.md#SIG:POSIX_SIGNAL.pipe:VAL:SPEC)

Write on a pipe when there is no process to read it.

<span id="SIG:POSIX_SIGNAL.quit:VAL"></span>

### `quit`

```sml
val quit : signal
```

```repl
Posix.Signal.quit;; (* POSIX signal value *)
```

[`quit`](posix-signal.md#SIG:POSIX_SIGNAL.quit:VAL:SPEC)

Quit.

<span id="SIG:POSIX_SIGNAL.segv:VAL"></span>

### `segv`

```sml
val segv : signal
```

```repl
Posix.Signal.segv;; (* POSIX signal value *)
```

[`segv`](posix-signal.md#SIG:POSIX_SIGNAL.segv:VAL:SPEC)

Segmentation violation.

<span id="SIG:POSIX_SIGNAL.term:VAL"></span>

### `term`

```sml
val term : signal
```

```repl
Posix.Signal.term;; (* POSIX signal value *)
```

[`term`](posix-signal.md#SIG:POSIX_SIGNAL.term:VAL:SPEC)

Software termination signal.

<span id="SIG:POSIX_SIGNAL.usr1:VAL"></span>

### `usr1`

```sml
val usr1 : signal
```

```repl
Posix.Signal.usr1;; (* POSIX signal value *)
```

[`usr1`](posix-signal.md#SIG:POSIX_SIGNAL.usr1:VAL:SPEC)

User-defined signal 1.

<span id="SIG:POSIX_SIGNAL.usr2:VAL"></span>

### `usr2`

```sml
val usr2 : signal
```

```repl
Posix.Signal.usr2;; (* POSIX signal value *)
```

[`usr2`](posix-signal.md#SIG:POSIX_SIGNAL.usr2:VAL:SPEC)

User-defined signal 2.

<span id="SIG:POSIX_SIGNAL.chld:VAL"></span>

### `chld`

```sml
val chld : signal
```

```repl
Posix.Signal.chld;; (* POSIX signal value *)
```

[`chld`](posix-signal.md#SIG:POSIX_SIGNAL.chld:VAL:SPEC)

Sent to parent on child stop or exit.

<span id="SIG:POSIX_SIGNAL.cont:VAL"></span>

### `cont`

```sml
val cont : signal
```

```repl
Posix.Signal.cont;; (* POSIX signal value *)
```

[`cont`](posix-signal.md#SIG:POSIX_SIGNAL.cont:VAL:SPEC)

Continue if stopped. (It cannot be caught or ignored.)

<span id="SIG:POSIX_SIGNAL.stop:VAL"></span>

### `stop`

```sml
val stop : signal
```

```repl
Posix.Signal.stop;; (* POSIX signal value *)
```

[`stop`](posix-signal.md#SIG:POSIX_SIGNAL.stop:VAL:SPEC)

Stop. (It cannot be caught or ignored.)

<span id="SIG:POSIX_SIGNAL.tstp:VAL"></span>

### `tstp`

```sml
val tstp : signal
```

```repl
Posix.Signal.tstp;; (* POSIX signal value *)
```

[`tstp`](posix-signal.md#SIG:POSIX_SIGNAL.tstp:VAL:SPEC)

Interactive stop.

<span id="SIG:POSIX_SIGNAL.ttin:VAL"></span>

### `ttin`

```sml
val ttin : signal
```

```repl
Posix.Signal.ttin;; (* POSIX signal value *)
```

[`ttin`](posix-signal.md#SIG:POSIX_SIGNAL.ttin:VAL:SPEC)

Background read attempted from control terminal.

<span id="SIG:POSIX_SIGNAL.ttou:VAL"></span>

### `ttou`

```sml
val ttou : signal
```

```repl
Posix.Signal.ttou;; (* POSIX signal value *)
```

[`ttou`](posix-signal.md#SIG:POSIX_SIGNAL.ttou:VAL:SPEC)

Background write attempted from control terminal.

---

The name of the corresponding POSIX signal can be derived by capitalizing all letters and adding the string \`\`SIG'' as a prefix. For example, the POSIX signal associated with `usr2` is `SIGUSR2`.
