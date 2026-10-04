# <span id="section:0"></span>The `Array` structure

---

#### Synopsis

<span id="ARRAY:SIG:SPEC"></span>
<span id="Array:STR:SPEC"></span>

```sml
signature ARRAY
structure Array :> ARRAY
```

The `Array` structure defines polymorphic arrays, mutable sequences with constant-time access and update.

Arrays have a special equality property: two arrays are equal if they are the same array, _i.e._, created by the same call to a primitive array constructor such as `array`, `fromList`, etc.; otherwise they are not equal. This also holds for arrays of zero length. Thus, the type `ty array` admits equality even if `ty` does not.

---

#### Interface

<span id="SIG:ARRAY.array:TY:SPEC"></span>
<span id="SIG:ARRAY.vector:TY:SPEC"></span>
<span id="SIG:ARRAY.maxLen:VAL:SPEC"></span>
<span id="SIG:ARRAY.array:VAL:SPEC"></span>
<span id="SIG:ARRAY.fromList:VAL:SPEC"></span>
<span id="SIG:ARRAY.tabulate:VAL:SPEC"></span>
<span id="SIG:ARRAY.length:VAL:SPEC"></span>
<span id="SIG:ARRAY.sub:VAL:SPEC"></span>
<span id="SIG:ARRAY.update:VAL:SPEC"></span>
<span id="SIG:ARRAY.vector:VAL:SPEC"></span>
<span id="SIG:ARRAY.copy:VAL:SPEC"></span>
<span id="SIG:ARRAY.copyVec:VAL:SPEC"></span>
<span id="SIG:ARRAY.appi:VAL:SPEC"></span>
<span id="SIG:ARRAY.app:VAL:SPEC"></span>
<span id="SIG:ARRAY.modifyi:VAL:SPEC"></span>
<span id="SIG:ARRAY.modify:VAL:SPEC"></span>
<span id="SIG:ARRAY.foldli:VAL:SPEC"></span>
<span id="SIG:ARRAY.foldri:VAL:SPEC"></span>
<span id="SIG:ARRAY.foldl:VAL:SPEC"></span>
<span id="SIG:ARRAY.foldr:VAL:SPEC"></span>
<span id="SIG:ARRAY.findi:VAL:SPEC"></span>
<span id="SIG:ARRAY.find:VAL:SPEC"></span>
<span id="SIG:ARRAY.exists:VAL:SPEC"></span>
<span id="SIG:ARRAY.all:VAL:SPEC"></span>
<span id="SIG:ARRAY.collate:VAL:SPEC"></span>

```sml
eqtype 'a array = 'a array
type 'a vector = 'a Vector.vector
val maxLen : int
val array : int * 'a -> 'a array
val fromList : 'a list -> 'a array
val tabulate : int * (int -> 'a) -> 'a array
val length : 'a array -> int
val sub : 'a array * int -> 'a
val update : 'a array * int * 'a -> unit
val vector : 'a array -> 'a vector
val copy : {src : 'a array, dst : 'a array, di : int} -> unit
val copyVec : {src : 'a vector, dst : 'a array, di : int} -> unit
val appi : (int * 'a -> unit) -> 'a array -> unit
val app : ('a -> unit) -> 'a array -> unit
val modifyi : (int * 'a -> 'a) -> 'a array -> unit
val modify : ('a -> 'a) -> 'a array -> unit
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
val foldl : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
val foldr : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
val findi : (int * 'a -> bool) -> 'a array -> (int * 'a) option
val find : ('a -> bool) -> 'a array -> 'a option
val exists : ('a -> bool) -> 'a array -> bool
val all : ('a -> bool) -> 'a array -> bool
val collate : ('a * 'a -> order) -> 'a array * 'a array -> order
```

#### Description

<span id="SIG:ARRAY.maxLen:VAL"></span>**`val`**` maxLen `**`:`**` int`  
The maximum length of arrays supported by this implementation. Attempts to create larger arrays will result in the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception being raised.

<span id="SIG:ARRAY.array:VAL"></span>
`array (``n``, ``init``) `  
creates a new array of length `n`; each element is initialized to the value `init`. If `n` \< 0 or [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC) \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:ARRAY.fromList:VAL"></span>
`fromList ``l`` `  
creates a new array from `l`. The length of the array is [`length`](list.md#SIG:LIST.length:VAL:SPEC)` ``l` and the `i`<sup>(th)</sup> element of the array is the `i`<sup>(th)</sup> element of the the list. If the length of the list is greater than [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:ARRAY.tabulate:VAL"></span>
`tabulate (``n``, ``f``) `  
creates an array of `n` elements, where the elements are defined in order of increasing index by applying `f` to the element's index. This is equivalent to the expression:

fromList (List.tabulate (`n`, `f`))

If `n` \< 0 or [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC) \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:ARRAY.length:VAL"></span>
`length ``arr`` `  
returns \|`arr`\|, the length of the array `arr`.

<span id="SIG:ARRAY.sub:VAL"></span>
`sub (``arr``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the array `arr`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:ARRAY.update:VAL"></span>
`update (``arr``, ``i``, ``x``) `  
sets the `i`<sup>(th)</sup> element of the array `arr` to `x`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:ARRAY.vector:VAL"></span>
`vector ``arr`` `  
generates a vector from `arr`. Specifically, the result is equivalent to

          Vector.tabulate (length `arr`, fn i =\> sub (`arr`, i))


<span id="SIG:ARRAY.copy:VAL"></span>
`copy {``src``, ``dst``, ``di``} `
`copyVec {``src``, ``dst``, ``di``}`  
These functions copy the entire array or vector `src` into the array `dst`, with the `i`<sup>(th)</sup> element in `src`, for 0 \<= `i` \< \|`src`\|, being copied to position `di` + `i` in the destination array. If `di` \< 0 or if \|`dst`\| \< `di`+\|`src`\|, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> In `copy`, if `dst` and `src` are equal, we must have `di`` = 0` to avoid an exception, and `copy` is then the identity.


<span id="SIG:ARRAY.appi:VAL"></span>
`appi ``f`` ``arr`` `
` app ``f`` ``arr`` `  
These apply the function `f` to the elements of the array `arr` in order of increasing indices. The more general form [`appi`](array.md#SIG:ARRAY.appi:VAL:SPEC) supplies `f` with the array index of the corresponding element.

<span id="SIG:ARRAY.modifyi:VAL"></span>
`modifyi ``f`` ``arr`` `
` modify ``f`` ``arr`` `  
These apply the function `f` to the elements of the array `arr` in order of increasing indices, and replace each element with the result. The more general [`modifyi`](array.md#SIG:ARRAY.modifyi:VAL:SPEC) supplies `f` with the array index of the corresponding element. The expression `modify ``f`` ``arr` is equivalent to `modifyi (``f`` o #2) ``arr`.

<span id="SIG:ARRAY.foldli:VAL"></span>
`foldli ``f`` ``init`` ``arr`` `
` foldri ``f`` ``init`` ``arr`` `
` foldl ``f`` ``init`` ``arr`` `
` foldr ``f`` ``init`` ``arr`` `  
These fold the function `f` over all the elements of the array `arr`, using the value `init` as the initial value. The functions [`foldli`](array.md#SIG:ARRAY.foldli:VAL:SPEC) and [`foldl`](array.md#SIG:ARRAY.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) and [`foldr`](array.md#SIG:ARRAY.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](array.md#SIG:ARRAY.foldli:VAL:SPEC) and [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) supply `f` with the array index of the corresponding element.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``arr` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `arr`

The analogous equivalences hold for [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) and [`foldr`](array.md#SIG:ARRAY.foldr:VAL:SPEC).

<span id="SIG:ARRAY.findi:VAL"></span>
`findi ``f`` ``arr`` `
` find ``f`` ``arr`` `  
These functions apply `f` to each element of the array `arr`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](array.md#SIG:ARRAY.findi:VAL:SPEC) also supplies `f` with the array index of the element and, upon finding an entry satisfying the predicate, returns that index with the element.

<span id="SIG:ARRAY.exists:VAL"></span>
`exists ``f`` ``arr`` `  
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

<span id="SIG:ARRAY.all:VAL"></span>
`all ``f`` ``arr`` `  
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](array.md#SIG:ARRAY.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``arr``))`.

<span id="SIG:ARRAY.collate:VAL"></span>
`collate ``f`` (``a1``, ``a2``) `  
performs lexicographic comparison of the two arrays using the given ordering `f` on elements.

#### Examples

```repl
Array.fromList [10, 20, 30];;
```

#### See Also

> [`ArraySlice`](array-slice.md#ArraySlice:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`Vector`](vector.md#Vector:STR:SPEC)
