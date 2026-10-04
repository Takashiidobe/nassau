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

<span id="SIG:POSIX_SIGNAL.toWord:VAL"></span>**`val`**` toWord `**`:`**` signal `**`->`**` SysWord.word`
**`val`**` fromWord `**`:`**` SysWord.word `**`->`**` signal`  
These convert between a signal identifier and its underlying integer representation. Note that [`fromWord`](posix-signal.md#SIG:POSIX_SIGNAL.fromWord:VAL:SPEC) does not check that the result corresponds to a valid POSIX signal.

#### Examples

```repl
Posix.Signal.toWord Posix.Signal.int;;
```

#### See Also

> [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

#### Discussion

The values defined in this structure represent the standard POSIX signals. The following table provides a brief description of their meanings.

---

**SML name**

**Description**

[`abrt`](posix-signal.md#SIG:POSIX_SIGNAL.abrt:VAL:SPEC)

End process (abort).

[`alrm`](posix-signal.md#SIG:POSIX_SIGNAL.alrm:VAL:SPEC)

Alarm clock.

[`bus`](posix-signal.md#SIG:POSIX_SIGNAL.bus:VAL:SPEC)

Bus error.

[`fpe`](posix-signal.md#SIG:POSIX_SIGNAL.fpe:VAL:SPEC)

Floating-point exception.

[`hup`](posix-signal.md#SIG:POSIX_SIGNAL.hup:VAL:SPEC)

Hangup.

[`ill`](posix-signal.md#SIG:POSIX_SIGNAL.ill:VAL:SPEC)

Illegal instruction.

[`int`](posix-signal.md#SIG:POSIX_SIGNAL.int:VAL:SPEC)

Interrupt.

[`kill`](posix-signal.md#SIG:POSIX_SIGNAL.kill:VAL:SPEC)

Kill. (It cannot be caught or ignored.)

[`pipe`](posix-signal.md#SIG:POSIX_SIGNAL.pipe:VAL:SPEC)

Write on a pipe when there is no process to read it.

[`quit`](posix-signal.md#SIG:POSIX_SIGNAL.quit:VAL:SPEC)

Quit.

[`segv`](posix-signal.md#SIG:POSIX_SIGNAL.segv:VAL:SPEC)

Segmentation violation.

[`term`](posix-signal.md#SIG:POSIX_SIGNAL.term:VAL:SPEC)

Software termination signal.

[`usr1`](posix-signal.md#SIG:POSIX_SIGNAL.usr1:VAL:SPEC)

User-defined signal 1.

[`usr2`](posix-signal.md#SIG:POSIX_SIGNAL.usr2:VAL:SPEC)

User-defined signal 2.

[`chld`](posix-signal.md#SIG:POSIX_SIGNAL.chld:VAL:SPEC)

Sent to parent on child stop or exit.

[`cont`](posix-signal.md#SIG:POSIX_SIGNAL.cont:VAL:SPEC)

Continue if stopped. (It cannot be caught or ignored.)

[`stop`](posix-signal.md#SIG:POSIX_SIGNAL.stop:VAL:SPEC)

Stop. (It cannot be caught or ignored.)

[`tstp`](posix-signal.md#SIG:POSIX_SIGNAL.tstp:VAL:SPEC)

Interactive stop.

[`ttin`](posix-signal.md#SIG:POSIX_SIGNAL.ttin:VAL:SPEC)

Background read attempted from control terminal.

[`ttou`](posix-signal.md#SIG:POSIX_SIGNAL.ttou:VAL:SPEC)

Background write attempted from control terminal.

---

The name of the corresponding POSIX signal can be derived by capitalizing all letters and adding the string \`\`SIG'' as a prefix. For example, the POSIX signal associated with `usr2` is `SIGUSR2`.
