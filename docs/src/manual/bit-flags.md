# <span id="section:0"></span>The `BIT_FLAGS` signature

---

#### Synopsis

<span id="BIT_FLAGS:SIG:SPEC"></span>

```sml
signature BIT_FLAGS (* OPTIONAL *)
```

The `BIT_FLAGS` signature defines a generic set of operations on an abstract representation of system flags. It is typically included as part of the interface of substructures which provide a set of options.

---

#### Interface

<span id="SIG:BIT_FLAGS.flags:TY:SPEC"></span>
<span id="SIG:BIT_FLAGS.toWord:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.fromWord:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.all:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.flags:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.intersect:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.clear:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.allSet:VAL:SPEC"></span>
<span id="SIG:BIT_FLAGS.anySet:VAL:SPEC"></span>

```sml
eqtype flags
val toWord : flags -> SysWord.word
val fromWord : SysWord.word -> flags
val all : flags
val flags : flags list -> flags
val intersect : flags list -> flags
val clear : flags * flags -> flags
val allSet : flags * flags -> bool
val anySet : flags * flags -> bool
```

#### Description

<span id="SIG:BIT_FLAGS.flags:TY"></span>**`eqtype`**` flags`  
This type is the abstract representation of a set of system flags.

<span id="SIG:BIT_FLAGS.toWord:VAL"></span>

### `toWord`

```sml
val toWord : flags -> SysWord.word
```

### `fromWord`

```sml
val fromWord : SysWord.word -> flags
```
These functions convert between the abstract [`flags`](bit-flags.md#SIG:BIT_FLAGS.flags:TY:SPEC) type and a bit-vector that is represented as a system [`word`](word.md#SIG:WORD.word:TY:SPEC). The interpretation of the bits is system-dependent, but follows the C language binding for the host operating system. Note that there is no error checking on the `fromWord` function's argument.


```repl
val opts = Posix.FileSys.O.flags [Posix.FileSys.O.nonblock];
Posix.FileSys.O.toWord opts;; (* system word for nonblock *)
Posix.FileSys.O.fromWord (Posix.FileSys.O.toWord opts) = opts;; (* true *)
```

<span id="SIG:BIT_FLAGS.all:VAL"></span>

### `all`

```sml
val all : flags
```
represents the union of all flags. Note that this may well be a superset of the `flags` value defined in a matching structure. For example, [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC) is used to define the flags specified by the POSIX standard; a POSIX-conforming operating system may provide additional flags that will not be defined in the [`Posix`](posix.md#Posix:STR:SPEC) structure but could be set in the `all` value.


```repl
Posix.FileSys.O.toWord Posix.FileSys.O.all;; (* system word with all bits set *)
```

<span id="SIG:BIT_FLAGS.flags:VAL"></span>

### `flags`

```sml
val flags : flags list -> flags
```
returns a value that represents the union of the flags in the list `l`. The expression `flags []` denotes the empty set.


```repl
Posix.FileSys.O.flags [] = Posix.FileSys.O.flags [];; (* true *)
Posix.FileSys.O.toWord (Posix.FileSys.O.flags [Posix.FileSys.O.nonblock]);; (* system word for nonblock *)
```

<span id="SIG:BIT_FLAGS.intersect:VAL"></span>

### `intersect`

```sml
val intersect : flags list -> flags
```
returns a value that represents the intersection of the sets of flags in the list `l`. The expression `intersect []` denotes `all`.


```repl
Posix.FileSys.O.toWord (Posix.FileSys.O.intersect []);; (* same as O.all *)
Posix.FileSys.O.toWord (Posix.FileSys.O.intersect [Posix.FileSys.O.nonblock, Posix.FileSys.O.append]);; (* intersection word *)
```

<span id="SIG:BIT_FLAGS.clear:VAL"></span>

### `clear`

```sml
val clear : flags * flags -> flags
```
returns the set of those flags in `fl2` that are not set in `fl1`, _i.e._, the set difference `fl2`` \ ``fl1`. It is equivalent to:

fromWord(SysWord.andb(SysWord.notb (toWord fl1), toWord fl2))



```repl
val f = Posix.FileSys.O.flags [Posix.FileSys.O.nonblock];
Posix.FileSys.O.clear (f, Posix.FileSys.O.flags [Posix.FileSys.O.nonblock, Posix.FileSys.O.append]) = Posix.FileSys.O.append;; (* true *)
```

<span id="SIG:BIT_FLAGS.allSet:VAL"></span>

### `allSet`

```sml
val allSet : flags * flags -> bool
```
returns `true` if all of the flags in `fl1` are also in `fl2` (_i.e._, this tests for inclusion of `fl1` in `fl2`).


```repl
Posix.FileSys.O.allSet (Posix.FileSys.O.nonblock, Posix.FileSys.O.flags [Posix.FileSys.O.nonblock, Posix.FileSys.O.append]);; (* true *)
```

<span id="SIG:BIT_FLAGS.anySet:VAL"></span>

### `anySet`

```sml
val anySet : flags * flags -> bool
```
returns `true` if any of the flags in `fl1` is also in `fl2` (_i.e._, this tests for non-empty intersection).

```repl
Posix.FileSys.O.anySet (Posix.FileSys.O.flags [Posix.FileSys.O.nonblock, Posix.FileSys.O.append], Posix.FileSys.O.append);; (* true *)
```


#### See Also

> [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`Posix.Process`](posix.md#SIG:POSIX.Process:STR:SPEC), [`Posix.TTY`](posix.md#SIG:POSIX.TTY:STR:SPEC), [`SysWord`](word.md#SysWord:STR:SPEC), [`Windows`](windows.md#Windows:STR:SPEC)

#### Discussion

The number of distinct flags in an implementation of the `BIT_FLAGS` interface must be less than or equal to the number of bits in the [`SysWord.word`](word.md#SIG:WORD.word:TY:SPEC) type. In addition, `fromWord o toWord` must be the identity function, and `toWord o fromWord` must be equivalent to

fn w =\> SysWord.andb(w, toWord all)
