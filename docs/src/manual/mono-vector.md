# <span id="section:0"></span>The `MONO_VECTOR` signature

---

#### Synopsis

<span id="MONO_VECTOR:SIG:SPEC"></span>
<span id="Word8Vector:STR:SPEC"></span>
<span id="CharVector:STR:SPEC"></span>
<span id="WideCharVector:STR:SPEC"></span>
<span id="BoolVector:STR:SPEC"></span>
<span id="IntVector:STR:SPEC"></span>
<span id="WordVector:STR:SPEC"></span>
<span id="RealVector:STR:SPEC"></span>
<span id="LargeIntVector:STR:SPEC"></span>
<span id="LargeWordVector:STR:SPEC"></span>
<span id="LargeRealVector:STR:SPEC"></span>
<span id="Int{N}Vector:STR:SPEC"></span>
<span id="Word{N}Vector:STR:SPEC"></span>
<span id="Real{N}Vector:STR:SPEC"></span>

```sml
signature MONO_VECTOR
structure Word8Vector :> MONO_VECTOR
where type elem = Word8.word
structure CharVector :> MONO_VECTOR
where type vector = String.string
where type elem = char
structure WideCharVector :> MONO_VECTOR (* OPTIONAL *)
where type vector = WideString.string
where type elem = WideChar.char
structure BoolVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = bool
structure IntVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = int
structure WordVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = word
structure RealVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = real
structure LargeIntVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = LargeInt.int
structure LargeWordVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = LargeWord.word
structure LargeRealVector :> MONO_VECTOR (* OPTIONAL *)
where type elem = LargeReal.real
structure Int<N>Vector :> MONO_VECTOR (* OPTIONAL *)
where type elem = Int{N}.int
structure Word<N>Vector :> MONO_VECTOR (* OPTIONAL *)
where type elem = Word{N}.word
structure Real<N>Vector :> MONO_VECTOR (* OPTIONAL *)
where type elem = Real{N}.real
```

The `MONO_VECTOR` signature is a generic interface to monomorphic vectors, immutable sequences with constant-time access. Monomorphic vectors allow more compact representations than the analogous polymorphic vectors over the same element type.

---

#### Interface

<span id="SIG:MONO_VECTOR.vector:TY:SPEC"></span>
<span id="SIG:MONO_VECTOR.elem:TY:SPEC"></span>
<span id="SIG:MONO_VECTOR.maxLen:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.fromList:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.tabulate:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.length:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.sub:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.update:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.concat:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.appi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.app:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.mapi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.map:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.foldli:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.foldri:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.foldl:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.foldr:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.findi:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.find:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.exists:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.all:VAL:SPEC"></span>
<span id="SIG:MONO_VECTOR.collate:VAL:SPEC"></span>

```sml
type vector
type elem
val maxLen : int
val fromList : elem list -> vector
val tabulate : int * (int -> elem) -> vector
val length : vector -> int
val sub : vector * int -> elem
val update : vector * int * elem -> vector
val concat : vector list -> vector
val appi : (int * elem -> unit) -> vector -> unit
val app : (elem -> unit) -> vector -> unit
val mapi : (int * elem -> elem) -> vector -> vector
val map : (elem -> elem) -> vector -> vector
val foldli : (int * elem * 'a -> 'a) -> 'a -> vector -> 'a
val foldri : (int * elem * 'a -> 'a) -> 'a -> vector -> 'a
val foldl : (elem * 'a -> 'a) -> 'a -> vector -> 'a
val foldr : (elem * 'a -> 'a) -> 'a -> vector -> 'a
val findi : (int * elem -> bool) -> vector -> (int * elem) option
val find : (elem -> bool) -> vector -> elem option
val exists : (elem -> bool) -> vector -> bool
val all : (elem -> bool) -> vector -> bool
val collate : (elem * elem -> order) -> vector * vector -> order
```

#### Description

<span id="SIG:MONO_VECTOR.maxLen:VAL"></span>**`val`**` maxLen `**`:`**` int`  
The maximum length of vectors supported by this implementation. Attempts to create larger vectors will result in the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception being raised.

<span id="SIG:MONO_VECTOR.fromList:VAL"></span>
`fromList ``l`` `  
creates a new vector from `l`, whose length is `length ``l` and with the `i`<sup>(th)</sup> element of `l` used as the `i`<sup>(th)</sup> element of the vector. If the length of the list is greater than [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_VECTOR.tabulate:VAL"></span>
`tabulate (``n``, ``f``) `  
creates a vector of `n` elements, where the elements are defined in order of increasing index by applying `f` to the element's index. This is equivalent to the expression:

fromList (List.tabulate (`n`, `f`))

If `n` \< 0 or `maxLen` \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_VECTOR.length:VAL"></span>
`length ``vec`` `  
returns \|`vec`\|, the length (_i.e._, the number of elements) of the vector `vec`.

<span id="SIG:MONO_VECTOR.sub:VAL"></span>
`sub (``vec``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the vector `vec`. If `i` \< 0 or \|`vec`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_VECTOR.update:VAL"></span>
`update (``vec``, ``i``, ``x``) `  
returns a new vector, identical to `vec`, except the `i`<sup>(th)</sup> element of `vec` is set to `x`. If `i` \< 0 or \|`vec`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_VECTOR.concat:VAL"></span>
`concat ``l`` `  
returns the vector that is the concatenation of the vectors in the list `l`. If the total length of these vectors exceeds [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_VECTOR.appi:VAL"></span>
`appi ``f`` ``vec`` `
` app ``f`` ``vec`` `  
These apply the function `f` to the elements of a vector in left to right order (_i.e._, increasing indices). The more general [`appi`](mono-vector.md#SIG:MONO_VECTOR.appi:VAL:SPEC) function supplies both the element and the element's index to the function `f`. The expression `app ``f`` ``vec` is equivalent to:

      appi (`f` o #2) `vec`


<span id="SIG:MONO_VECTOR.mapi:VAL"></span>
`mapi ``f`` ``vec`` `
` map ``f`` ``vec`` `  
These functions produce new vectors by mapping the function `f` from left to right over the argument vector. The more general [`mapi`](vector.md#SIG:VECTOR.mapi:VAL:SPEC) function supplies both the element and the element's index to the function `f`. The expression `mapi ``f`` ``vec` is equivalent to:

      fromList (List.map f (foldri (fn (i,a,l) =\> (i,a)::l) \[\] vec))

The expression `map ``f`` ``vec` is equivalent to:

      mapi (`f` o #2) `vec`


<span id="SIG:MONO_VECTOR.foldli:VAL"></span>
`foldli ``f`` ``init`` ``vec`` `
` foldri ``f`` ``init`` ``vec`` `
` foldl ``f`` ``init`` ``vec`` `
` foldr ``f`` ``init`` ``vec`` `  
These fold the function `f` over all the elements of a vector, using the value `init` as the initial value. The functions [`foldli`](mono-vector.md#SIG:MONO_VECTOR.foldli:VAL:SPEC) and [`foldl`](mono-vector.md#SIG:MONO_VECTOR.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](mono-vector.md#SIG:MONO_VECTOR.foldri:VAL:SPEC) and [`foldr`](mono-vector.md#SIG:MONO_VECTOR.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](mono-vector.md#SIG:MONO_VECTOR.foldli:VAL:SPEC) and [`foldri`](mono-vector.md#SIG:MONO_VECTOR.foldri:VAL:SPEC) supply both the element and the element's index to the function `f`.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f` is equivalent to:

      foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`))

A similar relation holds between [`foldr`](mono-vector.md#SIG:MONO_VECTOR.foldr:VAL:SPEC) and [`foldri`](mono-vector.md#SIG:MONO_VECTOR.foldri:VAL:SPEC).

<span id="SIG:MONO_VECTOR.findi:VAL"></span>
`findi ``f`` ``vec`` `
` find ``f`` ``vec`` `  
These apply `f` to each element of the vector `vec`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](mono-vector.md#SIG:MONO_VECTOR.findi:VAL:SPEC) also supplies `f` with the vector index of the element and, upon finding an entry satisfying the predicate, returns that index with the element.

<span id="SIG:MONO_VECTOR.exists:VAL"></span>
`exists ``f`` ``vec`` `  
applies `f` to each element `x` of the vector `vec`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

<span id="SIG:MONO_VECTOR.all:VAL"></span>
`all ``f`` ``vec`` `  
applies `f` to each element `x` of the vector `vec`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](vector.md#SIG:VECTOR.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f`` ) ``vec``))`.

<span id="SIG:MONO_VECTOR.collate:VAL"></span>
`collate ``f`` (``v1``, ``v2``) `  
performs lexicographic comparison of the two vectors using the given ordering `f` on elements.

#### Examples

```repl
Word8Vector.fromList [0w1, 0w2, 0w3];;
```

#### See Also

> [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC), [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC), [`Vector`](vector.md#Vector:STR:SPEC)

#### Discussion

The type [`String.string`](string.md#SIG:STRING.string:TY:SPEC) is identical to `CharVector.vector`.
