# <span id="section:0"></span>The `MONO_ARRAY2` signature

---

#### Synopsis

<span id="MONO_ARRAY2:SIG:SPEC"></span>
<span id="Word8Array2:STR:SPEC"></span>
<span id="CharArray2:STR:SPEC"></span>
<span id="WideCharArray2:STR:SPEC"></span>
<span id="BoolArray2:STR:SPEC"></span>
<span id="IntArray2:STR:SPEC"></span>
<span id="WordArray2:STR:SPEC"></span>
<span id="RealArray2:STR:SPEC"></span>
<span id="LargeIntArray2:STR:SPEC"></span>
<span id="LargeWordArray2:STR:SPEC"></span>
<span id="LargeRealArray2:STR:SPEC"></span>
<span id="Int{N}Array2:STR:SPEC"></span>
<span id="Word{N}Array2:STR:SPEC"></span>
<span id="Real{N}Array2:STR:SPEC"></span>

```sml
signature MONO_ARRAY2 (* OPTIONAL *)
structure Word8Array2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = Word8Vector.vector
where type elem = Word8.word
structure CharArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = CharVector.vector
where type elem = char
structure WideCharArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = WideCharVector.vector
where type elem = WideChar.char
structure BoolArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = BoolVector.vector
where type elem = bool
structure IntArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = IntVector.vector
where type elem = int
structure WordArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = WordVector.vector
where type elem = word
structure RealArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = RealVector.vector
where type elem = real
structure LargeIntArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = LargeIntVector.vector
where type elem = LargeInt.int
structure LargeWordArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = LargeWordVector.vector
where type elem = LargeWord.word
structure LargeRealArray2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = LargeRealVector.vector
where type elem = LargeReal.real
structure Int<N>Array2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = Int{N}Vector.vector
where type elem = Int{N}.int
structure Word<N>Array2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = Word{N}Vector.vector
where type elem = Word{N}.word
structure Real<N>Array2 :> MONO_ARRAY2 (* OPTIONAL *)
where type vector = Real{N}Vector.vector
where type elem = Real{N}.real
```

The `MONO_ARRAY2` signature is a generic interface to mutable 2-dimensional arrays. As usual, arrays have the equality property that two arrays are equal only if they are the same array, _i.e._, created by the same call to a primitive array constructor such as `array`, `fromList`, etc.; otherwise they are not equal. This also holds for arrays of zero length.

The elements of 2-dimensional arrays are indexed by pair of integers `(i,j)` where `i` gives the row index, and `i` gives the column index. As usual, indices start at 0, with increasing indices going from left to right and, in the case of rows, from top to bottom.

---

#### Interface

<span id="SIG:MONO_ARRAY2.array:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY2.elem:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY2.vector:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY2.region:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY2.traversal:TY:SPEC"></span>
<span id="SIG:MONO_ARRAY2.array:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.fromList:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.tabulate:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.sub:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.update:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.dimensions:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.nCols:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.nRows:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.row:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.column:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.copy:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.appi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.app:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.foldi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.fold:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.modifyi:VAL:SPEC"></span>
<span id="SIG:MONO_ARRAY2.modify:VAL:SPEC"></span>

```sml
eqtype array
type elem
type vector
type region = {
base : array,
row : int,
col : int,
nrows : int option,
ncols : int option
}
datatype traversal = datatype Array2.traversal
val array : int * int * elem -> array
val fromList : elem list list -> array
val tabulate : traversal -> int * int * (int * int -> elem) -> array
val sub : array * int * int -> elem
val update : array * int * int * elem -> unit
val dimensions : array -> int * int
val nCols : array -> int
val nRows : array -> int
val row : array * int -> vector
val column : array * int -> vector
val copy : {
src : region,
dst : array,
dst_row : int,
dst_col : int
} -> unit
val appi : traversal -> (int * int * elem -> unit) -> region -> unit
val app : traversal -> (elem -> unit) -> array -> unit
val foldi : traversal -> (int * int * elem * 'b -> 'b) -> 'b -> region -> 'b
val fold : traversal -> (elem * 'b -> 'b) -> 'b -> array -> 'b
val modifyi : traversal -> (int * int * elem -> elem) -> region -> unit
val modify : traversal -> (elem -> elem) -> array -> unit
```

#### Description

<span id="SIG:MONO_ARRAY2.vector:TY"></span>**`type`**` vector`  
The type of one-dimensional immutable vectors of the underlying element type.

<span id="SIG:MONO_ARRAY2.region:TY"></span>**`type`**` region = {`
`                base `**`:`**` array,`
`                row `**`:`**` int,`
`                col `**`:`**` int,`
`                nrows `**`:`**` int option,`
`                ncols `**`:`**` int option`
`              }`  
This type specifies a rectangular subregion of a 2-dimensional array. If `ncols = `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``w``)`, the region includes only those elements in columns with indices in the range from `w` to `col` + (`w` - 1), inclusively. If `ncols = `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the region includes only those elements lying on or to the right of column `col`. A similar interpretation holds for the `row` and `nrows` fields. Thus, the region corresponds to all those elements with position (i,j) such that i lies in the specified range of rows and j lies in the specified range of columns.

A region `reg` is said to be _valid_ if it denotes a legal subarray of its base array. More specifically, `reg` is _valid_ if

> 0 \<= `#row` `reg` \<= `nRows` (#base `reg`)

when `#nrows ``reg`` = NONE`, or

> 0 \<= `#row` `reg` \<= (`#row` `reg`)+`nr` \<= `nRows` (#base `reg`)

when `#nrows ``reg`` = SOME(``nr``)`, and the analogous conditions hold for columns.

<span id="SIG:MONO_ARRAY2.traversal:TY"></span>**`datatype`**` traversal = `**`datatype`**` `[`Array2.traversal`](array2.md#SIG:ARRAY2.traversal:TY:SPEC)  
This type specifies ways of traversing an array or region. For more complete information, see the entry for [`Array2.traversal`](array2.md#SIG:ARRAY2.traversal:TY:SPEC).

<span id="SIG:MONO_ARRAY2.array:VAL"></span>
`array (``r``, ``c``, ``init``) `  
creates a new array with `r` rows and `c` columns, with each element initialized to the value `init`. If `r` \< 0, `c` \< 0 or the resulting array size is too large, the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY2.fromList:VAL"></span>
`fromList ``l`` `  
creates a new array from a list of a list of elements. The elements should be presented in row major form, _i.e._, `hd ``l` gives the first row, `hd (tl ``l``)` gives the second row, etc. It raises the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if the the resulting array size is too large, or if the lists in `l` do not all have the same length.

<span id="SIG:MONO_ARRAY2.tabulate:VAL"></span>
`tabulate tr (``r``, ``c``, ``f``) `  
creates a new array with `r` rows and `c` columns, with the (`i`,`j`)<sup>(th)</sup> element initialized to `f`` (``i``,``j``)`. The elements are initialized in the traversal order specified by `tr`. If `r` \< 0, `c` \< 0 or the resulting array size is too large, the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY2.sub:VAL"></span>
`sub (``arr``, ``i``, ``j``) `  
returns the (`i`,`j`)<sup>(th)</sup> element of the array `arr`. If `i` \< 0, `j` \< 0, `nRows` `arr` \<= `i` or `nCols` `arr` \<= `j`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY2.update:VAL"></span>
`update (``arr``, ``i``, ``j``, ``a``) `  
sets the (`i`,`j`)<sup>(th)</sup> element of the array `arr` to `a`. If `i` \< 0, `j` \< 0, `nRows` `arr` \<= `i` or `nCols` `arr` \<= `j`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

<span id="SIG:MONO_ARRAY2.dimensions:VAL"></span>
`dimensions ``arr`` `
` nCols ``arr`` `
` nRows ``arr`` `  
These return size information concerning the array `arr`. `nCols` returns the number of columns, `nRows` returns the number of rows and `dimension` returns a pair containing the number of rows and columns of the array. The functions `nRows` and `nCols` are respectively equivalent to `#1 o dimensions` and `#2 o dimensions`

<span id="SIG:MONO_ARRAY2.row:VAL"></span>
`row (``arr``, ``i``) `  
returns row `i` of `arr`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `nRows` `arr` \<= `i`.

<span id="SIG:MONO_ARRAY2.column:VAL"></span>
`column (``arr``, ``j``) `  
returns column `j` of `arr`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `j` \< 0 or `nCols` `arr` \<= `j`.

<span id="SIG:MONO_ARRAY2.copy:VAL"></span>
`copy {``src``, ``dst``, ``dst_row``, ``dst_col``} `  
copies the region `src` into the array `dst`, with the (`#row` `src`,`#col` `src`)<sup>(th)</sup> element being copied into the destination array at position (`dst_row`,`dst_col`). If the source region is not valid, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised. Similarly, if the derived destination region (the source region `src` translated to (`dst_row`,`dst_col`)) is not valid in `dst`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> The `copy` function must correctly handle the case in which `src` and `dst` are equal, and the source and destination regions overlap.


<span id="SIG:MONO_ARRAY2.appi:VAL"></span>
`appi ``tr`` ``f`` ``reg`` `
` app ``tr`` ``f`` ``arr`` `  
These apply the function `f` to the elements of an array in the order specified by `tr`. The more general [`appi`](mono-array2.md#SIG:MONO_ARRAY2.appi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `app` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus the expression `app ``tr`` ``f`` ``arr` is equivalent to:

appi `tr` (`f` o #3) (`arr`, {row=0,col=0,nrows=NONE,ncols=NONE})


<span id="SIG:MONO_ARRAY2.foldi:VAL"></span>
`foldi ``tr`` ``f`` ``init`` ``reg`` `
` fold ``tr`` ``f`` ``init`` ``arr`` `  
These fold the function `f` over the elements of an array `arr`, traversing the elements in `tr` order, and using the value `init` as the initial value. The more general [`foldi`](mono-array2.md#SIG:MONO_ARRAY2.foldi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `fold` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus the expression `fold ``tr`` ``f`` ``init`` ``arr` is equivalent to:

      foldi `tr` (fn (\_,\_,a,b) =\> `f` (a,b)) `init` 
            (`arr`, {row=0, col=0, nrows=NONE, ncols=NONE})


<span id="SIG:MONO_ARRAY2.modifyi:VAL"></span>
`modifyi ``tr`` ``f`` ``reg`` `
` modify ``tr`` ``f`` ``arr`` `  
These apply the function `f` to the elements of an array in the order specified by `tr`, and replace each element with the result of `f`. The more general [`modifyi`](mono-array2.md#SIG:MONO_ARRAY2.modifyi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `modify` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus the expression `modify ``f`` ``arr` is equivalent to:

modifyi (`f` o #3) (`arr`, {row=0,col=0,nrows=NONE,ncols=NONE})


#### Examples

```repl
Word8Array2.array (2, 2, 0w0);;
```

#### See Also

> [`Array2`](array2.md#Array2:STR:SPEC)

#### Discussion

If an implementation provides any structure matching `MONO_ARRAY2`, it must also supply the structure [`Array2`](array2.md#Array2:STR:SPEC) and its signature [`ARRAY2`](array2.md#ARRAY2:SIG:SPEC).

Note that the indices passed to argument functions in [`appi`](array2.md#SIG:ARRAY2.appi:VAL:SPEC), [`foldi`](array2.md#SIG:ARRAY2.foldi:VAL:SPEC), and [`modifyi`](array2.md#SIG:ARRAY2.modifyi:VAL:SPEC) are with respect to the underlying matrix and not based on the region. This is different from the convention for the analogous functions on 1-dimensional slices.

> **Implementation note:**
>
> Unlike one-dimensional types, the signature for 2-dimensional arrays does not specify any bounds on possible arrays. Implementations should support a total number of elements that is at least as large as the total number of elements in the corresponding single dimension array type.
