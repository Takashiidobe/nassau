# <span id="section:0"></span>The `MONO_ARRAY` signature

---

#### Synopsis

<span id="MONO_ARRAY:SIG:SPEC"></span>
<span id="Word8Array:STR:SPEC"></span>
<span id="CharArray:STR:SPEC"></span>
<span id="WideCharArray:STR:SPEC"></span>
<span id="BoolArray:STR:SPEC"></span>
<span id="IntArray:STR:SPEC"></span>
<span id="WordArray:STR:SPEC"></span>
<span id="RealArray:STR:SPEC"></span>
<span id="LargeIntArray:STR:SPEC"></span>
<span id="LargeWordArray:STR:SPEC"></span>
<span id="LargeRealArray:STR:SPEC"></span>
<span id="Int{N}Array:STR:SPEC"></span>
<span id="Word{N}Array:STR:SPEC"></span>
<span id="Real{N}Array:STR:SPEC"></span>

```sml
signature MONO_ARRAY
structure Word8Array :> MONO_ARRAY
where type vector = Word8Vector.vector
where type elem = Word8.word
structure CharArray :> MONO_ARRAY
where type vector = CharVector.vector
where type elem = char
structure WideCharArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = WideCharVector.vector
where type elem = WideChar.char
structure BoolArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = BoolVector.vector
where type elem = bool
structure IntArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = IntVector.vector
where type elem = int
structure WordArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = WordVector.vector
where type elem = word
structure RealArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = RealVector.vector
where type elem = real
structure LargeIntArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = LargeIntVector.vector
where type elem = LargeInt.int
structure LargeWordArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = LargeWordVector.vector
where type elem = LargeWord.word
structure LargeRealArray :> MONO_ARRAY (* OPTIONAL *)
where type vector = LargeRealVector.vector
where type elem = LargeReal.real
structure Int<N>Array :> MONO_ARRAY (* OPTIONAL *)
where type vector = Int{N}Vector.vector
where type elem = Int{N}.int
structure Word<N>Array :> MONO_ARRAY (* OPTIONAL *)
where type vector = Word{N}Vector.vector
where type elem = Word{N}.word
structure Real<N>Array :> MONO_ARRAY (* OPTIONAL *)
where type vector = Real{N}Vector.vector
where type elem = Real{N}.real
```

The `MONO_ARRAY` signature is a generic interface to monomorphic arrays, mutable sequences with constant-time access and update. Monomorphic arrays allow more compact representations than the analogous polymorphic arrays over the same element type.

Arrays have a special equality property: two arrays are equal if they are the same array, _i.e._, created by the same call to a primitive array constructor such as `array`, `fromList`, etc.; otherwise they are not equal. This also holds for arrays of zero length.

---

#### Interface

<span id="SIG:MONO_ARRAY.array:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY.elem:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY.vector:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY.maxLen:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.array:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.fromList:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.tabulate:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.length:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.sub:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.update:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.vector:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.copy:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.copyVec:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.appi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.app:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.modifyi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.modify:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.foldli:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.foldri:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.foldl:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.foldr:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.findi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.find:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.exists:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.all:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY.collate:VAL:SPEC"></span>

```sml
eqtype array
type elem
type vector
val maxLen : int
val array : int * elem -> array
val fromList : elem list -> array
val tabulate : int * (int -> elem) -> array
val length : array -> int
val sub : array * int -> elem
val update : array * int * elem -> unit
val vector : array -> vector
val copy : {src : array, dst : array, di : int} -> unit
val copyVec : {src : vector, dst : array, di : int} -> unit
val appi : (int * elem -> unit) -> array -> unit
val app : (elem -> unit) -> array -> unit
val modifyi : (int * elem -> elem) -> array -> unit
val modify : (elem -> elem) -> array -> unit
val foldli : (int * elem * 'b -> 'b) -> 'b -> array -> 'b
val foldri : (int * elem * 'b -> 'b) -> 'b -> array -> 'b
val foldl : (elem * 'b -> 'b) -> 'b -> array -> 'b
val foldr : (elem * 'b -> 'b) -> 'b -> array -> 'b
val findi : (int * elem -> bool) -> array -> (int * elem) option
val find : (elem -> bool) -> array -> elem option
val exists : (elem -> bool) -> array -> bool
val all : (elem -> bool) -> array -> bool
val collate : (elem * elem -> order) -> array * array -> order
```

#### Description

<span id="SIG:MONO_ARRAY.vector:TY"></span>**`type`**` vector`  
The corresponding monomorphic vector type. We denote the length of a vector `vec` of type [`vector`](mono-array.md#SIG:MONO_ARRAY.vector:TY:SPEC) by \|`vec`\|.

<span id="SIG:MONO_ARRAY.maxLen:VAL"></span>**`val`**` maxLen `**`:`**` int`  
The maximum length of arrays supported by this implementation. Attempts to create larger arrays will result in the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception being raised.

<span id="SIG:MONO_ARRAY.array:VAL"></span>
`array (``n``, ``init``) `  
creates a new array of length `n`; each element is initialized to the value `init`. If `n` \< 0 or `maxLen` \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY.fromList:VAL"></span>
`fromList ``l`` `  
creates a new array from `l`, whose length is `length ``l` and with the `i`<sup>(th)</sup> element of `l` used as the `i`<sup>(th)</sup> element of the array. If the length of the list is greater than [`maxLen`](mono-array.md#SIG:MONO_ARRAY.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY.tabulate:VAL"></span>
`tabulate (``n``, ``f``) `  
creates an array of `n` elements, where the elements are defined in order of increasing index by applying `f` to the element's index. This is equivalent to the expression:

fromList (List.tabulate (`n`, `f`))

If `n` \< 0 or `maxLen` \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY.length:VAL"></span>
`length ``arr`` `  
returns \|`arr`\|, the number of elements in the array `arr`.

<span id="SIG:MONO_ARRAY.sub:VAL"></span>
`sub (``arr``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the array `arr`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY.update:VAL"></span>
`update (``arr``, ``i``, ``x``) `  
sets the `i`<sup>(th)</sup> element of the array `arr` to `x`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY.vector:VAL"></span>
`vector ``arr`` `  
generates a vector from `arr`. Specifically, if `vec` is the resulting vector, we have \|`vec`\| = \|`arr`\| and, for 0 \<= `i` \< \|`arr`\|, element `i` of `vec` is `sub (``arr``, i)`.

<span id="SIG:MONO_ARRAY.copy:VAL"></span>
`copy {``src``, ``dst``, ``di``} `
`copyVec {``src``, ``dst``, ``di``}`  
These functions copy the entire array or vector `src` into the array `dst`, with the `i`<sup>(th)</sup> element in `src`, for 0 \<= `i` \< \|`src`\|, being copied to position `di` + `i` in the destination array. If `di` \< 0 or if \|`dst`\| \< `di`+\|`src`\|, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> In `copy`, if `dst` and `src` are equal, we must have `di`` = 0` to avoid an exception, and `copy` is then the identity.


<span id="SIG:MONO_ARRAY.appi:VAL"></span>
`appi ``f`` ``arr`` `
` app ``f`` ``arr`` `  
These apply the function `f` to the elements of an array in left to right order (_i.e._, increasing indices). The more general [`appi`](mono-array.md#SIG:MONO_ARRAY.appi:VAL:SPEC) function supplies both the element and the element's index to the function `f`. The expression `app ``f`` ``arr` is equivalent to:

      appi (`f` o #2) `arr`


<span id="SIG:MONO_ARRAY.modifyi:VAL"></span>
`modifyi ``f`` ``arr`` `
` modify ``f`` ``arr`` `  
These apply the function `f` to the elements of an array in left to right order (_i.e._, increasing indices), and replace each element with the result of applying `f`. The more general [`modifyi`](mono-array.md#SIG:MONO_ARRAY.modifyi:VAL:SPEC) function supplies both the element and the element's index to the function `f`. The expression `modify ``f`` ``arr` is equivalent to:

      modifyi (`f` o #2) `arr`


<span id="SIG:MONO_ARRAY.foldli:VAL"></span>
`foldli ``f`` ``init`` ``arr`` `
` foldri ``f`` ``init`` ``arr`` `
` foldl ``f`` ``init`` ``arr`` `
` foldr ``f`` ``init`` ``arr`` `  
These fold the function `f` over all the elements of an array, using the value `init` as the initial value. The functions [`foldli`](mono-array.md#SIG:MONO_ARRAY.foldli:VAL:SPEC) and [`foldl`](mono-array.md#SIG:MONO_ARRAY.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](mono-array.md#SIG:MONO_ARRAY.foldri:VAL:SPEC) and [`foldr`](mono-array.md#SIG:MONO_ARRAY.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](mono-array.md#SIG:MONO_ARRAY.foldli:VAL:SPEC) and [`foldri`](mono-array.md#SIG:MONO_ARRAY.foldri:VAL:SPEC) supply `f` with the array index of the corresponding element.

The indexed versions could be implemented as:

fun foldli f init seq = let
      val len = length seq
      fun loop (i, b) =
            if i = len then b
            else loop(i+1,f(i,sub(seq,i),b))
      in
        loop(0,init)
      end
fun foldri f init seq = let
      val len = length seq
      fun loop (i, b) =
            if i = ~1 then b
            else loop(i-1,f(i,sub(seq,i),b))
      in
        loop(len-1,init)
      end

The expression `foldl ``f`` ``init`` ``arr` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `arr`

The analogous equivalences hold for [`foldri`](mono-array.md#SIG:MONO_ARRAY.foldri:VAL:SPEC) and [`foldr`](mono-array.md#SIG:MONO_ARRAY.foldr:VAL:SPEC).

<span id="SIG:MONO_ARRAY.findi:VAL"></span>
`findi ``f`` ``arr`` `
` find ``f`` ``arr`` `  
These apply `f` to each element of the array `arr`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](mono-array.md#SIG:MONO_ARRAY.findi:VAL:SPEC) also supplies `f` with the array index of the element and, upon finding an entry satisfying the predicate, returns that index with the element.

<span id="SIG:MONO_ARRAY.exists:VAL"></span>
`exists ``f`` ``arr`` `  
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

<span id="SIG:MONO_ARRAY.all:VAL"></span>
`all ``f`` ``arr`` `  
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](mono-array.md#SIG:MONO_ARRAY.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``arr``))`.

<span id="SIG:MONO_ARRAY.collate:VAL"></span>
`collate ``f`` (``a1``, ``a2``) `  
performs lexicographic comparison of the two arrays using the given ordering `f` on elements.

#### Examples

```repl
Word8Array.fromList [0w1, 0w2, 0w3];;
```

#### See Also

> [`Array`](array.md#Array:STR:SPEC), [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC)

#### Discussion

If an implementation provides a structure matching [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) for some element type `ty`, it must provide the corresponding monomorphic structure matching [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC) with the vector types in the two structures identified.
