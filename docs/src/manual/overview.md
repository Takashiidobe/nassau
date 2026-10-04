# <span id="section:0"></span>Overview

This section gives an overview of the organization of the SML Basis Library.

### <span id="section:1"></span>Top-level environment

The top-level environment consists of those type, exception, and value identifiers that can be used without qualification. These identifiers are called _pervasive_. For example, the unqualified type `int` is bound to the type [`Int.int`](integer.md#SIG:INTEGER.int:TY:SPEC) and the function `length` is bound to [`List.length`](list.md#SIG:LIST.length:VAL:SPEC). In addition to the pervasive identifiers, the top-level environment also consists of overloaded identifiers (_e.g._, `+` and `*`) and infix definitions. The top-level environment is described in [**\[Top-level environment\]**](top-level-environment.md#section:sec:top-level-env).

### <span id="section:2"></span>Basic types

Operations of the various basic types (_i.e._, [`bool`](bool.md#SIG:BOOL.bool:TY:SPEC), [`int`](integer.md#SIG:INTEGER.int:TY:SPEC), [`word`](word.md#SIG:WORD.word:TY:SPEC), and [`real`](real.md#SIG:REAL.real:TY:SPEC)) are all provided by the SML Basis Library. Additional support for computing with real numbers is provided by the [`Math`](math.md#Math:STR:SPEC) and [`IEEEReal`](ieee-float.md#IEEEReal:STR:SPEC) structures. In addition, implementations may implement multiple different precisions of integers, words, or reals.

### <span id="section:3"></span>Standard datatypes

The SML Basis Library provides support for basic operations on the standard [`option`](option.md#SIG:OPTION.option:TY:SPEC) and [`list`](list.md#SIG:LIST.list:TY:SPEC) datatypes with the [`Option`](option.md#Option:STR:SPEC), [`List`](list.md#List:STR:SPEC), [`ListPair`](list-pair.md#ListPair:STR:SPEC) structures.

### <span id="section:4"></span>Vectors and arrays

The SML Basis Library supports of mutable _array_ and immutable _vector_ types. In addition to the [`array`](array.md#SIG:ARRAY.array:TY:SPEC) and [`vector`](vector.md#SIG:VECTOR.vector:TY:SPEC) type constructors, there are a variety of monomorphic array and vector types. The monomorphic types provide a more compact representation at the cost of less polymorphism. The Library also defines _slices_ of arrays and vectors, which are an abstraction of contiguous subsequences.

### <span id="section:5"></span>Text

Text processing is supported in the form three basic types: [`char`](char.md#SIG:CHAR.char:TY:SPEC), [`string`](string.md#SIG:STRING.string:TY:SPEC), and [`substring`](substring.md#SIG:SUBSTRING.substring:TY:SPEC). Strings are immutable vectors of characters and substrings are string slices (in fact, the types [`string`](string.md#SIG:STRING.string:TY:SPEC) is just another name for [`CharVector.vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC) and [`substring`](substring.md#SIG:SUBSTRING.substring:TY:SPEC) is another name for [`CharVectorSlice.slice`](mono-vector-slice.md#SIG:MONO_VECTOR_SLICE.slice:TY:SPEC)). The SML Basis Library also provides functions for converting to and from basic types and strings.

### <span id="section:6"></span>Input/output

The SML Basis Library supports both binary and text input/output (I/O) using a three-level I/O stack. At the lowest level, the [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC) interface provides unbuffered I/O on abstract _readers_ and _writers_. The [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) interface provides buffering on top of the primitive readers and writers; it also provides a _functional_ input model that supports arbitrary lookahead. The top-level of I/O support is the [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) interface, which supports dynamically bound streams (_i.e._, one can redirect an imperative I/O stream after it has been opened).

### <span id="section:7"></span>System interfaces

The SML Basis Library provides support for writing portable code that uses common systems services (_e.g._, directory navigation). The [`OS`](os.md#OS:STR:SPEC) structure collects together support for file system operations ([`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC)), low-level I/O ([`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC)), pathnames ([`OS.Path`](os.md#SIG:OS.Path:STR:SPEC)), and process control ([`OS.Process`](os.md#SIG:OS.Process:STR:SPEC)). There is also support for time and date manipulations, as well as interval timing.

### <span id="section:8"></span>Sockets

Network programming using _sockets_ is supported by a collection of optional structures. The [`Socket`](socket.md#Socket:STR:SPEC) structure collects together the various operations for socket control and I/O. Three structures are provided for socket creation: [`INetSock`](inet-sock.md#INetSock:STR:SPEC) for Internet-domain sockets, [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC) for Unix-domain sockets, and [`GenericSock`](generic-sock.md#GenericSock:STR:SPEC) for arbitrary sockets. The [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC), [`NetProtDB`](prot-db.md#NetProtDB:STR:SPEC), and [`NetServDB`](serv-db.md#NetServDB:STR:SPEC) structures provide access to the network database (_e.g._, for hostname lookup).

### <span id="section:9"></span>Unix-specific interfaces

The SML Basis Library supports access to additional system services on Unix systems via the optional [`Unix`](unix.md#Unix:STR:SPEC) and [`Posix`](posix.md#Posix:STR:SPEC) structures.

### <span id="section:10"></span>Microsoft Windows-specific interfaces

The SML Basis Library supports access to additional system services on Microsoft Microsoft Windows systems via the optional [`Windows`](windows.md#Windows:STR:SPEC) structure.

### <span id="section:11"></span>Required components

For an implementation to be compliant with the SML Basis Library specification, it must provide all of the _required_ components. Furthermore, these components must be implemented as defined by specification; extending these interfaces is not permitted.

#### <span id="section:12"></span>Required signatures

The following table lists the signatures that every SML implementation is required to provide:

---

[`ARRAY`](array.md#ARRAY:SIG:SPEC)

[`ARRAY_SLICE`](array-slice.md#ARRAY_SLICE:SIG:SPEC)

[`BIN_IO`](bin-io.md#BIN_IO:SIG:SPEC)

[`BOOL`](bool.md#BOOL:SIG:SPEC)

[`BYTE`](byte.md#BYTE:SIG:SPEC)

[`CHAR`](char.md#CHAR:SIG:SPEC)

[`COMMAND_LINE`](command-line.md#COMMAND_LINE:SIG:SPEC)

[`DATE`](date.md#DATE:SIG:SPEC)

[`GENERAL`](general.md#GENERAL:SIG:SPEC)

[`IEEE_REAL`](ieee-float.md#IEEE_REAL:SIG:SPEC)

[`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC)

[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`IO`](io.md#IO:SIG:SPEC)

[`LIST`](list.md#LIST:SIG:SPEC)

[`LIST_PAIR`](list-pair.md#LIST_PAIR:SIG:SPEC)

[`MATH`](math.md#MATH:SIG:SPEC)

[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`OPTION`](option.md#OPTION:SIG:SPEC)

[`OS`](os.md#OS:SIG:SPEC)

[`OS_FILE_SYS`](os-file-sys.md#OS_FILE_SYS:SIG:SPEC)

[`OS_IO`](os-io.md#OS_IO:SIG:SPEC)

[`OS_PATH`](os-path.md#OS_PATH:SIG:SPEC)

[`OS_PROCESS`](os-process.md#OS_PROCESS:SIG:SPEC)

[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)

[`REAL`](real.md#REAL:SIG:SPEC)

[`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC)

[`STRING`](string.md#STRING:SIG:SPEC)

[`STRING_CVT`](string-cvt.md#STRING_CVT:SIG:SPEC)

[`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC)

[`TEXT`](text.md#TEXT:SIG:SPEC)

[`TEXT_IO`](text-io.md#TEXT_IO:SIG:SPEC)

[`TEXT_STREAM_IO`](text-stream-io.md#TEXT_STREAM_IO:SIG:SPEC)

[`TIME`](time.md#TIME:SIG:SPEC)

[`TIMER`](timer.md#TIMER:SIG:SPEC)

[`VECTOR`](vector.md#VECTOR:SIG:SPEC)

[`VECTOR_SLICE`](vector-slice.md#VECTOR_SLICE:SIG:SPEC)

[`WORD`](word.md#WORD:SIG:SPEC)

---

#### <span id="section:13"></span>Required structures

The following table lists the structures that every SML implementation is required to provide:

---

[`Array`](array.md#Array:STR:SPEC)`:>`[`ARRAY`](array.md#ARRAY:SIG:SPEC)

[`ArraySlice`](array-slice.md#ArraySlice:STR:SPEC)`:>`[`ARRAY_SLICE`](array-slice.md#ARRAY_SLICE:SIG:SPEC)

[`BinIO`](bin-io.md#BinIO:STR:SPEC)`:>`[`BIN_IO`](bin-io.md#BIN_IO:SIG:SPEC)

[`BinPrimIO`](prim-io.md#BinPrimIO:STR:SPEC)`:>`[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)

[`Bool`](bool.md#Bool:STR:SPEC)`:>`[`BOOL`](bool.md#BOOL:SIG:SPEC)

[`Byte`](byte.md#Byte:STR:SPEC)`:>`[`BYTE`](byte.md#BYTE:SIG:SPEC)

[`CharArray`](mono-array.md#CharArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`CharArraySlice`](mono-array-slice.md#CharArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`Char`](char.md#Char:STR:SPEC)`:>`[`CHAR`](char.md#CHAR:SIG:SPEC)

[`CharVector`](mono-vector.md#CharVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`CharVectorSlice`](mono-vector-slice.md#CharVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`CommandLine`](command-line.md#CommandLine:STR:SPEC)`:>`[`COMMAND_LINE`](command-line.md#COMMAND_LINE:SIG:SPEC)

[`Date`](date.md#Date:STR:SPEC)`:>`[`DATE`](date.md#DATE:SIG:SPEC)

[`General`](general.md#General:STR:SPEC)`:>`[`GENERAL`](general.md#GENERAL:SIG:SPEC)

[`IEEEReal`](ieee-float.md#IEEEReal:STR:SPEC)`:>`[`IEEE_REAL`](ieee-float.md#IEEE_REAL:SIG:SPEC)

[`Int`](integer.md#Int:STR:SPEC)`:>`[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`IO`](io.md#IO:STR:SPEC)`:>`[`IO`](io.md#IO:SIG:SPEC)

[`LargeInt`](integer.md#LargeInt:STR:SPEC)`:>`[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`LargeReal`](real.md#LargeReal:STR:SPEC)`:>`[`REAL`](real.md#REAL:SIG:SPEC)

[`LargeWord`](word.md#LargeWord:STR:SPEC)`:>`[`WORD`](word.md#WORD:SIG:SPEC)

[`List`](list.md#List:STR:SPEC)`:>`[`LIST`](list.md#LIST:SIG:SPEC)

[`ListPair`](list-pair.md#ListPair:STR:SPEC)`:>`[`LIST_PAIR`](list-pair.md#LIST_PAIR:SIG:SPEC)

[`Math`](math.md#Math:STR:SPEC)`:>`[`MATH`](math.md#MATH:SIG:SPEC)

[`Option`](option.md#Option:STR:SPEC)`:>`[`OPTION`](option.md#OPTION:SIG:SPEC)

[`OS`](os.md#OS:STR:SPEC)`:>`[`OS`](os.md#OS:SIG:SPEC)

[`Position`](integer.md#Position:STR:SPEC)`:>`[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`Real`](real.md#Real:STR:SPEC)`:>`[`REAL`](real.md#REAL:SIG:SPEC)

[`StringCvt`](string-cvt.md#StringCvt:STR:SPEC)`:>`[`STRING_CVT`](string-cvt.md#STRING_CVT:SIG:SPEC)

[`String`](string.md#String:STR:SPEC)`:>`[`STRING`](string.md#STRING:SIG:SPEC)

[`Substring`](substring.md#Substring:STR:SPEC)`:>`[`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC)

[`TextIO`](text-io.md#TextIO:STR:SPEC)`:>`[`TEXT_IO`](text-io.md#TEXT_IO:SIG:SPEC)

[`TextPrimIO`](prim-io.md#TextPrimIO:STR:SPEC)`:>`[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)

[`Text`](text.md#Text:STR:SPEC)`:>`[`TEXT`](text.md#TEXT:SIG:SPEC)

[`Timer`](timer.md#Timer:STR:SPEC)`:>`[`TIMER`](timer.md#TIMER:SIG:SPEC)

[`Time`](time.md#Time:STR:SPEC)`:>`[`TIME`](time.md#TIME:SIG:SPEC)

[`VectorSlice`](vector-slice.md#VectorSlice:STR:SPEC)`:>`[`VECTOR_SLICE`](vector-slice.md#VECTOR_SLICE:SIG:SPEC)

[`Vector`](vector.md#Vector:STR:SPEC)`:>`[`VECTOR`](vector.md#VECTOR:SIG:SPEC)

[`Word8Array`](mono-array.md#Word8Array:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`Word8ArraySlice`](mono-array-slice.md#Word8ArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`Word8Vector`](mono-vector.md#Word8Vector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`Word8VectorSlice`](mono-vector-slice.md#Word8VectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`Word8`](word.md#Word8:STR:SPEC)`:>`[`WORD`](word.md#WORD:SIG:SPEC)

[`Word`](word.md#Word:STR:SPEC)`:>`[`WORD`](word.md#WORD:SIG:SPEC)

---

### <span id="section:14"></span>Optional components

In addition to the required components, an implementation may choose to provide some optional components. While these components are optional, if present, they must be implemented as defined by specification; extending these interfaces is not permitted.

#### <span id="section:15"></span>Optional signatures

The following table lists the optional signatures that an SML implementation may choose to provide:

---

[`ARRAY2`](array2.md#ARRAY2:SIG:SPEC)

[`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC)

[`GENERIC_SOCK`](generic-sock.md#GENERIC_SOCK:SIG:SPEC)

[`INET_SOCK`](inet-sock.md#INET_SOCK:SIG:SPEC)

[`INT_INF`](int-inf.md#INT_INF:SIG:SPEC)

[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`NET_HOST_DB`](net-host-db.md#NET_HOST_DB:SIG:SPEC)

[`NET_PROT_DB`](prot-db.md#NET_PROT_DB:SIG:SPEC)

[`NET_SERV_DB`](serv-db.md#NET_SERV_DB:SIG:SPEC)

[`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

[`PACK_WORD`](pack-word.md#PACK_WORD:SIG:SPEC)

[`POSIX`](posix.md#POSIX:SIG:SPEC)

[`POSIX_ERROR`](posix-error.md#POSIX_ERROR:SIG:SPEC)

[`POSIX_FILE_SYS`](posix-file-sys.md#POSIX_FILE_SYS:SIG:SPEC)

[`POSIX_IO`](posix-io.md#POSIX_IO:SIG:SPEC)

[`POSIX_PROC_ENV`](posix-proc-env.md#POSIX_PROC_ENV:SIG:SPEC)

[`POSIX_PROCESS`](posix-process.md#POSIX_PROCESS:SIG:SPEC)

[`POSIX_SIGNAL`](posix-signal.md#POSIX_SIGNAL:SIG:SPEC)

[`POSIX_SYS_DB`](posix-sys-db.md#POSIX_SYS_DB:SIG:SPEC)

[`POSIX_TTY`](posix-tty.md#POSIX_TTY:SIG:SPEC)

[`SOCKET`](socket.md#SOCKET:SIG:SPEC)

[`UNIX`](unix.md#UNIX:SIG:SPEC)

[`UNIX_SOCK`](unix-sock.md#UNIX_SOCK:SIG:SPEC)

[`WINDOWS`](windows.md#WINDOWS:SIG:SPEC)

---

#### <span id="section:16"></span>Optional structures

The following table lists the optional structures that an SML implementation may choose to provide:

---

[`Array2`](array2.md#Array2:STR:SPEC)`:>`[`ARRAY2`](array2.md#ARRAY2:SIG:SPEC)

[`BoolArray`](mono-array.md#BoolArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`BoolArray2`](mono-array2.md#BoolArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`BoolArraySlice`](mono-array-slice.md#BoolArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`BoolVector`](mono-vector.md#BoolVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`BoolVectorSlice`](mono-vector-slice.md#BoolVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`CharArray2`](mono-array2.md#CharArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`FixedInt`](integer.md#FixedInt:STR:SPEC)`:>`[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`GenericSock`](generic-sock.md#GenericSock:STR:SPEC)`:>`[`GENERIC_SOCK`](generic-sock.md#GENERIC_SOCK:SIG:SPEC)

[`INetSock`](inet-sock.md#INetSock:STR:SPEC)`:>`[`INET_SOCK`](inet-sock.md#INET_SOCK:SIG:SPEC)

[`IntArray`](mono-array.md#IntArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`IntArray2`](mono-array2.md#IntArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`IntArraySlice`](mono-array-slice.md#IntArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`IntVector`](mono-vector.md#IntVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`IntVectorSlice`](mono-vector-slice.md#IntVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`Int`_`<N>`_`Array`](mono-array.md#Int%7BN%7DArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`Int`_`<N>`_`Array2`](mono-array2.md#Int%7BN%7DArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`Int`_`<N>`_`ArraySlice`](mono-array-slice.md#Int%7BN%7DArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`Int`_`<N>`_](integer.md#Int%7BN%7D:STR:SPEC)`:>`[`INTEGER`](integer.md#INTEGER:SIG:SPEC)

[`Int`_`<N>`_`Vector`](mono-vector.md#Int%7BN%7DVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`Int`_`<N>`_`VectorSlice`](mono-vector-slice.md#Int%7BN%7DVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`IntInf`](int-inf.md#IntInf:STR:SPEC)`:>`[`INT_INF`](int-inf.md#INT_INF:SIG:SPEC)

[`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC)`:>`[`NET_HOST_DB`](net-host-db.md#NET_HOST_DB:SIG:SPEC)

[`NetProtDB`](prot-db.md#NetProtDB:STR:SPEC)`:>`[`NET_PROT_DB`](prot-db.md#NET_PROT_DB:SIG:SPEC)

[`NetServDB`](serv-db.md#NetServDB:STR:SPEC)`:>`[`NET_SERV_DB`](serv-db.md#NET_SERV_DB:SIG:SPEC)

[`PackWord`_`<N>`_`Big`](pack-word.md#PackWord%7BN%7DBig:STR:SPEC)`:>`[`PACK_WORD`](pack-word.md#PACK_WORD:SIG:SPEC)

[`PackWord`_`<N>`_`Little`](pack-word.md#PackWord%7BN%7DLittle:STR:SPEC)`:>`[`PACK_WORD`](pack-word.md#PACK_WORD:SIG:SPEC)

[`PackRealBig`](pack-float.md#PackRealBig:STR:SPEC)`:>`[`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

[`PackRealLittle`](pack-float.md#PackRealLittle:STR:SPEC)`:>`[`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

[`PackReal`_`<N>`_`Big`](pack-float.md#PackReal%7BN%7DBig:STR:SPEC)`:>`[`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

[`PackReal`_`<N>`_`Little`](pack-float.md#PackReal%7BN%7DLittle:STR:SPEC)`:>`[`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

[`Posix`](posix.md#Posix:STR:SPEC)`:>`[`POSIX`](posix.md#POSIX:SIG:SPEC)

[`RealArray2`](mono-array2.md#RealArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`RealArray`](mono-array.md#RealArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`RealArraySlice`](mono-array-slice.md#RealArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`RealVector`](mono-vector.md#RealVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`RealVectorSlice`](mono-vector-slice.md#RealVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`Real`_`<N>`_`Array`](mono-array.md#Real%7BN%7DArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`Real`_`<N>`_`Array2`](mono-array2.md#Real%7BN%7DArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`Real`_`<N>`_`ArraySlice`](mono-array-slice.md#Real%7BN%7DArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`Real`_`<N>`_](real.md#Real%7BN%7D:STR:SPEC)`:>`[`REAL`](real.md#REAL:SIG:SPEC)

[`Real`_`<N>`_`Vector`](mono-vector.md#Real%7BN%7DVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`Real`_`<N>`_`VectorSlice`](mono-vector-slice.md#Real%7BN%7DVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`Socket`](socket.md#Socket:STR:SPEC)`:>`[`SOCKET`](socket.md#SOCKET:SIG:SPEC)

[`SysWord`](word.md#SysWord:STR:SPEC)`:>`[`WORD`](word.md#WORD:SIG:SPEC)

[`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)`:>`[`UNIX_SOCK`](unix-sock.md#UNIX_SOCK:SIG:SPEC)

[`Unix`](unix.md#Unix:STR:SPEC)`:>`[`UNIX`](unix.md#UNIX:SIG:SPEC)

[`WideCharArray`](mono-array.md#WideCharArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`WideCharArray2`](mono-array2.md#WideCharArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`WideCharArraySlice`](mono-array-slice.md#WideCharArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`WideChar`](char.md#WideChar:STR:SPEC)`:>`[`CHAR`](char.md#CHAR:SIG:SPEC)

[`WideCharVector`](mono-vector.md#WideCharVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`WideCharVectorSlice`](mono-vector-slice.md#WideCharVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`WideString`](string.md#WideString:STR:SPEC)`:>`[`STRING`](string.md#STRING:SIG:SPEC)

[`WideSubstring`](substring.md#WideSubstring:STR:SPEC)`:>`[`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC)

[`WideTextPrimIO`](prim-io.md#WideTextPrimIO:STR:SPEC)`:>`[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)

[`WideText`](text.md#WideText:STR:SPEC)`:>`[`TEXT`](text.md#TEXT:SIG:SPEC)

[`Windows`](windows.md#Windows:STR:SPEC)`:>`[`WINDOWS`](windows.md#WINDOWS:SIG:SPEC)

[`Word`_`<N>`_`Array`](mono-array.md#Word%7BN%7DArray:STR:SPEC)`:>`[`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC)

[`Word`_`<N>`_`Array2`](mono-array2.md#Word%7BN%7DArray2:STR:SPEC)`:>`[`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

[`Word`_`<N>`_`ArraySlice`](mono-array-slice.md#Word%7BN%7DArraySlice:STR:SPEC)`:>`[`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC)

[`Word`_`<N>`_`Vector`](mono-vector.md#Word%7BN%7DVector:STR:SPEC)`:>`[`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)

[`Word`_`<N>`_`VectorSlice`](mono-vector-slice.md#Word%7BN%7DVectorSlice:STR:SPEC)`:>`[`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

[`Word`_`<N>`_](word.md#Word%7BN%7D:STR:SPEC)`:>`[`WORD`](word.md#WORD:SIG:SPEC)

---

#### <span id="section:17"></span>Optional functors

The following table lists the optional functors that an SML implementation may choose to provide:

---

[`ImperativeIO`](imperative-io-fn.md#ImperativeIO:FCT:SPEC)`:>`[`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC)

[`PrimIO`](prim-io-fn.md#PrimIO:FCT:SPEC)`:>`[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)

[`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC)`:>`[`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC)

---
