# <span id="section:0"></span>The `VectorSlice` structure

---

#### Synopsis

<span id="VECTOR_SLICE:SIG:SPEC"></span>
<span id="VectorSlice:STR:SPEC"></span>

```sml
signature VECTOR_SLICE
structure VectorSlice :> VECTOR_SLICE
```

The `VectorSlice` structure provides an abstraction of subvectors for polymorphic vectors. A `slice` value can be viewed as a triple `(``v``, ``i``, ``n``)`, where `v` is the underlying vector, `i` is the starting index, and `n` is the length of the subvector, with the constraint that 0 \<= `i` \<= `i` + `n` \<= \|`v`\|, where \|`v`\| is the length of `v`. Slices provide a convenient notation for specifying and operating on a contiguous subset of elements in a vector.

---

#### Interface

<span id="SIG:VECTOR_SLICE.slice:TY:SPEC"></span>
<span id="SIG:VECTOR_SLICE.length:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.sub:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.full:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.slice:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.subslice:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.base:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.vector:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.concat:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.isEmpty:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.getItem:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.appi:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.app:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.mapi:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.map:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.foldli:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.foldri:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.foldl:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.foldr:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.findi:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.find:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.exists:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.all:VAL:SPEC"></span>
<span id="SIG:VECTOR_SLICE.collate:VAL:SPEC"></span>

```sml
type 'a slice
val length : 'a slice -> int
val sub : 'a slice * int -> 'a
val full : 'a Vector.vector -> 'a slice
val slice : 'a Vector.vector * int * int option -> 'a slice
val subslice : 'a slice * int * int option -> 'a slice
val base : 'a slice -> 'a Vector.vector * int * int
val vector : 'a slice -> 'a Vector.vector
val concat : 'a slice list -> 'a Vector.vector
val isEmpty : 'a slice -> bool
val getItem : 'a slice -> ('a * 'a slice) option
val appi : (int * 'a -> unit) -> 'a slice -> unit
val app : ('a -> unit) -> 'a slice -> unit
val mapi : (int * 'a -> 'b) -> 'a slice -> 'b Vector.vector
val map : ('a -> 'b) -> 'a slice -> 'b Vector.vector
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a slice -> 'b
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a slice -> 'b
val foldl : ('a * 'b -> 'b) -> 'b -> 'a slice -> 'b
val foldr : ('a * 'b -> 'b) -> 'b -> 'a slice -> 'b
val findi : (int * 'a -> bool) -> 'a slice -> (int * 'a) option
val find : ('a -> bool) -> 'a slice -> 'a option
val exists : ('a -> bool) -> 'a slice -> bool
val all : ('a -> bool) -> 'a slice -> bool
val collate : ('a * 'a -> order) -> 'a slice * 'a slice -> order
```

#### Description

<span id="SIG:VECTOR_SLICE.length:VAL"></span>
`length ``sl`` `  
returns \|`sl`\|, the length (_i.e._, number of elements) of the slice.

<span id="SIG:VECTOR_SLICE.sub:VAL"></span>
`sub (``sl``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the slice `sl`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:VECTOR_SLICE.full:VAL"></span>
`full ``vec`` `  
creates a slice representing the entire vector `vec`. It is equivalent to

[slice](vector-slice.md#SIG:VECTOR_SLICE.slice:VAL:SPEC)(`vec`, 0, [NONE](option.md#SIG:OPTION.option:TY:SPEC))


<span id="SIG:VECTOR_SLICE.slice:VAL"></span>
`slice (``vec``, ``i``, ``sz``) `  
creates a slice based on the vector `vec` starting at index `i` of the vector `vec`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the vector, _i.e._, `vec`\[`i`..\|`vec`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`vec`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `vec``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`arr`\| \< `i` + `j`. Note that, if defined, [`slice`](vector-slice.md#SIG:VECTOR_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`vec`\|.

<span id="SIG:VECTOR_SLICE.subslice:VAL"></span>
`subslice (``sl``, ``i``, ``sz``) `  
creates a slice based on the given slice `sl` starting at index `i` of `sl`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the slice, _i.e._, `sl`\[`i`..\|`sl`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`sl`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `sl``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`sl`\| \< `i` + `j`. Note that, if defined, [`slice`](vector-slice.md#SIG:VECTOR_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`sl`\|.

<span id="SIG:VECTOR_SLICE.base:VAL"></span>
`base ``sl`` `  
returns a triple `(``vec``, ``i``, ``n``)` representing the concrete representation of the slice. `vec` is the underlying vector, `i` is the starting index, and `n` is the length of the slice.

<span id="SIG:VECTOR_SLICE.vector:VAL"></span>
`vector ``sl`` `  
generates a vector from the slice `sl`. Specifically, the result is equivalent to

          Vector.tabulate (length `sl`, fn i =\> sub (`sl`, i))


<span id="SIG:VECTOR_SLICE.concat:VAL"></span>
`concat ``l`` `  
is the concatenation of all the slices in `l`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the sum of all the lengths is greater than [`Vector.maxLen`](vector.md#SIG:VECTOR.maxLen:VAL:SPEC).

<span id="SIG:VECTOR_SLICE.isEmpty:VAL"></span>
`isEmpty ``sl`` `  
returns `true` if `sl` has length 0.

<span id="SIG:VECTOR_SLICE.getItem:VAL"></span>
`getItem ``sl`` `  
returns the first item in `sl` and the rest of the slice, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `sl` is empty.

<span id="SIG:VECTOR_SLICE.appi:VAL"></span>
`appi ``f`` ``sl`` `
` app ``f`` ``sl`` `  
These apply the function `f` to the elements of a slice in left to right order (_i.e._, increasing indices). The more general [`appi`](vector-slice.md#SIG:VECTOR_SLICE.appi:VAL:SPEC) function supplies `f` with the index of the corresponding element in the slice. The expression `app ``f`` ``sl` is equivalent to `appi (``f`` o #2) ``sl`.

<span id="SIG:VECTOR_SLICE.mapi:VAL"></span>
`mapi ``f`` ``sl`` `
` map ``f`` ``sl`` `  
These functions generate new vectors by mapping the function `f` from left to right over the argument slice. The more general [`mapi`](vector-slice.md#SIG:VECTOR_SLICE.mapi:VAL:SPEC) function supplies both the element and the element's index in the slice to the function `f`. The first expression is equivalent to:

let
  fun ff (i,a,l) = `f`(i,a)::l
in
  Vector.fromList (rev (foldli ff \[\] `sl`))
end

The latter expression is equivalent to:

      [mapi](vector-slice.md#SIG:VECTOR_SLICE.mapi:VAL:SPEC) (`f` o #2) `sl`


<span id="SIG:VECTOR_SLICE.foldli:VAL"></span>
`foldli ``f`` ``init`` ``sl`` `
` foldri ``f`` ``init`` ``sl`` `
` foldl ``f`` ``init`` ``sl`` `
` foldr ``f`` ``init`` ``sl`` `  
These fold the function `f` over all the elements of a vector slice, using the value `init` as the initial value. The functions [`foldli`](vector-slice.md#SIG:VECTOR_SLICE.foldli:VAL:SPEC) and [`foldl`](vector-slice.md#SIG:VECTOR_SLICE.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](vector-slice.md#SIG:VECTOR_SLICE.foldri:VAL:SPEC) and [`foldr`](vector-slice.md#SIG:VECTOR_SLICE.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](vector-slice.md#SIG:VECTOR_SLICE.foldli:VAL:SPEC) and [`foldri`](vector-slice.md#SIG:VECTOR_SLICE.foldri:VAL:SPEC) supply `f` with the index of the corresponding element in the slice.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``sl` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `sl`

The analogous equivalence holds for [`foldri`](vector-slice.md#SIG:VECTOR_SLICE.foldri:VAL:SPEC) and [`foldr`](vector-slice.md#SIG:VECTOR_SLICE.foldr:VAL:SPEC).

<span id="SIG:VECTOR_SLICE.findi:VAL"></span>
`findi ``f`` ``sl`` `
` find ``f`` ``sl`` `  
These apply `f` to each element of the slice `sl`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](vector-slice.md#SIG:VECTOR_SLICE.findi:VAL:SPEC) also supplies `f` with the index of the element in the slice and, upon finding an entry satisfying the predicate, returns that index with the element.

<span id="SIG:VECTOR_SLICE.exists:VAL"></span>
`exists ``f`` ``sl`` `  
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f``(``x``)` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.

<span id="SIG:VECTOR_SLICE.all:VAL"></span>
`all ``f`` ``sl`` `  
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f``(``x``)` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](vector-slice.md#SIG:VECTOR_SLICE.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f`` ) ``sl``))`.

<span id="SIG:VECTOR_SLICE.collate:VAL"></span>
`collate ``f`` (``sl``, ``sl2``) `  
performs lexicographic comparison of the two slices using the given ordering `f` on elements.

#### Examples

```repl
VectorSlice.slice (Vector.fromList [1, 2, 3], 1, SOME 2);;
```

#### See Also

> [`Array`](array.md#Array:STR:SPEC), [`ArraySlice`](array-slice.md#ArraySlice:STR:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC)
