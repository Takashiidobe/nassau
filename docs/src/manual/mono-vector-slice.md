# <span id="section:0"></span>The `MONO_VECTOR_SLICE` signature

---

#### Synopsis

<span id="MONO_VECTOR_SLICE:SIG:SPEC"></span>
<span id="Word8VectorSlice:STR:SPEC"></span>
<span id="CharVectorSlice:STR:SPEC"></span>
<span id="WideCharVectorSlice:STR:SPEC"></span>
<span id="BoolVectorSlice:STR:SPEC"></span>
<span id="IntVectorSlice:STR:SPEC"></span>
<span id="WordVectorSlice:STR:SPEC"></span>
<span id="RealVectorSlice:STR:SPEC"></span>
<span id="LargeIntVectorSlice:STR:SPEC"></span>
<span id="LargeWordVectorSlice:STR:SPEC"></span>
<span id="LargeRealVectorSlice:STR:SPEC"></span>
<span id="Int{N}VectorSlice:STR:SPEC"></span>
<span id="Word{N}VectorSlice:STR:SPEC"></span>
<span id="Real{N}VectorSlice:STR:SPEC"></span>

```sml
signature MONO_VECTOR_SLICE
structure Word8VectorSlice :> MONO_VECTOR_SLICE
where type vector = Word8Vector.vector
where type elem = Word8.word
structure CharVectorSlice :> MONO_VECTOR_SLICE
where type slice = Substring.substring
where type vector = String.string
where type elem = char
structure WideCharVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type slice = WideSubstring.substring
where type vector = WideString.string
where type elem = WideChar.char
structure BoolVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = BoolVector.vector
where type elem = bool
structure IntVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = IntVector.vector
where type elem = int
structure WordVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = WordVector.vector
where type elem = word
structure RealVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = RealVector.vector
where type elem = real
structure LargeIntVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = LargeIntVector.vector
where type elem = LargeInt.int
structure LargeWordVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = LargeWordVector.vector
where type elem = LargeWord.word
structure LargeRealVectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type vector = LargeRealVector.vector
where type elem = LargeReal.real
structure Int<N>VectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type elem = Int{N}.int
where type vector = Int{N}Vector.vector
structure Word<N>VectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type elem = Word{N}.word
where type vector = Word{N}Vector.vector
structure Real<N>VectorSlice :> MONO_VECTOR_SLICE (* OPTIONAL *)
where type elem = Real{N}.real
where type vector = Real{N}Vector.vector
```

The `MONO_VECTOR_SLICE` signature provides an abstraction of subarrays for monomorphic immutable arrays or vectors. A `slice` value can be viewed as a triple `(``v``, ``i``, ``n``)`, where `v` is the underlying vector, `i` is the starting index, and `n` is the length of the subarray, with the constraint that 0 \<= `i` \<= `i` + `n` \<= \|`v`\|, where \|`v`\| is the length of the vector `v`. Slices provide a convenient notation for specifying and operating on a contiguous subset of elements in a vector.

---

#### Interface

<span id="SIG:MONO_VECTOR_SLICE.elem:TY:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.vector:TY:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.slice:TY:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.length:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.sub:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.full:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.slice:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.subslice:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.base:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.vector:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.concat:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.isEmpty:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.getItem:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.appi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.app:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.mapi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.map:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.foldli:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.foldr:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.foldl:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.foldri:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.findi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.find:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.exists:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.all:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR_SLICE.collate:VAL:SPEC"></span>

```sml
type elem
type vector
type slice
val length : slice -> int
val sub : slice * int -> elem
val full : vector -> slice
val slice : vector * int * int option -> slice
val subslice : slice * int * int option -> slice
val base : slice -> vector * int * int
val vector : slice -> vector
val concat : slice list -> vector
val isEmpty : slice -> bool
val getItem : slice -> (elem * slice) option
val appi : (int * elem -> unit) -> slice -> unit
val app : (elem -> unit) -> slice -> unit
val mapi : (int * elem -> elem) -> slice -> vector
val map : (elem -> elem) -> slice -> vector
val foldli : (int * elem * 'b -> 'b) -> 'b -> slice -> 'b
val foldr : (elem * 'b -> 'b) -> 'b -> slice -> 'b
val foldl : (elem * 'b -> 'b) -> 'b -> slice -> 'b
val foldri : (int * elem * 'b -> 'b) -> 'b -> slice -> 'b
val findi : (int * elem -> bool) -> slice -> (int * elem) option
val find : (elem -> bool) -> slice -> elem option
val exists : (elem -> bool) -> slice -> bool
val all : (elem -> bool) -> slice -> bool
val collate : (elem * elem -> order) -> slice * slice -> order
```

#### Description

<span id="SIG:MONO_VECTOR_SLICE.vector:TY"></span>**`type`**` vector`
The underlying monomorphic vector type. We denote the length of a vector `vec` of type [`vector`](mono-vector-slice.md#SIG:MONO_VECTOR_SLICE.vector:TY:SPEC) by \|`vec`\|.

<span id="SIG:MONO_VECTOR_SLICE.length:VAL"></span>

### `length`

```sml
val length : slice -> int
```
`length ``sl`` `
returns \|`sl`\|, the length (_i.e._, number of elements) of the slice.

```repl
CharVectorSlice.length (CharVectorSlice.full "abc");; (* 3 *)
```
<span id="SIG:MONO_VECTOR_SLICE.sub:VAL"></span>

### `sub`

```sml
val sub : slice * int -> elem
```
`sub (``sl``, ``i``) `
returns the `i`<sup>(th)</sup> element of the slice `sl`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

```repl
CharVectorSlice.sub (CharVectorSlice.full "abc", 1);; (* #"b" *)
```
<span id="SIG:MONO_VECTOR_SLICE.full:VAL"></span>

### `full`

```sml
val full : vector -> slice
```
`full ``vec`` `
creates a slice representing the entire vector `vec`. It is equivalent to

[slice](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC)(`vec`, 0, [NONE](option.md#SIG:OPTION.option:TY:SPEC))

```repl
CharVectorSlice.full "abc";; (* slice of length 3 *)
```
<span id="SIG:MONO_VECTOR_SLICE.slice:VAL"></span>

### `slice`

```sml
val slice : vector * int * int option -> slice
```
`slice (``vec``, ``i``, ``sz``) `
creates a slice based on the vector `vec` starting at index `i` of the vector `vec`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the vector, _i.e._, `vec`\[`i`..\|`vec`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`vec`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `vec``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`vec`\| \< `i` + `j`. Note that, if defined, [`slice`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`vec`\|.

```repl
CharVectorSlice.slice ("abc", 1, SOME 2);; (* slice containing #"b", #"c" *)
```
<span id="SIG:MONO_VECTOR_SLICE.subslice:VAL"></span>

### `subslice`

```sml
val subslice : slice * int * int option -> slice
```
`subslice (``sl``, ``i``, ``sz``) `
creates a slice based on the given slice `sl` starting at index `i` of `sl`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the slice, _i.e._, `sl`\[`i`..\|`sl`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`sl`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `sl``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`sl`\| \< `i` + `j`. Note that, if defined, [`slice`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`sl`\|.

```repl
CharVectorSlice.subslice (CharVectorSlice.full "abcd", 1, SOME 2);; (* slice containing #"b", #"c" *)
```
<span id="SIG:MONO_VECTOR_SLICE.base:VAL"></span>

### `base`

```sml
val base : slice -> vector * int * int
```
`base ``sl`` `
returns a triple `(``vec``, ``i``, ``n``)` representing the concrete representation of the slice. `vec` is the underlying vector, `i` is the starting index, and `n` is the length of the slice.

```repl
CharVectorSlice.base (CharVectorSlice.slice ("abcd", 1, SOME 2));; (* ("abcd", 1, 2) *)
```
<span id="SIG:MONO_VECTOR_SLICE.vector:VAL"></span>

### `vector`

```sml
val vector : slice -> vector
```
`vector ``sl`` `
generates a vector from the slice `sl`. Specifically, if `vec` is the resulting vector, we have \|`vec`\| = \|`sl`\| and, for 0 \<= `i` \< \|`sl`\|, element `i` of `vec` is `sub (``sl``, i)`.

```repl
CharVectorSlice.vector (CharVectorSlice.slice ("abcd", 1, SOME 2));; (* "bc" *)
```
<span id="SIG:MONO_VECTOR_SLICE.concat:VAL"></span>

### `concat`

```sml
val concat : slice list -> vector
```
`concat ``l`` `
is the concatenation of all the vectors in `l`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the sum of all the lengths is greater than the maximum length allowed by vectors of type [`vector`](mono-vector-slice.md#SIG:MONO_VECTOR_SLICE.vector:TY:SPEC).

```repl
CharVectorSlice.concat [CharVectorSlice.full "ab", CharVectorSlice.full "cd"];; (* "abcd" *)
```
<span id="SIG:MONO_VECTOR_SLICE.isEmpty:VAL"></span>

### `isEmpty`

```sml
val isEmpty : slice -> bool
```
`isEmpty ``sl`` `
returns `true` if `sl` has length 0.

```repl
CharVectorSlice.isEmpty (CharVectorSlice.full "");; (* true *)
```
<span id="SIG:MONO_VECTOR_SLICE.getItem:VAL"></span>

### `getItem`

```sml
val getItem : slice -> (elem * slice) option
```
`getItem ``sl`` `
returns the first item in `sl` and the rest of the slice, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `sl` is empty.

```repl
CharVectorSlice.getItem (CharVectorSlice.full "abc");; (* SOME (#"a", slice of "bc") *)
```
<span id="SIG:MONO_VECTOR_SLICE.appi:VAL"></span>

### `appi`

```sml
val appi : (int * elem -> unit) -> slice -> unit
```
`appi ``f`` ``sl`` `
` app ``f`` ``sl`` `
These apply the function `f` to the elements of a slice in left to right order (_i.e._, increasing indices). The more general [`appi`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.appi:VAL:SPEC) function supplies `f` with the index of the corresponding element in the slice. The expression `app ``f`` ``sl` is equivalent to `appi (``f`` o #2) ``sl`.

```repl
CharVectorSlice.appi (fn (i, c) => print (Int.toString i ^ Char.toString c)) (CharVectorSlice.full "ab");; (* prints 0a1b *)
```
<span id="SIG:MONO_VECTOR_SLICE.mapi:VAL"></span>

### `mapi`

```sml
val mapi : (int * elem -> elem) -> slice -> vector
```
`mapi ``f`` ``sl`` `
` map ``f`` ``sl`` `
These functions generate new vectors by mapping the function `f` from left to right over the argument slice. The more general [`mapi`](vector-slice.md#SIG:VECTOR_SLICE.mapi:VAL:SPEC) function supplies both the element and the element's index in the slice to the function `f`. The latter expression is equivalent to:

      mapi (`f` o #2) `sl`

```repl
CharVectorSlice.mapi (fn (_, c) => Char.toUpper c) (CharVectorSlice.full "ab");; (* vector "AB" *)
```
<span id="SIG:MONO_VECTOR_SLICE.foldli:VAL"></span>

### `foldli`

```sml
val foldli : (int * elem * 'b -> 'b) -> 'b -> slice -> 'b
```
`foldli ``f`` ``init`` ``sl`` `
` foldr ``f`` ``init`` ``sl`` `
` foldl ``f`` ``init`` ``sl`` `
` foldri ``f`` ``init`` ``sl`` `
These fold the function `f` over all the elements of a vector slice, using the value `init` as the initial value. The functions [`foldli`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldli:VAL:SPEC) and [`foldl`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldli:VAL:SPEC) and [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) supply `f` with the index of the corresponding element in the slice.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``sl` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `sl`

The analogous equivalence holds for [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldr:VAL:SPEC).

```repl
CharVectorSlice.foldli (fn (i, _, n) => i + n) 0 (CharVectorSlice.full "abc");; (* 3 *)
```
<span id="SIG:MONO_VECTOR_SLICE.findi:VAL"></span>

### `findi`

```sml
val findi : (int * elem -> bool) -> slice -> (int * elem) option
```
`findi ``f`` ``sl`` `
` find ``f`` ``sl`` `
These apply `f` to each element of the slice `sl`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](mono-vector-slice.md#SIG:MONO_VECTOR_SLICE.findi:VAL:SPEC) also supplies `f` with the index of the element in the slice and, upon finding an entry satisfying the predicate, returns that index with the element.

```repl
CharVectorSlice.findi (fn (_, c) => c = #"b") (CharVectorSlice.full "abc");; (* SOME (1, #"b") *)
```
<span id="SIG:MONO_VECTOR_SLICE.exists:VAL"></span>

### `exists`

```sml
val exists : (elem -> bool) -> slice -> bool
```
`exists ``f`` ``sl`` `
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

```repl
CharVectorSlice.exists (fn c => c = #"b") (CharVectorSlice.full "abc");; (* true *)
```
<span id="SIG:MONO_VECTOR_SLICE.all:VAL"></span>

### `all`

```sml
val all : (elem -> bool) -> slice -> bool
```
`all ``f`` ``sl`` `
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``sl``))`.

```repl
CharVectorSlice.all Char.isLower (CharVectorSlice.full "abc");; (* true *)
```
<span id="SIG:MONO_VECTOR_SLICE.collate:VAL"></span>

### `collate`

```sml
val collate : (elem * elem -> order) -> slice * slice -> order
```
```
`collate ``f`` (``sl``, ``sl2``) `
performs lexicographic comparison of the two slices using the given ordering `f` on elements.

#### See Also

> [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`VectorSlice`](vector-slice.md#VectorSlice:STR:SPEC)

#### Discussion

If an implementation provides a structure matching [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC) for some element type `ty`, it must provide the corresponding monomorphic structure matching [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC) with the vector types in the two structures identified.

```repl
CharVectorSlice.collate Char.compare (CharVectorSlice.full "abc", CharVectorSlice.full "abd");; (* LESS *)
```
