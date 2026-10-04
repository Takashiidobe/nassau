# <span id="section:sec:top-level-env"></span>Top-level environment

This chapter describes the standard initial top-level environment, that is, those identifiers available unqualified before the user introduces additional top-level bindings. As special aspects of this environment, infix identifiers and overloading are also discussed.

There are two reasons for including (non-module) identifiers in the top-level environment. The first is convenience. Certain types and values are used so frequently that it would be perverse to force the programmer to always open the containing structures or to use the qualified names. This is particularly true for interactive interfaces, in which notational simplicity and fewer keystrokes are desirable. The second reason is to allow operator overloading.

### <span id="section:1"></span>Modules in the top-level environment

There are no default requirements on which modules will be initially available at top-level for either interactive or batch-oriented sessions. Each implementation may provide its own mechanism for making its various modules available to the user's code. Even the presence of a top-level identifier that is logically defined in a structure (_e.g._, the type [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) is defined in the [`Int`](integer.md#Int:STR:SPEC) structure) is no guarantee that the structure name is in the environment.

### <span id="section:2"></span>Top-level type, exception, and value identifiers

Various types, exceptions, and values are available in the top-level environment without qualification. In particular, everything in the [`General`](general.md#General:STR:SPEC) structure is available.

We note that the special identifiers `=` and `<>`, corresponding to polymorphic equality and inequality, are available in the top-level environment but are not bound in any module.

**\[FLOAT\]** presents the top-level types and their defining structures, if any.

---

**Top-level types**

`eqtype unit`

[`General`](general.md#General:STR:SPEC)

`eqtype int`

[`Int`](integer.md#Int:STR:SPEC)

`eqtype word`

[`Word`](word.md#Word:STR:SPEC)

`type real`

[`Real`](real.md#Real:STR:SPEC)

`eqtype char`

[`Char`](char.md#Char:STR:SPEC)

`eqtype string`

[`String`](string.md#String:STR:SPEC)

`type substring`

[`Substring`](substring.md#Substring:STR:SPEC)

`type exn`

[`General`](general.md#General:STR:SPEC)

`eqtype 'a array`

[`Array`](array.md#Array:STR:SPEC)

`eqtype 'a vector`

[`Vector`](vector.md#Vector:STR:SPEC)

`eqtype 'a ref`

_primitive_

`datatype bool = false | true`

_primitive_

`datatype 'a option = NONE | SOME of 'a`

[`Option`](option.md#Option:STR:SPEC)

`datatype order = LESS | EQUAL | GREATER`

[`General`](general.md#General:STR:SPEC)

`datatype 'a list = nil | :: of ('a * 'a list)`

_primitive_

---

Although the types [`bool`](bool.md#SIG:BOOL.bool:TY:SPEC) and [`list`](list.md#SIG:LIST.list:TY:SPEC) are considered primitive and defined in the top-level environment, for consistency they are also bound in the structures [`Bool`](bool.md#Bool:STR:SPEC) and [`List`](list.md#List:STR:SPEC), respectively.

The next list presents the exception constructors available at top-level. All of these are defined in the [`General`](general.md#General:STR:SPEC) structure, except for [`Option`](option.md#SIG:OPTION.Option:EXN:SPEC), which is defined in the [`Option`](option.md#Option:STR:SPEC) structure, and [`Empty`](list.md#SIG:LIST.Empty:EXN:SPEC), which is defined in the [`List`](list.md#List:STR:SPEC) structure.

---

`exception Bind`

`exception Chr`

`exception Div`

`exception Domain`

`exception Empty`

`exception Fail of string`

`exception Match`

`exception Option`

`exception Overflow`

`exception Size`

`exception Span`

`exception Subscript`

---

The next table presents the non-overloaded functions available at top-level, plus the structure value to which each is bound. Note that the `use` function is special. Although not defined precisely, its intended purpose is to take the pathname of a file and treat the contents of the file as SML source code typed in by the user. It can be used as a simple build mechanism, especially for interactive sessions. Most implementations will provide a more sophisticated build mechanism for larger collections of source files. Implementations are not required to supply a `use` function.

---

`val ! : 'a ref -> 'a`

`General.!`

`val := : 'a ref * 'a -> unit`

`General.:=`

`val @ : ('a list * 'a list) -> 'a list`

`List.@`

`val ^ : string * string -> string`

`String.^`

`val app : ('a -> unit) -> 'a list -> unit`

`List.app`

`val before : 'a * unit -> 'a`

`General.before`

`val ceil : real -> int`

`Real.ceil`

`val chr : int -> char`

`Char.chr`

`val concat : string list -> string`

`String.concat`

`val exnMessage : exn -> string`

`General.exnMessage`

`val exnName : exn -> string`

`General.exnName`

`val explode : string -> char list`

`String.explode`

`val floor : real -> int`

`Real.floor`

`val foldl : ('a*'b->'b)-> 'b -> 'a list -> 'b`

`List.foldl`

`val foldr : ('a*'b->'b)-> 'b -> 'a list -> 'b`

`List.foldr`

`val getOpt : ('a option * 'a) -> 'a`

`Option.getOpt`

`val hd : 'a list -> 'a`

`List.hd`

`val ignore : 'a -> unit`

`General.ignore`

`val implode : char list -> string`

`String.implode`

`val isSome : 'a option -> bool`

`Option.isSome`

`val length : 'a list -> int`

`List.length`

`val map : ('a -> 'b) -> 'a list -> 'b list`

`List.map`

`val not : bool -> bool`

`Bool.not`

`val null : 'a list -> bool`

`List.null`

`val o : ('a->'b) * ('c->'a) -> 'c->'b`

`General.o`

`val ord : char -> int`

`Char.ord`

`val print : string -> unit`

`TextIO.print`

`val real : int -> real`

`Real.fromInt`

`val ref : 'a -> 'a ref`

_primitive_

`val rev : 'a list -> 'a list`

`List.rev`

`val round : real -> int`

`Real.round`

`val size : string -> int`

`String.size`

`val str : char -> string`

`String.str`

`val substring : string * int * int -> string`

`String.substring`

`val tl : 'a list -> 'a list`

`List.tl`

`val trunc : real -> int`

`Real.trunc`

`val use : string -> unit`

_implementation dependent_

`val valOf : 'a option -> 'a`

`Option.valOf`

`val vector : 'a list -> 'a vector`

`Vector.fromList`

---

### <span id="section:3"></span>Overloaded identifiers

The SML Standard Basis includes a fixed set of overloaded identifiers; programmers may not define new overloadings. These identifiers, with their type schemas and default types, are:

---

`val + : ``num`` * ``num`` -> ``num`

`int * int -> int`

`val - : ``num`` * ``num`` -> ``num`

`int * int -> int`

`val * : ``num`` * ``num`` -> ``num`

`int * int -> int`

`val div : ``wordint`` * ``wordint`` -> ``wordint`

`int * int -> int`

`val mod : ``wordint`` * ``wordint`` -> ``wordint`

`int * int -> int`

`val / : ``real`` * ``real`` -> ``real`

`real * real -> real`

`val ~ : ``num`` -> ``num`

`int -> int`

`val abs : ``realint`` -> ``realint`

`int -> int`

`val < : ``numtext`` * ``numtext`` -> bool`

`int * int -> bool`

`val > : ``numtext`` * ``numtext`` -> bool`

`int * int -> bool`

`val <= : ``numtext`` * ``numtext`` -> bool`

`int * int -> bool`

`val >= : ``numtext`` * ``numtext`` -> bool`

`int * int -> bool`

---

where

- _int_ `= {FixedInt.int, Int.int,`[`Int`_`<N>`_`.int`](integer.md#SIG:INTEGER.int:TY:SPEC)`, IntInf.int, LargeInt.int, Position.int}`
- _word_ `= {LargeWord.word, Word.word, Word8.word,`[`Word`_`<N>`_`.word`](word.md#SIG:WORD.word:TY:SPEC)`, SysWord.word}`
- _real_ `= {LargeReal.real, Real.real,`[`Real`_`<N>`_`.real`](real.md#SIG:REAL.real:TY:SPEC)`}`
- _text_ ` = {String.string, Char.char, WideString.string, WideChar.char}`
- _wordint `=` word **union** int_
- _realint `=` real **union** int_
- _num `=` word **union** int **union** real_
- _numtext `=` num **union** text_

The same type must be chosen throughout the entire type schema of an overloaded operator. For example, the function `abs` cannot have type `int -> real`, but only a type like `int -> int`. In addition, we note that [`Int`_`<N>`_`.int`](integer.md#SIG:INTEGER.int:TY:SPEC), [`IntInf.int`](integer.md#SIG:INTEGER.int:TY:SPEC), [`Word`_`<N>`_`.word`](word.md#SIG:WORD.word:TY:SPEC), [`Real`_`<N>`_`.real`](real.md#SIG:REAL.real:TY:SPEC), [`WideString.string`](string.md#SIG:STRING.string:TY:SPEC), and [`WideChar.char`](char.md#SIG:CHAR.char:TY:SPEC) are optional types.

The function identifiers have a default type that is adopted in lieu of any type information supplied by the surrounding context. All overloaded value identifiers default to an `int`-based type except for the operator `/`, whose default type is `real * real -> real`. Thus, the following code would typecheck:

fun f(x,y) = x \<= y
val x = (1 : LargeInt.int)
val y = x + 1
fun g x = x + x before ignore (x + 0w0)

with `f`, `y`, and `g` having types `int * int -> bool`, `LargeInt.int`, and `word -> word`, respectively.

### <span id="section:4"></span>Infix identifiers

The top-level environment has the following infix identifiers:

infix  7  \* / div mod
infix  6  + - ^
infixr 5  :: @
infix  4  = \<\> \> \>= \< \<=
infix  3  := o
infix  0  before

The digit in each row gives the precedence (binding power) of each identifier, so that `+` and `-` bind equally tightly, and both bind more tightly than `::` and `@`. All these identifiers are left-associative (bind more tightly to the left) except `::` and `@`, which are right-associative.

### <span id="section:5"></span>The operating environment

The Basis Library specifies very little about the operating system environment in which SML programs are executed. This gives implementations the widest possible freedom. Programs may be executed as part of an interactive session, as stand-alone executables, or as server processes.

There are a few points, however, where the surrounding environment does impinge on the Basis Library. We summarize these points here.

The [`CommandLine`](command-line.md#CommandLine:STR:SPEC) structure defines functions that return the name and arguments with which a program was invoked. The method for setting these values is entirely up to the implementation. We would expect that if a stand-alone executable is run from a command line, then these values would be determined from the name and arguments specified on that command line.

Implementations may provide a mechanism for taking a function and producing a stand-alone executable. If such a mechanism is provided, the type of the function being exported must be

(string \* string list) -\> OS.Process.status

When the stand-alone executable is invoked, the function should be called with a first argument equal to `CommandLine.name ()` and a second argument equal to `CommandLine.arguments ()`.

The [`OS.Process.getEnv`](os-process.md#SIG:OS_PROCESS.getEnv:VAL:SPEC) function assumes that the environment associates a set of `name-value` pairs with the invocation of a program, where both `name` and `value` are strings. This function returns the value associated with the given name. This is essentially a mechanism for providing global variables, by which the user can provide values that can be used deep within a program. The method for specifying this set is OS-dependent. The set may be empty.

The [`OS.Process.exit`](os-process.md#SIG:OS_PROCESS.exit:VAL:SPEC) and [`OS.Process.terminate`](os-process.md#SIG:OS_PROCESS.terminate:VAL:SPEC) functions return a status value to the environment. The type of this value, and how the environment interprets it, is OS-dependent.

The [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC) function adds an argument function to the actions that are executed when the program reaches a normal termination. A normal termination is a call to [`OS.Process.exit`](os-process.md#SIG:OS_PROCESS.exit:VAL:SPEC), or as defined by the implementation. If a stand-alone executable is created from a function as above, then normal termination occurs when that function returns. We would expect other methods for creating stand-alone executables to behave similarly.

Abnormal terminations include calls to [`OS.Process.terminate`](os-process.md#SIG:OS_PROCESS.terminate:VAL:SPEC), or when a stand-alone executable does not handle a raised exception. The functions registered by [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC) are not evaluated in the event of an abnormal program termination.

Some actions are implicitly registered with [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC), so that they always occur on a normal program termination. These must include the flushing and closing of all open output streams created by the open functions in [`BinIO`](bin-io.md#BinIO:STR:SPEC) and [`TextIO`](text-io.md#TextIO:STR:SPEC), and the flushing (but not closing) of [`TextIO.stdOut`](text-io.md#SIG:TEXT_IO.stdOut:VAL:SPEC) and [`TextIO.stdErr`](text-io.md#SIG:TEXT_IO.stdErr:VAL:SPEC). Although this covers most usual cases, for maximum portability and robustness, code should flush streams explicitly.
