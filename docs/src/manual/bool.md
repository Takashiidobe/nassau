# <span id="section:0"></span>The `Bool` structure

---

#### Synopsis

<span id="BOOL:SIG:SPEC"></span>
<span id="Bool:STR:SPEC"></span>

```sml
signature BOOL
structure Bool :> BOOL
```

The `Bool` structure provides some basic operations on boolean values.

---

#### Interface

<span id="SIG:BOOL.bool:TY:SPEC"></span>
<span id="SIG:BOOL.false:TY:SPEC"></span>
<span id="SIG:BOOL.true:TY:SPEC"></span>
<span id="SIG:BOOL.not:VAL:SPEC"></span>
<span id="SIG:BOOL.toString:VAL:SPEC"></span>
<span id="SIG:BOOL.scan:VAL:SPEC"></span>
<span id="SIG:BOOL.fromString:VAL:SPEC"></span>

```sml
datatype bool = false | true
val not : bool -> bool
val toString : bool -> string
val scan : (char, 'a) StringCvt.reader -> (bool, 'a) StringCvt.reader
val fromString : string -> bool option
```

#### Description

<span id="SIG:BOOL.not:VAL"></span>

### `not`

```sml
val not : bool -> bool
```
returns the logical negation of the boolean value `b`.


```repl
Bool.not false;; (* true *)
```

<span id="SIG:BOOL.toString:VAL"></span>

### `toString`

```sml
val toString : bool -> string
```
returns the string representation of `b`, either `"true"` or `"false"`.


```repl
Bool.toString true;; (* "true" *)
```

<span id="SIG:BOOL.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (bool, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : string -> bool option
```
These scan a character source for a boolean value. The first takes a character stream reader `getc` and a stream `strm`. Ignoring case and initial whitespace, the sequences `"true"` and `"false"` are converted to the corresponding boolean values. On successful scanning of a boolean value, [`scan`](bool.md#SIG:BOOL.scan:VAL:SPEC) returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``b``, ``rest``)`, where `b` is the scanned value and `rest` is the remaining character stream.

The second form scans a boolean from a string `s`. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``b``)` for a scanned value `b`; otherwise it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The function `fromString` is equivalent to [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` scan`.

```repl
Bool.scan Substring.getc (Substring.full "true");; (* SOME (true, empty substring) *)
Bool.fromString "false";; (* SOME false *)
Bool.fromString "perhaps";; (* NONE *)
```


#### See Also

> [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC)

#### Discussion

The [`bool`]() type is considered primitive and is defined in the top-level environment. It is rebound here for consistency.

In addition to the [`not`](bool.md#SIG:BOOL.not:VAL:SPEC) function presented here, the language defines the special operators **`andalso`** and **`orelse`**, which provide short-circuit evaluation of the AND and OR of two boolean expressions. The semantics of strict AND and OR operators, which would evaluate both expressions before applying the operator, are rarely needed and can easily be obtained using the **`andalso`** and **`orelse`** operators.
