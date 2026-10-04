# <span id="section:0"></span>The `MONO_ARRAY_SLICE` signature

---

#### Synopsis

<span id="MONO_ARRAY_SLICE:SIG:SPEC"></span>
<span id="Word8ArraySlice:STR:SPEC"></span>
<span id="CharArraySlice:STR:SPEC"></span>
<span id="WideCharArraySlice:STR:SPEC"></span>
<span id="BoolArraySlice:STR:SPEC"></span>
<span id="IntArraySlice:STR:SPEC"></span>
<span id="WordArraySlice:STR:SPEC"></span>
<span id="RealArraySlice:STR:SPEC"></span>
<span id="LargeIntArraySlice:STR:SPEC"></span>
<span id="LargeWordArraySlice:STR:SPEC"></span>
<span id="LargeRealArraySlice:STR:SPEC"></span>
<span id="Int{N}ArraySlice:STR:SPEC"></span>
<span id="Word{N}ArraySlice:STR:SPEC"></span>
<span id="Real{N}ArraySlice:STR:SPEC"></span>

```sml
signature MONO_ARRAY_SLICE
structure Word8ArraySlice :> MONO_ARRAY_SLICE
where type vector = Word8Vector.vector
where type vector_slice = Word8VectorSlice.slice
where type array = Word8Array.array
where type elem = Word8.word
structure CharArraySlice :> MONO_ARRAY_SLICE
where type vector = CharVector.vector
where type vector_slice = CharVectorSlice.slice
where type array = CharArray.array
where type elem = char
structure WideCharArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = WideCharVector.vector
where type vector_slice = WideCharVectorSlice.slice
where type array = WideCharArray.array
where type elem = WideChar.char
structure BoolArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = BoolVector.vector
where type vector_slice = BoolVectorSlice.slice
where type array = BoolArray.array
where type elem = bool
structure IntArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = IntVector.vector
where type vector_slice = IntVectorSlice.slice
where type array = IntArray.array
where type elem = int
structure WordArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = WordVector.vector
where type vector_slice = WordVectorSlice.slice
where type array = WordArray.array
where type elem = word
structure RealArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = RealVector.vector
where type vector_slice = RealVectorSlice.slice
where type array = RealArray.array
where type elem = real
structure LargeIntArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = LargeIntVector.vector
where type vector_slice = LargeIntVectorSlice.slice
where type array = LargeIntArray.array
where type elem = LargeInt.int
structure LargeWordArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = LargeWordVector.vector
where type vector_slice = LargeWordVectorSlice.slice
where type array = LargeWordArray.array
where type elem = LargeWord.word
structure LargeRealArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = LargeRealVector.vector
where type vector_slice = LargeRealVectorSlice.slice
where type array = LargeRealArray.array
where type elem = LargeReal.real
structure Int<N>ArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = Int{N}Vector.vector
where type vector_slice = Int{N}VectorSlice.slice
where type array = Int{N}Array.array
where type elem = Int{N}.int
structure Word<N>ArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = Word{N}Vector.vector
where type vector_slice = Word{N}VectorSlice.slice
where type array = Word{N}Array.array
where type elem = Word{N}.word
structure Real<N>ArraySlice :> MONO_ARRAY_SLICE (* OPTIONAL *)
where type vector = Real{N}Vector.vector
where type vector_slice = Real{N}VectorSlice.slice
where type array = Real{N}Array.array
where type elem = Real{N}.real
```

The `MONO_ARRAY_SLICE` signature provides an abstraction of subarrays for monomorphic arrays. A `slice` value can be viewed as a triple `(``a``, ``i``, ``n``)`, where `a` is the underlying array, `i` is the starting index, and `n` is the length of the subarray, with the constraint that 0 \<= `i` \<= `i` + `n` \<= \|`a`\|, where \|`a`\| is the length of the array `a`. Slices provide a convenient notation for specifying and operating on a contiguous subset of elements in an array.

---

#### Interface

<span id="SIG:MONO_ARRAY_SLICE.elem:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.array:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.slice:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.vector:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.vector_slice:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.length:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.sub:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.update:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.full:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.subslice:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.base:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.vector:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.copy:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.copyVec:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.isEmpty:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.getItem:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.appi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.app:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.modifyi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.modify:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.foldli:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.foldr:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.foldl:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.findi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.find:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.exists:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.all:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY_SLICE.collate:VAL:SPEC"></span>

```sml
type elem
type array
type slice
type vector
type vector_slice
val length : slice -> int
val sub : slice * int -> elem
val update : slice * int * elem -> unit
val full : array -> slice
val slice : array * int * int option -> slice
val subslice : slice * int * int option -> slice
val base : slice -> array * int * int
val vector : slice -> vector
val copy : {src : slice, dst : array, di : int} -> unit
val copyVec : {src : vector_slice, dst : array, di : int} -> unit
val isEmpty : slice -> bool
val getItem : slice -> (elem * slice) option
val appi : (int * elem -> unit) -> slice -> unit
val app : (elem -> unit) -> slice -> unit
val modifyi : (int * elem -> elem) -> slice -> unit
val modify : (elem -> elem) -> slice -> unit
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

<span id="SIG:MONO_ARRAY_SLICE.array:TY"></span>**`type`**` array`  
The underlying monomorphic array type. We denote the length of an array `arr` of type [`array`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.array:TY:SPEC) by \|`arr`\|.

<span id="SIG:MONO_ARRAY_SLICE.vector:TY"></span>**`type`**` vector`  
The underlying monomorphic vector type. We denote the length of a vector `vec` of type [`vector`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.vector:TY:SPEC) by \|`vec`\|.

<span id="SIG:MONO_ARRAY_SLICE.vector_slice:TY"></span>**`type`**` vector_slice`  
Slices of the monomorphic [`vector`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.vector:TY:SPEC) type.

<span id="SIG:MONO_ARRAY_SLICE.length:VAL"></span>
`length ``sl`` `  
returns \|`sl`\|, the length (_i.e._, number of elements) of the slice.

<span id="SIG:MONO_ARRAY_SLICE.sub:VAL"></span>
`sub (``sl``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the slice `sl`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY_SLICE.update:VAL"></span>
`update (``sl``, ``i``, ``a``) `  
sets the `i`<sup>(th)</sup> element of the slice `sl` to `a`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY_SLICE.full:VAL"></span>
`full ``arr`` `  
creates a slice representing the entire array `arr`. It is equivalent to [`slice`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC)`(``arr``, 0, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`)`.

<span id="SIG:MONO_ARRAY_SLICE.slice:VAL"></span>
`slice (``arr``, ``i``, ``sz``) `  
creates a slice based on the array `arr` starting at index `i` of the array `arr`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the array, _i.e._, `arr`\[`i`..\|`arr`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`arr`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `arr``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`arr`\| \< `i` + `j`. Note that, if defined, [`slice`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`arr`\|.

<span id="SIG:MONO_ARRAY_SLICE.subslice:VAL"></span>
`subslice (``sl``, ``i``, ``sz``) `  
creates a slice based on the given slice `sl` starting at index `i` of `sl`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the slice, _i.e._, `sl`\[`i`..\|`sl`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`sl`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `sl``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`sl`\| \< `i` + `j`. Note that, if defined, [`slice`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`sl`\|.

<span id="SIG:MONO_ARRAY_SLICE.base:VAL"></span>
`base ``sl`` `  
returns a triple `(``arr``, ``i``, ``n``)` representing the concrete representation of the slice. `arr` is the underlying array, `i` is the starting index, and `n` is the length of the slice.

<span id="SIG:MONO_ARRAY_SLICE.vector:VAL"></span>
`vector ``sl`` `  
generates a vector from the slice `sl`. Specifically, if `vec` is the resulting vector, we have \|`vec`\| = `length` `sl` and, for 0 \<= `i` \< `length` `sl`, element `i` of `vec` is `sub (``sl``, i)`.

<span id="SIG:MONO_ARRAY_SLICE.copy:VAL"></span>
`copy {``src``, ``dst``, ``di``} `
`copyVec {``src``, ``dst``, ``di``}`  
These functions copy the given slice into the array `dst`, with element `sub (``src``,``i``)`, for 0 \<= `i` \< \|`src`\|, being copied to position `di` + `i` in the destination array. If `di` \< 0 or if \|`dst`\| \< `di`+\|`src`\|, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> The `copy` function must correctly handle the case in which `dst` and the base array of `src` are equal, and the source and destination slices overlap.


<span id="SIG:MONO_ARRAY_SLICE.isEmpty:VAL"></span>
`isEmpty ``sl`` `  
returns `true` if `sl` has length 0.

<span id="SIG:MONO_ARRAY_SLICE.getItem:VAL"></span>
`getItem ``sl`` `  
returns the first item in `sl` and the rest of the slice, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `sl` is empty.

<span id="SIG:MONO_ARRAY_SLICE.appi:VAL"></span>
`appi ``f`` ``sl`` `
` app ``f`` ``sl`` `  
These apply the function `f` to the elements of a slice in left to right order (_i.e._, increasing indices). The more general [`appi`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.appi:VAL:SPEC) function supplies `f` with the index of the corresponding element in the slice. The expression `app ``f`` ``sl` is equivalent to `appi (``f`` o #2) ``sl`.

<span id="SIG:MONO_ARRAY_SLICE.modifyi:VAL"></span>
`modifyi ``f`` ``sl`` `
` modify ``f`` ``sl`` `  
These apply the function `f` to the elements of an array slice in left to right order (_i.e._, increasing indices), and replace each element with the result. The more general [`modifyi`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.modifyi:VAL:SPEC) supplies `f` with the index of the corresponding element in the slice. The expression `modify ``f`` ``sl` is equivalent to `modifyi (``f`` o #2) ``sl`.

<span id="SIG:MONO_ARRAY_SLICE.foldli:VAL"></span>
`foldli ``f`` ``init`` ``sl`` `
` foldr ``f`` ``init`` ``sl`` `
` foldl ``f`` ``init`` ``sl`` `
` foldri ``f`` ``init`` ``sl`` `  
These fold the function `f` over all the elements of an array slice, using the value `init` as the initial value. The functions [`foldli`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldli:VAL:SPEC) and [`foldl`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldli:VAL:SPEC) and [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) supply `f` with the index of the corresponding element in the slice.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``sl` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `sl`

The analogous equivalence holds for [`foldri`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.foldr:VAL:SPEC).

<span id="SIG:MONO_ARRAY_SLICE.findi:VAL"></span>
`findi ``f`` ``sl`` `
` find ``f`` ``sl`` `  
These apply `f` to each element of the slice `sl`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.findi:VAL:SPEC) also supplies `f` with the index of the element in the slice and, upon finding an entry satisfying the predicate, returns that index with the element.

<span id="SIG:MONO_ARRAY_SLICE.exists:VAL"></span>
`exists ``f`` ``sl`` `  
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

<span id="SIG:MONO_ARRAY_SLICE.all:VAL"></span>
`all ``f`` ``sl`` `  
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](mono-array-slice.md#SIG:MONO_ARRAY_SLICE.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``l``))`.

<span id="SIG:MONO_ARRAY_SLICE.collate:VAL"></span>
`collate ``f`` (``sl``, ``sl2``) `  
performs lexicographic comparison of the two slices using the given ordering `f` on elements.

#### Examples

```repl
Word8ArraySlice.full (Word8Array.array (3, 0w0));;
```

#### See Also

> [`ArraySlice`](array-slice.md#ArraySlice:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

#### Discussion

If an implementation provides a structure matching [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC) for some element type `ty`, it must provide the corresponding monomorphic structures matching the signatures [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), and [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), with the vector, array and vector slice types all respectively identified.
