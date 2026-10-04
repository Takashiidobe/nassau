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

<span id="SIG:POSIX_TTY.V.eof:VAL"></span>**`val`**` eof `**`:`**` int`
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

<span id="SIG:POSIX_TTY.V.nccs:VAL"></span>**`val`**` nccs `**`:`**` int`  
The total number of special characters. Thus, valid indices range from 0 to `nccs`-1.

<span id="SIG:POSIX_TTY.V.cc:TY"></span>**`type`**` cc`  
A vector of special control characters used by the device driver.

<span id="SIG:POSIX_TTY.V.cc:VAL"></span>
`cc ``l`` `  
creates a value of type [`cc`](posix-tty.md#SIG:POSIX_TTY.V.cc:TY:SPEC), mapping an index to its paired character. Unspecified indices are associated with `#"\000"`. For example, to have the character `#"\^D"` (control-D) serve as the `EOF` (end-of-file) character, one would use

cc \[(V.eof, #"\\D")\]

to create a [`cc`](posix-tty.md#SIG:POSIX_TTY.V.cc:TY:SPEC) value, embed this in a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) type, and invoke [`TC.setattr`](posix-tty.md#SIG:POSIX_TTY.TC.setattr:VAL:SPEC).

<span id="SIG:POSIX_TTY.V.update:VAL"></span>
`update (``cs``, ``l``) `  
returns a copy of `cs`, but with the new mappings specified by `l` overwriting the original mappings.

<span id="SIG:POSIX_TTY.V.sub:VAL"></span>
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

<span id="SIG:POSIX_TTY.O.opost:VAL"></span>**`val`**` opost `**`:`**` flags`  
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

<span id="SIG:POSIX_TTY.compareSpeed:VAL"></span>

`compareSpeed (``sp``, ``sp'``) `

returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when the baud rate `sp` is less than, equal to, or greater than that of `sp'`, respectively.

<span id="SIG:POSIX_TTY.speedToWord:VAL"></span>**`val`**` speedToWord `**`:`**` speed `**`->`**` SysWord.word`
**`val`**` wordToSpeed `**`:`**` SysWord.word `**`->`**` speed`

These converts between a [`speed`](posix-tty.md#SIG:POSIX_TTY.speed:TY:SPEC) value and its underlying word representation. No checking is performed by [`wordToSpeed`](posix-tty.md#SIG:POSIX_TTY.wordToSpeed:VAL:SPEC) to ensure the resulting value corresponds to an allowed speed in the given system.

<span id="SIG:POSIX_TTY.termios:TY"></span>**`type`**` termios`

The attributes associated with a terminal. It acts as an abstract representation of the record used as the argument to the [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:VAL:SPEC) function.

<span id="SIG:POSIX_TTY.termios:VAL"></span>**`val`**` termios `**`:`**` {`
`                  iflag `**`:`**` I.flags,`
`                  oflag `**`:`**` O.flags,`
`                  cflag `**`:`**` C.flags,`
`                  lflag `**`:`**` L.flags,`
`                  cc `**`:`**` V.cc,`
`                  ispeed `**`:`**` speed,`
`                  ospeed `**`:`**` speed`
`                } `**`->`**` termios`

This creates a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value using the given flags, special characters, and speeds.

<span id="SIG:POSIX_TTY.fieldsOf:VAL"></span>**`val`**` fieldsOf `**`:`**` termios`
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

<span id="SIG:POSIX_TTY.getiflag:VAL"></span>**`val`**` getiflag `**`:`**` termios `**`->`**` I.flags`
**`val`**` getoflag `**`:`**` termios `**`->`**` O.flags`
**`val`**` getcflag `**`:`**` termios `**`->`**` C.flags`
**`val`**` getlflag `**`:`**` termios `**`->`**` L.flags`
**`val`**` getcc `**`:`**` termios `**`->`**` V.cc`

These are the obvious projection functions from a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value to its constituent fields.

<span id="SIG:POSIX_TTY.CF:STR"></span>

**`structure`**` CF`

The [`CF`](posix-tty.md#SIG:POSIX_TTY.CF:STR:SPEC) substructure contains functions for getting and setting the input and output baud rates in a [`termios`](posix-tty.md#SIG:POSIX_TTY.termios:TY:SPEC) value.

<span id="SIG:POSIX_TTY.CF.getospeed:VAL"></span>**`val`**` getospeed `**`:`**` termios `**`->`**` speed`
**`val`**` getispeed `**`:`**` termios `**`->`**` speed`  
These return the output and input baud rates, respectively, of the argument.

<span id="SIG:POSIX_TTY.CF.setospeed:VAL"></span>
`setospeed (``t``, ``speed``) `
`setispeed (``t``, ``speed``)`  
These return a copy of `t`, but with the output (input) speed set to `speed`.

<span id="SIG:POSIX_TTY.TC:STR"></span>

**`structure`**` TC`

The [`TC`](posix-tty.md#SIG:POSIX_TTY.TC:STR:SPEC) substructure contains various types and functions used for handling terminal line control.

<span id="SIG:POSIX_TTY.TC.set_action:TY"></span>**`eqtype`**` set_action`  
Values of this type specify the behavior of the [`setattr`](posix-tty.md#SIG:POSIX_TTY.TC.setattr:VAL:SPEC) function.

<span id="SIG:POSIX_TTY.TC.sanow:VAL"></span>**`val`**` sanow `**`:`**` set_action`
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

<span id="SIG:POSIX_TTY.TC.ooff:VAL"></span>**`val`**` ooff `**`:`**` flow_action`
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

<span id="SIG:POSIX_TTY.TC.iflush:VAL"></span>**`val`**` iflush `**`:`**` queue_sel`
**`val`**` oflush `**`:`**` queue_sel`
**`val`**` ioflush `**`:`**` queue_sel`  
[`iflush`](posix-tty.md#SIG:POSIX_TTY.TC.iflush:VAL:SPEC)  
Causes all data received but not read to be flushed.

[`oflush`](posix-tty.md#SIG:POSIX_TTY.TC.oflush:VAL:SPEC)  
Causes all data written but not transmitted to be flushed.

[`ioflush`](posix-tty.md#SIG:POSIX_TTY.TC.ioflush:VAL:SPEC)  
Discards all data written but not transmitted, or received but not read.


<span id="SIG:POSIX_TTY.TC.getattr:VAL"></span>
`getattr ``fd`` `  
gets the attributes of the terminal associated with file descriptor `fd`.

<span id="SIG:POSIX_TTY.TC.setattr:VAL"></span>
`setattr (``fd``, ``action``, ``termios``) `  
sets the attributes of the terminal associated with file descriptor `fd` as specified in `termios`. When the change occurs is specified by `action`.

<span id="SIG:POSIX_TTY.TC.sendbreak:VAL"></span>
`sendbreak (``fd``, ``t``) `  
causes the transmission of a sequence of zero-valued bits to be sent, if the associated terminal is using asynchronous serial data transmission. If `t` is 0, this will send zero-valued bits for at least a quarter second, and no more than half a second. If `t` is not zero, zero-valued bits are transmitted for an implementation-defined period of time.

<span id="SIG:POSIX_TTY.TC.drain:VAL"></span>
`drain ``fd`` `  
waits for all output written on `fd` to be transmitted.

<span id="SIG:POSIX_TTY.TC.flush:VAL"></span>
`flush (``fd``, ``qs``) `  
discards any data written but not transmitted, or received but not read, depending on the value of `qs`.

<span id="SIG:POSIX_TTY.TC.flow:VAL"></span>
`flow (``fd``, ``action``) `  
suspends and restarts transmission or reception of data, depending on the value of `action`.

<span id="SIG:POSIX_TTY.TC.getpgrp:VAL"></span>
`getpgrp ``fd`` `  
returns the process group ID of the foreground process group associated with the terminal attached to `fd`.

<span id="SIG:POSIX_TTY.TC.setpgrp:VAL"></span>
`setpgrp (``fd``, ``pid``) `  
sets the foreground process group ID associated with `fd` to `pid`.

#### Examples

```repl
Posix.TTY.tcgetattr Posix.FileSys.stdin;;
```

#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`Posix`](posix.md#Posix:STR:SPEC), [`Posix.Error`](posix.md#SIG:POSIX.Error:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC)

#### Discussion

The values of type [`speed`](posix-tty.md#SIG:POSIX_TTY.speed:TY:SPEC) defined in this structure specify the standard baud rates with the obvious correspondence, _i.e._, [`b1200`](posix-tty.md#SIG:POSIX_TTY.b1200:VAL:SPEC) is 1200 baud, [`b9600`](posix-tty.md#SIG:POSIX_TTY.b9600:VAL:SPEC) is 9600 baud, etc. The value [`b0`](posix-tty.md#SIG:POSIX_TTY.b0:VAL:SPEC) indicates \`\`hang up.''
