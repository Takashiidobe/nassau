# <span id="section:0"></span>The `Posix.TTY` structure

---

#### Synopsis

<span id="POSIX_TTY:SIG:SPEC"></span>
<span id="TTY:STR:SPEC"></span>

```sml
signature POSIX_TTY
structure TTY : POSIX_TTY
```

The structure `Posix.TTY` specifies a model of a general terminal interface, as described in Section 7 of the POSIX standard 1003.1,1996**\[CITE\]**.

---

#### Interface

<span id="SIG:POSIX_TTY.pid:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.file_desc:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.eof:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.eol:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.erase:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.intr:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.kill:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.min:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.quit:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.susp:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.time:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.start:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.stop:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.nccs:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cc:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.cc:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.update:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.sub:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.brkint:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.icrnl:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ignbrk:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.igncr:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ignpar:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.inlcr:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.inpck:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.istrip:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ixoff:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ixon:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.parmrk:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.opost:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.clocal:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cread:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cs5:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cs6:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cs7:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cs8:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.csize:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.cstopb:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.hupcl:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.parenb:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.parodd:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.echo:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.echoe:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.echok:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.echonl:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.icanon:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.iexten:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.isig:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.noflsh:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.tostop:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.speed:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.compareSpeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.speedToWord:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.wordToSpeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b0:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b50:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b75:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b110:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b134:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b150:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b200:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b300:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b600:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b1200:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b1800:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b2400:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b4800:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b9600:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b19200:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.b38400:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.termios:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.termios:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.fieldsOf:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getiflag:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getoflag:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getcflag:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getlflag:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getcc:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getospeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getispeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.setospeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.setispeed:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.set_action:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.sanow:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.sadrain:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.saflush:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.flow_action:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.ooff:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.oon:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ioff:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ion:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.queue_sel:TY:SPEC"></span>
<span id="SIG:POSIX_TTY.iflush:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.oflush:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.ioflush:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getattr:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.setattr:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.sendbreak:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.drain:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.flush:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.flow:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.getpgrp:VAL:SPEC"></span>
<span id="SIG:POSIX_TTY.setpgrp:VAL:SPEC"></span>

```sml
eqtype pid
eqtype file_desc
structure V : sig
val eof : int
val eol : int
val erase : int
val intr : int
val kill : int
val min : int
val quit : int
val susp : int
val time : int
val start : int
val stop : int
val nccs : int
type cc
val cc : (int * char) list -> cc
val update : cc * (int * char) list -> cc
val sub : cc * int -> char
end
structure I : sig
include BIT_FLAGS
val brkint : flags
val icrnl : flags
val ignbrk : flags
val igncr : flags
val ignpar : flags
val inlcr : flags
val inpck : flags
val istrip : flags
val ixoff : flags
val ixon : flags
val parmrk : flags
end
structure O : sig
include BIT_FLAGS
val opost : flags
end
structure C : sig
include BIT_FLAGS
val clocal : flags
val cread : flags
val cs5 : flags
val cs6 : flags
val cs7 : flags
val cs8 : flags
val csize : flags
val cstopb : flags
val hupcl : flags
val parenb : flags
val parodd : flags
end
structure L : sig
include BIT_FLAGS
val echo : flags
val echoe : flags
val echok : flags
val echonl : flags
val icanon : flags
val iexten : flags
val isig : flags
val noflsh : flags
val tostop : flags
end
eqtype speed
val compareSpeed : speed * speed -> order
val speedToWord : speed -> SysWord.word
val wordToSpeed : SysWord.word -> speed
val b0 : speed
val b50 : speed
val b75 : speed
val b110 : speed
val b134 : speed
val b150 : speed
val b200 : speed
val b300 : speed
val b600 : speed
val b1200 : speed
val b1800 : speed
val b2400 : speed
val b4800 : speed
val b9600 : speed
val b19200 : speed
val b38400 : speed
type termios
val termios : {
iflag : I.flags,
oflag : O.flags,
cflag : C.flags,
lflag : L.flags,
cc : V.cc,
ispeed : speed,
ospeed : speed
} -> termios
val fieldsOf : termios -> {
iflag : I.flags,
oflag : O.flags,
cflag : C.flags,
lflag : L.flags,
cc : V.cc,
ispeed : speed,
ospeed : speed
}
val getiflag : termios -> I.flags
val getoflag : termios -> O.flags
val getcflag : termios -> C.flags
val getlflag : termios -> L.flags
val getcc : termios -> V.cc
structure CF : sig
val getospeed : termios -> speed
val getispeed : termios -> speed
val setospeed : termios * speed -> termios
val setispeed : termios * speed -> termios
end
structure TC : sig
eqtype set_action
val sanow : set_action
val sadrain : set_action
val saflush : set_action
eqtype flow_action
val ooff : flow_action
val oon : flow_action
val ioff : flow_action
val ion : flow_action
eqtype queue_sel
val iflush : queue_sel
val oflush : queue_sel
val ioflush : queue_sel
val getattr : file_desc -> termios
val setattr : file_desc * set_action * termios -> unit
val sendbreak : file_desc * int -> unit
val drain : file_desc -> unit
val flush : file_desc * queue_sel -> unit
val flow : file_desc * flow_action -> unit
val getpgrp : file_desc -> pid
val setpgrp : file_desc * pid -> unit
end
```

#### Description

<span id="SIG:POSIX_TTY.pid:TY"></span>**`eqtype`**` pid`  
A process identifier.

<span id="SIG:POSIX_TTY.file_desc:TY"></span>**`eqtype`**` file_desc`  
An open file descriptor.

<span id="SIG:POSIX_TTY.V:STR"></span>
**`structure`**` V`  
The `V` substructure provides means for specifying the special control characters.

<span id="SIG:POSIX_TTY.V.eof:VAL"></span>

### `eof`

```sml
val eof : int
```
**`val`**` eof `**`:`**` int`
**`val`**` eol `**`:`**` int`
**`val`**` erase `**`:`**` int`
**`val`**` intr `**`:`**` int`
**`val`**` kill `**`:`**` int`
**`val`**` min `**`:`**` int`
**`val`**` quit `**`:`**` int`
**`val`**` susp `**`:`**` int`
**`val`**` time `**`:`**` int`
**`val`**` start `**`:`**` int`
**`val`**` stop `**`:`**` int`  
Indices for the special control characters `EOF`, `EOL`, `ERASE`, `INTR`, `KILL`, `MIN`, `QUIT`, `SUSP`, `TIME`, `START`, and `STOP`, respectively. These are the indices used in the functions [`cc`](posix-tty.md#SIG:POSIX_TTY.V.cc:VAL:SPEC) and [`sub`](posix-tty.md#SIG:POSIX_TTY.V.sub:VAL:SPEC).



```repl
Posix.TTY.V.eof;; (* EOF control-character index *)
```

<span id="SIG:POSIX_TTY.V.nccs:VAL"></span>

### `nccs`

```sml
val nccs : int
```
**`val`**` nccs `**`:`**` int`  
The total number of special characters. Thus, valid indices range from 0 to `nccs`-1.

<span id="SIG:POSIX_TTY.V.cc:TY"></span>**`type`**` cc`  
A vector of special control characters used by the device driver.



```repl
Posix.TTY.V.nccs;; (* number of control-character slots *)
```

<span id="SIG:POSIX_TTY.V.cc:VAL"></span>

### `cc`

```sml
val cc : (int * char) list -> cc
```
`cc ``l`` `  
creates a value of type [`cc`](posix-tty.md#SIG:POSIX_TTY.V.cc:TY:SPEC), mapping an index to its paired character. Unspecified indices are associated with `#"\000"`. For example, to have the character `#"\^D"` (control-D) serve as the `EOF` (end-of-file) character, one would use

cc \[(V.eof, #"\\D")\]

to create a [`cc`](posix-tty.md#SIG:POSIX_TTY.V.cc:TY:SPEC) value, embed this in a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) type, and invoke [`TC.setattr`](posix-tty.md#SIG:POSIX_TTY.TC.setattr:VAL:SPEC).



```repl
Posix.TTY.V.cc [(Posix.TTY.V.eof, #"\^D")];; (* creates a control-character vector *)
```

<span id="SIG:POSIX_TTY.V.update:VAL"></span>

### `update`

```sml
val update : cc * (int * char) list -> cc
```
`update (``cs``, ``l``) `  
returns a copy of `cs`, but with the new mappings specified by `l` overwriting the original mappings.



```repl
Posix.TTY.V.sub (Posix.TTY.V.update (Posix.TTY.V.cc [], [(Posix.TTY.V.eof, #"\^D")]), Posix.TTY.V.eof);; (* #"\^D" *)
```

<span id="SIG:POSIX_TTY.V.sub:VAL"></span>

### `sub`

```sml
val sub : cc * int -> char
```
`sub (``cs``, ``i``) `  
returns the special control character associated in `cs` with the index `i`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` is negative or `i` \>= [`nccs`](posix-tty.md#SIG:POSIX_TTY.V.nccs:VAL:SPEC).

<span id="SIG:POSIX_TTY.I:STR"></span>
**`structure`**` I`  
The [`I`](posix-tty.md#SIG:POSIX_TTY.I:STR:SPEC) substructure contains flags for specifying input control. The following table provides a brief description of the flags.

---

**Flag name**

**Description**

[`brkint`](posix-tty.md#SIG:POSIX_TTY.I.brkint:VAL:SPEC)

Signal interrupt on break.

[`icrnl`](posix-tty.md#SIG:POSIX_TTY.I.icrnl:VAL:SPEC)

Map `CR` (`#"\^M"`) to `NL` (`#"\n"`) on input.

[`ignbrk`](posix-tty.md#SIG:POSIX_TTY.I.ignbrk:VAL:SPEC)

Ignore a break condition.

[`igncr`](posix-tty.md#SIG:POSIX_TTY.I.igncr:VAL:SPEC)

Ignore `CR` characters.

[`ignpar`](posix-tty.md#SIG:POSIX_TTY.I.ignpar:VAL:SPEC)

Ignore characters with parity errors.

[`inlcr`](posix-tty.md#SIG:POSIX_TTY.I.inlcr:VAL:SPEC)

Map `NL` to `CR` on input.

[`inpck`](posix-tty.md#SIG:POSIX_TTY.I.inpck:VAL:SPEC)

Enable input parity check.

[`istrip`](posix-tty.md#SIG:POSIX_TTY.I.istrip:VAL:SPEC)

Strip the eighth bit of a byte.

[`ixoff`](posix-tty.md#SIG:POSIX_TTY.I.ixoff:VAL:SPEC)

Enable start/stop input control.

[`ixon`](posix-tty.md#SIG:POSIX_TTY.I.ixon:VAL:SPEC)

Enable start/stop output control.

[`parmrk`](posix-tty.md#SIG:POSIX_TTY.I.parmrk:VAL:SPEC)

Mark parity errors.

---


<span id="SIG:POSIX_TTY.O:STR"></span>

**`structure`**` O`

The [`O`](posix-tty.md#SIG:POSIX_TTY.O:STR:SPEC) substructure contains flags for specifying output control.



```repl
Posix.TTY.V.sub (Posix.TTY.V.cc [], Posix.TTY.V.eof);; (* #"\000" for an unset slot *)
```

<span id="SIG:POSIX_TTY.O.opost:VAL"></span>

### `opost`

```sml
val opost : flags
```
**`val`**` opost `**`:`**` flags`  
Perform output processing.

<span id="SIG:POSIX_TTY.C:STR"></span>

**`structure`**` C`

The [`C`](posix-tty.md#SIG:POSIX_TTY.C:STR:SPEC) substructure contains flags for specifying basic terminal hardware control. The following table provides a brief description of the flags.

---

**Flag name**

**Description**

[`clocal`](posix-tty.md#SIG:POSIX_TTY.C.clocal:VAL:SPEC)

Ignore modem status lines.

[`cread`](posix-tty.md#SIG:POSIX_TTY.C.cread:VAL:SPEC)

Enable the receiver.

[`csize`](posix-tty.md#SIG:POSIX_TTY.C.csize:VAL:SPEC)

Mask for the number of bits per byte used for both transmission and reception. This is the union of [`cs5`](posix-tty.md#SIG:POSIX_TTY.C.cs5:VAL:SPEC), [`cs6`](posix-tty.md#SIG:POSIX_TTY.C.cs6:VAL:SPEC), [`cs7`](posix-tty.md#SIG:POSIX_TTY.C.cs7:VAL:SPEC), and [`cs8`](posix-tty.md#SIG:POSIX_TTY.C.cs8:VAL:SPEC).

[`cs5`](posix-tty.md#SIG:POSIX_TTY.C.cs5:VAL:SPEC)

5 bits per byte.

[`cs6`](posix-tty.md#SIG:POSIX_TTY.C.cs6:VAL:SPEC)

6 bits per byte.

[`cs7`](posix-tty.md#SIG:POSIX_TTY.C.cs7:VAL:SPEC)

7 bits per byte.

[`cs8`](posix-tty.md#SIG:POSIX_TTY.C.cs8:VAL:SPEC)

8 bits per byte.

[`cstopb`](posix-tty.md#SIG:POSIX_TTY.C.cstopb:VAL:SPEC)

Specifies sending two stop bits rather than one.

[`hupcl`](posix-tty.md#SIG:POSIX_TTY.C.hupcl:VAL:SPEC)

Hang up the modem connection when the last process with the port open closes it.

[`parenb`](posix-tty.md#SIG:POSIX_TTY.C.parenb:VAL:SPEC)

Enable parity generation and detection.

[`parodd`](posix-tty.md#SIG:POSIX_TTY.C.parodd:VAL:SPEC)

Use odd parity rather than even if [`parenb`](posix-tty.md#SIG:POSIX_TTY.C.parenb:VAL:SPEC) is set.

---


<span id="SIG:POSIX_TTY.L:STR"></span>

**`structure`**` L`

The `L` substructure contains flags for specifying various local control modes. The following table provides a brief description of the flags.

---

**Flag name**

**Description**

[`echo`](posix-tty.md#SIG:POSIX_TTY.L.echo:VAL:SPEC)

Echo input characters back to the terminal.

[`echoe`](posix-tty.md#SIG:POSIX_TTY.L.echoe:VAL:SPEC)

Echo the `ERASE` character on backspace in canonical mode.

[`echok`](posix-tty.md#SIG:POSIX_TTY.L.echok:VAL:SPEC)

Echo the `KILL` character in canonical mode.

[`echonl`](posix-tty.md#SIG:POSIX_TTY.L.echonl:VAL:SPEC)

In canonical mode, echo a `NL` character even if `echo` is not set.

[`icanon`](posix-tty.md#SIG:POSIX_TTY.L.icanon:VAL:SPEC)

Set canonical mode, enabling erase and kill processing, and providing line-based input.

[`iexten`](posix-tty.md#SIG:POSIX_TTY.L.iexten:VAL:SPEC)

Enable extended functions.

[`isig`](posix-tty.md#SIG:POSIX_TTY.L.isig:VAL:SPEC)

Enable input characters to be mapped to signals.

[`noflsh`](posix-tty.md#SIG:POSIX_TTY.L.noflsh:VAL:SPEC)

Disable the normal input and output flushing connected with the `INTR`, `QUIT`, and `SUSP` characters. (See the [`Posix.TTY.V`](posix-tty.md#SIG:POSIX_TTY.V:STR:SPEC) substructure.)

[`tostop`](posix-tty.md#SIG:POSIX_TTY.L.tostop:VAL:SPEC)

Send [`Posix.Signal.ttou`](posix-signal.md#SIG:POSIX_SIGNAL.ttou:VAL:SPEC) for background output.

---


<span id="SIG:POSIX_TTY.speed:TY"></span>**`eqtype`**` speed`

Terminal input and output baud rates.



```repl
Posix.TTY.O.opost;; (* output-processing flag *)
```

<span id="SIG:POSIX_TTY.compareSpeed:VAL"></span>

### `compareSpeed`

```sml
val compareSpeed : speed * speed -> order
```
`compareSpeed (``sp``, ``sp'``) `

returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when the baud rate `sp` is less than, equal to, or greater than that of `sp'`, respectively.



```repl
Posix.TTY.compareSpeed (Posix.TTY.b9600, Posix.TTY.b19200);; (* LESS *)
```

<span id="SIG:POSIX_TTY.speedToWord:VAL"></span>

### `speedToWord`

```sml
val speedToWord : speed -> SysWord.word
```
**`val`**` speedToWord `**`:`**` speed `**`->`**` SysWord.word`
**`val`**` wordToSpeed `**`:`**` SysWord.word `**`->`**` speed`

These converts between a [`speed`](posix-tty.md#SIG:POSIX_TTY.speed:TY:SPEC) value and its underlying word representation. No checking is performed by [`wordToSpeed`](posix-tty.md#SIG:POSIX_TTY.wordToSpeed:VAL:SPEC) to ensure the resulting value corresponds to an allowed speed in the given system.

<span id="SIG:POSIX_TTY.termios:TY"></span>**`type`**` termios`

The attributes associated with a terminal. It acts as an abstract representation of the record used as the argument to the [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:VAL:SPEC) function.



```repl
Posix.TTY.speedToWord Posix.TTY.b9600;; (* word representation of the speed *)
```

<span id="SIG:POSIX_TTY.termios:VAL"></span>

### `termios`

```sml
val termios : {
iflag : I.flags,
oflag : O.flags,
cflag : C.flags,
lflag : L.flags,
cc : V.cc,
ispeed : speed,
ospeed : speed
} -> termios
```
**`val`**` termios `**`:`**` {`
`                  iflag `**`:`**` I.flags,`
`                  oflag `**`:`**` O.flags,`
`                  cflag `**`:`**` C.flags,`
`                  lflag `**`:`**` L.flags,`
`                  cc `**`:`**` V.cc,`
`                  ispeed `**`:`**` speed,`
`                  ospeed `**`:`**` speed`
`                } `**`->`**` termios`

This creates a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value using the given flags, special characters, and speeds.



```repl
Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600};; (* constructs terminal attributes with empty flags and 9600 baud *)
```

<span id="SIG:POSIX_TTY.fieldsOf:VAL"></span>

### `fieldsOf`

```sml
val fieldsOf : termios -> {
iflag : I.flags,
oflag : O.flags,
cflag : C.flags,
lflag : L.flags,
cc : V.cc,
ispeed : speed,
ospeed : speed
}
```
**`val`**` fieldsOf `**`:`**` termios`
`                 `**`->`**` {`
`                   iflag `**`:`**` I.flags,`
`                   oflag `**`:`**` O.flags,`
`                   cflag `**`:`**` C.flags,`
`                   lflag `**`:`**` L.flags,`
`                   cc `**`:`**` V.cc,`
`                   ispeed `**`:`**` speed,`
`                   ospeed `**`:`**` speed`
`                 }`

This returns a concrete representation of a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value.



```repl
Posix.TTY.fieldsOf (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* record of terminal attributes *)
```

<span id="SIG:POSIX_TTY.getiflag:VAL"></span>

### `getiflag`

```sml
val getiflag : termios -> I.flags
```
**`val`**` getiflag `**`:`**` termios `**`->`**` I.flags`
**`val`**` getoflag `**`:`**` termios `**`->`**` O.flags`
**`val`**` getcflag `**`:`**` termios `**`->`**` C.flags`
**`val`**` getlflag `**`:`**` termios `**`->`**` L.flags`
**`val`**` getcc `**`:`**` termios `**`->`**` V.cc`

These are the obvious projection functions from a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value to its constituent fields.

<span id="SIG:POSIX_TTY.CF:STR"></span>

**`structure`**` CF`

The [`CF`](posix-tty.md#SIG:POSIX_TTY.CF:STR:SPEC) substructure contains functions for getting and setting the input and output baud rates in a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value.



```repl
Posix.TTY.getiflag (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* empty input flags *)
```

<span id="SIG:POSIX_TTY.CF.getospeed:VAL"></span>

### `getospeed`

```sml
val getospeed : termios -> speed
```
**`val`**` getospeed `**`:`**` termios `**`->`**` speed`
**`val`**` getispeed `**`:`**` termios `**`->`**` speed`  
These return the output and input baud rates, respectively, of the argument.



```repl
Posix.TTY.CF.getospeed (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* b9600 *)
```

<span id="SIG:POSIX_TTY.CF.setospeed:VAL"></span>

### `setospeed`

```sml
val setospeed : termios * speed -> termios
```
`setospeed (``t``, ``speed``) `
`setispeed (``t``, ``speed``)`  
These return a copy of `t`, but with the output (input) speed set to `speed`.

<span id="SIG:POSIX_TTY.TC:STR"></span>

**`structure`**` TC`

The [`TC`](posix-tty.md#SIG:POSIX_TTY.TC:STR:SPEC) substructure contains various types and functions used for handling terminal line control.

<span id="SIG:POSIX_TTY.TC.set_action:TY"></span>**`eqtype`**` set_action`  
Values of this type specify the behavior of the [`setattr`](posix-tty.md#SIG:POSIX_TTY.TC.setattr:VAL:SPEC) function.



```repl
Posix.TTY.CF.getospeed (Posix.TTY.CF.setospeed (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600}, Posix.TTY.b19200));; (* b19200 *)
```

<span id="SIG:POSIX_TTY.TC.sanow:VAL"></span>

### `sanow`

```sml
val sanow : set_action
```
**`val`**` sanow `**`:`**` set_action`
**`val`**` sadrain `**`:`**` set_action`
**`val`**` saflush `**`:`**` set_action`  
[`sanow`](posix-tty.md#SIG:POSIX_TTY.TC.sanow:VAL:SPEC)  
Changes occur immediately.

[`sadrain`](posix-tty.md#SIG:POSIX_TTY.TC.sadrain:VAL:SPEC)  
Changes occur after all output is transmitted.

[`saflush`](posix-tty.md#SIG:POSIX_TTY.TC.saflush:VAL:SPEC)  
Changes occur after all output is transmitted and after all received but unread input is discarded.


<span id="SIG:POSIX_TTY.TC.flow_action:TY"></span>**`eqtype`**` flow_action`  
Values of this type specify the behavior of the [`flow`](posix-tty.md#SIG:POSIX_TTY.TC.flow:VAL:SPEC) function.



```repl
Posix.TTY.TC.sanow;; (* immediate terminal-setting action *)
```

<span id="SIG:POSIX_TTY.TC.ooff:VAL"></span>

### `ooff`

```sml
val ooff : flow_action
```
**`val`**` ooff `**`:`**` flow_action`
**`val`**` oon `**`:`**` flow_action`
**`val`**` ioff `**`:`**` flow_action`
**`val`**` ion `**`:`**` flow_action`  
[`ooff`](posix-tty.md#SIG:POSIX_TTY.TC.ooff:VAL:SPEC)  
Causes suspension of output.

[`oon`](posix-tty.md#SIG:POSIX_TTY.TC.oon:VAL:SPEC)  
Restarts suspended output.

[`ioff`](posix-tty.md#SIG:POSIX_TTY.TC.ioff:VAL:SPEC)  
Causes the transmission of a `STOP` character to the terminal device, to stop it from transmitting data.

[`ion`](posix-tty.md#SIG:POSIX_TTY.TC.ion:VAL:SPEC)  
Causes the transmission of a `START` character to the terminal device, to restart it transmitting data.


<span id="SIG:POSIX_TTY.TC.queue_sel:TY"></span>**`eqtype`**` queue_sel`  
Values of this type specify the behavior of the [`flush`](posix-tty.md#SIG:POSIX_TTY.TC.flush:VAL:SPEC) function.



```repl
Posix.TTY.TC.ooff;; (* suspend output flow action *)
```

<span id="SIG:POSIX_TTY.TC.iflush:VAL"></span>

### `iflush`

```sml
val iflush : queue_sel
```
**`val`**` iflush `**`:`**` queue_sel`
**`val`**` oflush `**`:`**` queue_sel`
**`val`**` ioflush `**`:`**` queue_sel`  
[`iflush`](posix-tty.md#SIG:POSIX_TTY.TC.iflush:VAL:SPEC)  
Causes all data received but not read to be flushed.

[`oflush`](posix-tty.md#SIG:POSIX_TTY.TC.oflush:VAL:SPEC)  
Causes all data written but not transmitted to be flushed.

[`ioflush`](posix-tty.md#SIG:POSIX_TTY.TC.ioflush:VAL:SPEC)  
Discards all data written but not transmitted, or received but not read.




```repl
Posix.TTY.TC.iflush;; (* flush input queue selector *)
```

<span id="SIG:POSIX_TTY.TC.getattr:VAL"></span>

### `getattr`

```sml
val getattr : file_desc -> termios
```
`getattr ``fd`` `  
gets the attributes of the terminal associated with file descriptor `fd`.



```repl
if Posix.ProcEnv.isatty Posix.FileSys.stdin then SOME (Posix.TTY.TC.getattr Posix.FileSys.stdin) else NONE;; (* reads attributes only when stdin is a terminal *)
```

<span id="SIG:POSIX_TTY.TC.setattr:VAL"></span>

### `setattr`

```sml
val setattr : file_desc * set_action * termios -> unit
```
`setattr (``fd``, ``action``, ``termios``) `  
sets the attributes of the terminal associated with file descriptor `fd` as specified in `termios`. When the change occurs is specified by `action`.



```repl
Posix.TTY.TC.setattr;; (* applies settings to a terminal when called *)
```

<span id="SIG:POSIX_TTY.TC.sendbreak:VAL"></span>

### `sendbreak`

```sml
val sendbreak : file_desc * int -> unit
```
`sendbreak (``fd``, ``t``) `  
causes the transmission of a sequence of zero-valued bits to be sent, if the associated terminal is using asynchronous serial data transmission. If `t` is 0, this will send zero-valued bits for at least a quarter second, and no more than half a second. If `t` is not zero, zero-valued bits are transmitted for an implementation-defined period of time.



```repl
Posix.TTY.TC.sendbreak;; (* sends a terminal break when called *)
```

<span id="SIG:POSIX_TTY.TC.drain:VAL"></span>

### `drain`

```sml
val drain : file_desc -> unit
```
`drain ``fd`` `  
waits for all output written on `fd` to be transmitted.



```repl
Posix.TTY.TC.drain;; (* waits for terminal output to drain when called *)
```

<span id="SIG:POSIX_TTY.TC.flush:VAL"></span>

### `flush`

```sml
val flush : file_desc * queue_sel -> unit
```
`flush (``fd``, ``qs``) `  
discards any data written but not transmitted, or received but not read, depending on the value of `qs`.



```repl
Posix.TTY.TC.flush;; (* flushes a terminal queue when called *)
```

<span id="SIG:POSIX_TTY.TC.flow:VAL"></span>

### `flow`

```sml
val flow : file_desc * flow_action -> unit
```
`flow (``fd``, ``action``) `  
suspends and restarts transmission or reception of data, depending on the value of `action`.



```repl
Posix.TTY.TC.flow;; (* changes terminal flow when called *)
```

<span id="SIG:POSIX_TTY.TC.getpgrp:VAL"></span>

### `getpgrp`

```sml
val getpgrp : file_desc -> pid
```
`getpgrp ``fd`` `  
returns the process group ID of the foreground process group associated with the terminal attached to `fd`.



```repl
if Posix.ProcEnv.isatty Posix.FileSys.stdin then SOME (Posix.TTY.TC.getpgrp Posix.FileSys.stdin) else NONE;; (* reads the foreground group only when stdin is a terminal *)
```

<span id="SIG:POSIX_TTY.TC.setpgrp:VAL"></span>

### `setpgrp`

```sml
val setpgrp : file_desc * pid -> unit
```
`setpgrp (``fd``, ``pid``) `  
sets the foreground process group ID associated with `fd` to `pid`.




```repl
Posix.TTY.TC.setpgrp;; (* changes the foreground group when called *)
```

<span id="SIG:POSIX_TTY.V.eol:VAL"></span>

### `eol`

```sml
val eol : int
```

```repl
Posix.TTY.V.eol;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.erase:VAL"></span>

### `erase`

```sml
val erase : int
```

```repl
Posix.TTY.V.erase;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.intr:VAL"></span>

### `intr`

```sml
val intr : int
```

```repl
Posix.TTY.V.intr;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.kill:VAL"></span>

### `kill`

```sml
val kill : int
```

```repl
Posix.TTY.V.kill;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.min:VAL"></span>

### `min`

```sml
val min : int
```

```repl
Posix.TTY.V.min;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.quit:VAL"></span>

### `quit`

```sml
val quit : int
```

```repl
Posix.TTY.V.quit;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.susp:VAL"></span>

### `susp`

```sml
val susp : int
```

```repl
Posix.TTY.V.susp;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.time:VAL"></span>

### `time`

```sml
val time : int
```

```repl
Posix.TTY.V.time;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.start:VAL"></span>

### `start`

```sml
val start : int
```

```repl
Posix.TTY.V.start;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.V.stop:VAL"></span>

### `stop`

```sml
val stop : int
```

```repl
Posix.TTY.V.stop;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.brkint:VAL"></span>

### `brkint`

```sml
val brkint : flags
```

```repl
Posix.TTY.I.brkint;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.icrnl:VAL"></span>

### `icrnl`

```sml
val icrnl : flags
```

```repl
Posix.TTY.I.icrnl;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.ignbrk:VAL"></span>

### `ignbrk`

```sml
val ignbrk : flags
```

```repl
Posix.TTY.I.ignbrk;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.igncr:VAL"></span>

### `igncr`

```sml
val igncr : flags
```

```repl
Posix.TTY.I.igncr;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.ignpar:VAL"></span>

### `ignpar`

```sml
val ignpar : flags
```

```repl
Posix.TTY.I.ignpar;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.inlcr:VAL"></span>

### `inlcr`

```sml
val inlcr : flags
```

```repl
Posix.TTY.I.inlcr;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.inpck:VAL"></span>

### `inpck`

```sml
val inpck : flags
```

```repl
Posix.TTY.I.inpck;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.istrip:VAL"></span>

### `istrip`

```sml
val istrip : flags
```

```repl
Posix.TTY.I.istrip;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.ixoff:VAL"></span>

### `ixoff`

```sml
val ixoff : flags
```

```repl
Posix.TTY.I.ixoff;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.ixon:VAL"></span>

### `ixon`

```sml
val ixon : flags
```

```repl
Posix.TTY.I.ixon;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.I.parmrk:VAL"></span>

### `parmrk`

```sml
val parmrk : flags
```

```repl
Posix.TTY.I.parmrk;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.clocal:VAL"></span>

### `clocal`

```sml
val clocal : flags
```

```repl
Posix.TTY.C.clocal;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cread:VAL"></span>

### `cread`

```sml
val cread : flags
```

```repl
Posix.TTY.C.cread;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cs5:VAL"></span>

### `cs5`

```sml
val cs5 : flags
```

```repl
Posix.TTY.C.cs5;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cs6:VAL"></span>

### `cs6`

```sml
val cs6 : flags
```

```repl
Posix.TTY.C.cs6;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cs7:VAL"></span>

### `cs7`

```sml
val cs7 : flags
```

```repl
Posix.TTY.C.cs7;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cs8:VAL"></span>

### `cs8`

```sml
val cs8 : flags
```

```repl
Posix.TTY.C.cs8;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.csize:VAL"></span>

### `csize`

```sml
val csize : flags
```

```repl
Posix.TTY.C.csize;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.cstopb:VAL"></span>

### `cstopb`

```sml
val cstopb : flags
```

```repl
Posix.TTY.C.cstopb;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.hupcl:VAL"></span>

### `hupcl`

```sml
val hupcl : flags
```

```repl
Posix.TTY.C.hupcl;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.parenb:VAL"></span>

### `parenb`

```sml
val parenb : flags
```

```repl
Posix.TTY.C.parenb;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.C.parodd:VAL"></span>

### `parodd`

```sml
val parodd : flags
```

```repl
Posix.TTY.C.parodd;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.echo:VAL"></span>

### `echo`

```sml
val echo : flags
```

```repl
Posix.TTY.L.echo;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.echoe:VAL"></span>

### `echoe`

```sml
val echoe : flags
```

```repl
Posix.TTY.L.echoe;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.echok:VAL"></span>

### `echok`

```sml
val echok : flags
```

```repl
Posix.TTY.L.echok;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.echonl:VAL"></span>

### `echonl`

```sml
val echonl : flags
```

```repl
Posix.TTY.L.echonl;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.icanon:VAL"></span>

### `icanon`

```sml
val icanon : flags
```

```repl
Posix.TTY.L.icanon;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.iexten:VAL"></span>

### `iexten`

```sml
val iexten : flags
```

```repl
Posix.TTY.L.iexten;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.isig:VAL"></span>

### `isig`

```sml
val isig : flags
```

```repl
Posix.TTY.L.isig;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.noflsh:VAL"></span>

### `noflsh`

```sml
val noflsh : flags
```

```repl
Posix.TTY.L.noflsh;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.L.tostop:VAL"></span>

### `tostop`

```sml
val tostop : flags
```

```repl
Posix.TTY.L.tostop;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.wordToSpeed:VAL"></span>

### `wordToSpeed`

```sml
val wordToSpeed : SysWord.word -> speed
```

```repl
Posix.TTY.wordToSpeed (Posix.TTY.speedToWord Posix.TTY.b9600);; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b0:VAL"></span>

### `b0`

```sml
val b0 : speed
```

```repl
Posix.TTY.b0;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b50:VAL"></span>

### `b50`

```sml
val b50 : speed
```

```repl
Posix.TTY.b50;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b75:VAL"></span>

### `b75`

```sml
val b75 : speed
```

```repl
Posix.TTY.b75;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b110:VAL"></span>

### `b110`

```sml
val b110 : speed
```

```repl
Posix.TTY.b110;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b134:VAL"></span>

### `b134`

```sml
val b134 : speed
```

```repl
Posix.TTY.b134;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b150:VAL"></span>

### `b150`

```sml
val b150 : speed
```

```repl
Posix.TTY.b150;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b200:VAL"></span>

### `b200`

```sml
val b200 : speed
```

```repl
Posix.TTY.b200;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b300:VAL"></span>

### `b300`

```sml
val b300 : speed
```

```repl
Posix.TTY.b300;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b600:VAL"></span>

### `b600`

```sml
val b600 : speed
```

```repl
Posix.TTY.b600;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b1200:VAL"></span>

### `b1200`

```sml
val b1200 : speed
```

```repl
Posix.TTY.b1200;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b1800:VAL"></span>

### `b1800`

```sml
val b1800 : speed
```

```repl
Posix.TTY.b1800;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b2400:VAL"></span>

### `b2400`

```sml
val b2400 : speed
```

```repl
Posix.TTY.b2400;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b4800:VAL"></span>

### `b4800`

```sml
val b4800 : speed
```

```repl
Posix.TTY.b4800;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b9600:VAL"></span>

### `b9600`

```sml
val b9600 : speed
```

```repl
Posix.TTY.b9600;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b19200:VAL"></span>

### `b19200`

```sml
val b19200 : speed
```

```repl
Posix.TTY.b19200;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.b38400:VAL"></span>

### `b38400`

```sml
val b38400 : speed
```

```repl
Posix.TTY.b38400;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.getoflag:VAL"></span>

### `getoflag`

```sml
val getoflag : termios -> O.flags
```

```repl
Posix.TTY.getoflag (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* REPL result *)
```

<span id="SIG:POSIX_TTY.getcflag:VAL"></span>

### `getcflag`

```sml
val getcflag : termios -> C.flags
```

```repl
Posix.TTY.getcflag (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* REPL result *)
```

<span id="SIG:POSIX_TTY.getlflag:VAL"></span>

### `getlflag`

```sml
val getlflag : termios -> L.flags
```

```repl
Posix.TTY.getlflag (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* REPL result *)
```

<span id="SIG:POSIX_TTY.getcc:VAL"></span>

### `getcc`

```sml
val getcc : termios -> V.cc
```

```repl
Posix.TTY.getcc (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* REPL result *)
```

<span id="SIG:POSIX_TTY.CF.getispeed:VAL"></span>

### `getispeed`

```sml
val getispeed : termios -> speed
```

```repl
Posix.TTY.CF.getispeed (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600});; (* REPL result *)
```

<span id="SIG:POSIX_TTY.CF.setispeed:VAL"></span>

### `setispeed`

```sml
val setispeed : termios * speed -> termios
```

```repl
Posix.TTY.CF.setispeed (Posix.TTY.termios {iflag = Posix.TTY.I.flags [], oflag = Posix.TTY.O.flags [], cflag = Posix.TTY.C.flags [], lflag = Posix.TTY.L.flags [], cc = Posix.TTY.V.cc [], ispeed = Posix.TTY.b9600, ospeed = Posix.TTY.b9600}, Posix.TTY.b19200);; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.sadrain:VAL"></span>

### `sadrain`

```sml
val sadrain : set_action
```

```repl
Posix.TTY.TC.sadrain;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.saflush:VAL"></span>

### `saflush`

```sml
val saflush : set_action
```

```repl
Posix.TTY.TC.saflush;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.oon:VAL"></span>

### `oon`

```sml
val oon : flow_action
```

```repl
Posix.TTY.TC.oon;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.ioff:VAL"></span>

### `ioff`

```sml
val ioff : flow_action
```

```repl
Posix.TTY.TC.ioff;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.ion:VAL"></span>

### `ion`

```sml
val ion : flow_action
```

```repl
Posix.TTY.TC.ion;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.oflush:VAL"></span>

### `oflush`

```sml
val oflush : queue_sel
```

```repl
Posix.TTY.TC.oflush;; (* REPL result *)
```

<span id="SIG:POSIX_TTY.TC.ioflush:VAL"></span>

### `ioflush`

```sml
val ioflush : queue_sel
```

```repl
Posix.TTY.TC.ioflush;; (* REPL result *)
```

#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Error`](posix.md#SIG:POSIX.Error:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

#### Discussion

The values of type [`speed`](posix-tty.md#SIG:POSIX_TTY.speed:TY:SPEC) defined in this structure specify the standard baud rates with the obvious correspondence, _i.e._, [`b1200`](posix-tty.md#SIG:POSIX_TTY.b1200:VAL:SPEC) is 1200 baud, [`b9600`](posix-tty.md#SIG:POSIX_TTY.b9600:VAL:SPEC) is 9600 baud, etc. The value [`b0`](posix-tty.md#SIG:POSIX_TTY.b0:VAL:SPEC) indicates \`\`hang up.''
