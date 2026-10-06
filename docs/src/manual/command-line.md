# <span id="section:0"></span>The `CommandLine` structure

---

#### Synopsis

<span id="COMMAND_LINE:SIG:SPEC"></span>
<span id="CommandLine:STR:SPEC"></span>

```sml
signature COMMAND_LINE
structure CommandLine :> COMMAND_LINE
```

The `CommandLine` structure provides access to the name and arguments used to invoke the currently running program.

---

#### Interface

<span id="SIG:COMMAND_LINE.name:VAL:SPEC"></span>
<span id="SIG:COMMAND_LINE.arguments:VAL:SPEC"></span>

```sml
val name : unit -> string
val arguments : unit -> string list
```

#### Description

<span id="SIG:COMMAND_LINE.name:VAL"></span>

### `name`

```sml
val name : unit -> string
```
The name used to invoke the current program.


```repl
CommandLine.name ();; (* current program name *)
```

<span id="SIG:COMMAND_LINE.arguments:VAL"></span>

### `arguments`

```sml
val arguments : unit -> string list
```
The argument list used to invoke the current program.

> **Implementation note:**
>
> The arguments returned may be only a subset of the arguments actually supplied by the user, since an implementation's runtime system may consume some of them.


#### Discussion

The precise semantics of the above operations are operating system and implementation-specific. For example, `name` might return a full pathname or just the base name. See also the comment under `arguments`.
