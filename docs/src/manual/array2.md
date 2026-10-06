# <span id="section:0"></span>The `Array2` structure

---

#### Synopsis

<span id="ARRAY2:SIG:SPEC"></span>
<span id="Array2:STR:SPEC"></span>

```sml
signature ARRAY2 (* OPTIONAL *)
structure Array2 :> ARRAY2 (* OPTIONAL *)
```

The `Array2` structure provides polymorphic mutable 2-dimensional arrays. As with 1-dimensional arrays, these arrays have the equality property that two arrays are equal if they are the same array, _i.e._, created by the same call to a primitive array constructor such as `array`, `fromList`, etc.; otherwise they are not equal. This also holds for arrays of zero length. Thus, the type `ty array` admits equality even if `ty` does not.

The elements of 2-dimensional arrays are indexed by pair of integers `(i,j)` where `i` gives the row index, and `i` gives the column index. As usual, indices start at 0, with increasing indices going from left to right and, in the case of rows, from top to bottom.

---

#### Interface

<span id="SIG:ARRAY2.array:TY:SPEC"></span>
<span id="SIG:ARRAY2.region:TY:SPEC"></span>
<span id="SIG:ARRAY2.traversal:TY:SPEC"></span>
<span id="SIG:ARRAY2.RowMajor:TY:SPEC"></span>
<span id="SIG:ARRAY2.ColMajor:TY:SPEC"></span>
<span id="SIG:ARRAY2.array:VAL:SPEC"></span>
<span id="SIG:ARRAY2.fromList:VAL:SPEC"></span>
<span id="SIG:ARRAY2.tabulate:VAL:SPEC"></span>
<span id="SIG:ARRAY2.sub:VAL:SPEC"></span>
<span id="SIG:ARRAY2.update:VAL:SPEC"></span>
<span id="SIG:ARRAY2.dimensions:VAL:SPEC"></span>
<span id="SIG:ARRAY2.nCols:VAL:SPEC"></span>
<span id="SIG:ARRAY2.nRows:VAL:SPEC"></span>
<span id="SIG:ARRAY2.row:VAL:SPEC"></span>
<span id="SIG:ARRAY2.column:VAL:SPEC"></span>
<span id="SIG:ARRAY2.copy:VAL:SPEC"></span>
<span id="SIG:ARRAY2.appi:VAL:SPEC"></span>
<span id="SIG:ARRAY2.app:VAL:SPEC"></span>
<span id="SIG:ARRAY2.foldi:VAL:SPEC"></span>
<span id="SIG:ARRAY2.fold:VAL:SPEC"></span>
<span id="SIG:ARRAY2.modifyi:VAL:SPEC"></span>
<span id="SIG:ARRAY2.modify:VAL:SPEC"></span>

```sml
eqtype 'a array
type 'a region = {
base : 'a array,
row : int,
col : int,
nrows : int option,
ncols : int option
}
datatype traversal = RowMajor | ColMajor
val array : int * int * 'a -> 'a array
val fromList : 'a list list -> 'a array
val tabulate : traversal -> int * int * (int * int -> 'a) -> 'a array
val sub : 'a array * int * int -> 'a
val update : 'a array * int * int * 'a -> unit
val dimensions : 'a array -> int * int
val nCols : 'a array -> int
val nRows : 'a array -> int
val row : 'a array * int -> 'a Vector.vector
val column : 'a array * int -> 'a Vector.vector
val copy : {
src : 'a region,
dst : 'a array,
dst_row : int,
dst_col : int
} -> unit
val appi : traversal -> (int * int * 'a -> unit) -> 'a region -> unit
val app : traversal -> ('a -> unit) -> 'a array -> unit
val foldi : traversal -> (int * int * 'a * 'b -> 'b) -> 'b -> 'a region -> 'b
val fold : traversal -> ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
val modifyi : traversal -> (int * int * 'a -> 'a) -> 'a region -> unit
val modify : traversal -> ('a -> 'a) -> 'a array -> unit
```

#### Description

<span id="SIG:ARRAY2.region:TY"></span>**`type`**` `_`'a`_` region = {`
`                   base `**`:`**` `_`'a`_` array,`
`                   row `**`:`**` int,`
`                   col `**`:`**` int,`
`                   nrows `**`:`**` int option,`
`                   ncols `**`:`**` int option`
`                 }`  
This type specifies a rectangular subregion of a 2-dimensional array. If `ncols` equals [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``w``)`, with 0 \<= `w`, the region includes only those elements in columns with indices in the range from `col` to `col` + (`w` - 1), inclusively. If `ncols` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the region includes only those elements lying on or to the right of column `col`. A similar interpretation holds for the `row` and `nrows` fields. Thus, the region corresponds to all those elements with position (i,j) such that i lies in the specified range of rows and j lies in the specified range of columns.

A region `reg` is said to be _valid_ if it denotes a legal subarray of its base array. More specifically, `reg` is _valid_ if

> 0 \<= `#row` `reg` \<= `nRows` (#base `reg`)

when `#nrows ``reg`` = NONE`, or

> 0 \<= `#row` `reg` \<= (`#row` `reg`)+`nr` \<= `nRows` (#base `reg`)

when `#nrows ``reg`` = SOME(``nr``)`, and the analogous conditions hold for columns.

<span id="SIG:ARRAY2.traversal:TY"></span>**`datatype`**` traversal = RowMajor | ColMajor`  
This type specifies a way of traversing a region. Specifically, `RowMajor` indicates that, given a region, the rows are traversed from left to right (smallest column index to largest column index), starting with the first row in the region, then the second, and so on until the last row is traversed. `ColMajor` reverses the roles of row and column, traversing the columns from top down (smallest row index to largest row index), starting with the first column, then the second, and so on until the last column is traversed.

<span id="SIG:ARRAY2.array:VAL"></span>

### `array`

```sml
val array : int * int * 'a -> 'a array
```
creates a new array with `r` rows and `c` columns, with each element initialized to the value `init`. If `r` \< 0, `c` \< 0 or the resulting array would be too large, the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.


```repl
Array2.dimensions (Array2.array (0, 0, 0));; (* (0, 0) *)
Array2.dimensions (Array2.array (2, 3, 0));; (* (2, 3) *)
```
<span id="SIG:ARRAY2.fromList:VAL"></span>

### `fromList`

```sml
val fromList : 'a list list -> 'a array
```
creates a new array from a list of a list of elements. The elements should be presented in row major form, _i.e._, `hd ``l` gives the first row, `hd (tl ``l``)` gives the second row, etc. This raises the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if the resulting array would be too large or if the lists in `l` do not all have the same length.


```repl
Array2.dimensions (Array2.fromList []);; (* (0, 0) *)
Array2.dimensions (Array2.fromList [[1, 2], [3, 4]]);; (* (2, 2) *)
```
<span id="SIG:ARRAY2.tabulate:VAL"></span>

### `tabulate`

```sml
val tabulate : traversal -> int * int * (int * int -> 'a) -> 'a array
```
creates a new array with `r` rows and `c` columns, with the (`i`,`j`)<sup>(th)</sup> element initialized to `f`` (``i``,``j``)`. The elements are initialized in the traversal order specified by `trv`. If `r` \< 0, `c` \< 0 or the resulting array would be too large, the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.


```repl
Array2.row (Array2.tabulate Array2.RowMajor (2, 2, fn (i, j) => i + j), 1);; (* vector [1, 2] *)
```
<span id="SIG:ARRAY2.sub:VAL"></span>

### `sub`

```sml
val sub : 'a array * int * int -> 'a
```
returns the (`i`,`j`)<sup>(th)</sup> element of the array `arr`. If `i` \< 0, `j` \< 0, `nRows` `arr` \<= `i`, or `nCols` `arr` \<= `j`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
Array2.sub (Array2.fromList [[1, 2], [3, 4]], 1, 0);; (* 3 *)
```
<span id="SIG:ARRAY2.update:VAL"></span>

### `update`

```sml
val update : 'a array * int * int * 'a -> unit
```
sets the (`i`,`j`)<sup>(th)</sup> element of the array `arr` to `a`. If `i` \< 0, `j` \< 0, `nRows` `arr` \<= `i`, or `nCols` `arr` \<= `j`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
val a = Array2.fromList [[1, 2], [3, 4]];
Array2.update (a, 0, 1, 8);; (* () *)
Array2.sub (a, 0, 1);; (* 8 *)
```
<span id="SIG:ARRAY2.dimensions:VAL"></span>

### `dimensions`

```sml
val dimensions : 'a array -> int * int
```

### `nCols`

```sml
val nCols : 'a array -> int
```

### `nRows`

```sml
val nRows : 'a array -> int
```
These functions return size information concerning an array. `nCols` returns the number of columns, `nRows` returns the number of rows, and `dimension` returns a pair containing the number of rows and the number of columns of the array. The functions `nRows` and `nCols` are respectively equivalent to `#1 o dimensions` and `#2 o dimensions`


```repl
Array2.dimensions (Array2.fromList [[1, 2], [3, 4]]);; (* (2, 2) *)
Array2.nCols (Array2.fromList [[1, 2], [3, 4]]);; (* 2 *)
Array2.nRows (Array2.fromList [[1, 2], [3, 4]]);; (* 2 *)
```
<span id="SIG:ARRAY2.row:VAL"></span>

### `row`

```sml
val row : 'a array * int -> 'a Vector.vector
```
returns row `i` of `arr`. If (`nRows` `arr`) \<= `i` or `i` \< 0, this raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC).


```repl
Array2.row (Array2.fromList [[1, 2], [3, 4]], 0);; (* vector [1, 2] *)
```
<span id="SIG:ARRAY2.column:VAL"></span>

### `column`

```sml
val column : 'a array * int -> 'a Vector.vector
```
returns column `j` of `arr`. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `j` \< 0 or `nCols` `arr` \<= `j`.


```repl
Array2.column (Array2.fromList [[1, 2], [3, 4]], 1);; (* vector [2, 4] *)
```
<span id="SIG:ARRAY2.copy:VAL"></span>

### `copy`

```sml
val copy : {src : 'a region, dst : 'a array, dst_row : int, dst_col : int} -> unit
```
copies the region `src` into the array `dst`, with the element at position `(#row ``src``, #col ``src``)` copied into the destination array at position `(``dst_row``,``dst_col``)`. If the source region is not valid, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised. Similarly, if the derived destination region (the source region `src` translated to (`dst_row`,`dst_col`)) is not valid in `dst`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> The `copy` function must correctly handle the case in which the `#base ``src` and the `dst` arrays are equal, and the source and destination regions overlap.



```repl
val src = Array2.fromList [[1, 2], [3, 4]];
val dst = Array2.array (2, 2, 0);
Array2.copy {src = {base = src, row = 0, col = 0, nrows = SOME 2, ncols = SOME 2}, dst = dst, dst_row = 0, dst_col = 0};; (* () *)
Array2.sub (dst, 1, 1);; (* 4 *)
```
<span id="SIG:ARRAY2.appi:VAL"></span>

### `appi`

```sml
val appi : traversal -> (int * int * 'a -> unit) -> 'a region -> unit
```

### `app`

```sml
val app : traversal -> ('a -> unit) -> 'a array -> unit
```
These functions apply the function `f` to the elements of an array in the order specified by `tr`. The more general [`appi`](array2.md#SIG:ARRAY2.appi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `app` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus, the expression `app ``tr`` ``f`` ``arr` is equivalent to:

```sml
let
  val range = {base = arr, row = 0, col = 0, nrows = NONE, ncols = NONE}
in
  Array2.appi tr (f o #3) range
end
```



```repl
val a = Array2.fromList [[1, 2], [3, 4]];
Array2.appi Array2.RowMajor (fn (i, j, x) => print (Int.toString i ^ "," ^ Int.toString j ^ ":" ^ Int.toString x)) {base = a, row = 0, col = 0, nrows = SOME 2, ncols = SOME 2};; (* () *)
Array2.app Array2.ColMajor (fn x => print (Int.toString x)) a;; (* () *)
```
<span id="SIG:ARRAY2.foldi:VAL"></span>

### `foldi`

```sml
val foldi : traversal -> (int * int * 'a * 'b -> 'b) -> 'b -> 'a region -> 'b
```

### `fold`

```sml
val fold : traversal -> ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
```
These functions fold the function `f` over the elements of an array `arr`, traversing the elements in `tr` order, and using the value `init` as the initial value. The more general [`foldi`](array2.md#SIG:ARRAY2.foldi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `fold` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus, the expression `fold ``tr`` ``f`` ``init`` ``arr` is equivalent to:

```sml
Array2.foldi tr (fn (_, _, a, b) => f (a, b)) init
  {base = arr, row = 0, col = 0, nrows = NONE, ncols = NONE}
```



```repl
val a = Array2.fromList [[1, 2], [3, 4]];
Array2.foldi Array2.RowMajor (fn (i, j, x, acc) => acc + i + j + x) 0 {base = a, row = 0, col = 0, nrows = SOME 2, ncols = SOME 2};; (* 14 *)
Array2.fold Array2.RowMajor (op +) 0 a;; (* 10 *)
```
<span id="SIG:ARRAY2.modifyi:VAL"></span>

### `modifyi`

```sml
val modifyi : traversal -> (int * int * 'a -> 'a) -> 'a region -> unit
```

### `modify`

```sml
val modify : traversal -> ('a -> 'a) -> 'a array -> unit
```
These functions apply the function `f` to the elements of an array in the order specified by `tr`, and replace each element with the result of `f`. The more general [`modifyi`](array2.md#SIG:ARRAY2.modifyi:VAL:SPEC) function applies `f` to the elements of the region `reg` and supplies both the element and the element's coordinates in the base array to the function `f`. If `reg` is not valid, then the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.

The function `modify` applies `f` to the whole array and does not supply the element's coordinates to `f`. Thus, the expression `modify ``tr`` ``f`` ``arr` is equivalent to:

```sml
let
  val range = {base = arr, row = 0, col = 0, nrows = NONE, ncols = NONE}
in
  Array2.modifyi tr (f o #3) range
end
```


```repl
val a = Array2.fromList [[1, 2], [3, 4]];
Array2.modifyi Array2.RowMajor (fn (i, j, x) => x + i + j) {base = a, row = 0, col = 0, nrows = SOME 2, ncols = SOME 2};; (* () *)
Array2.sub (a, 1, 1);; (* 6 *)
Array2.modify Array2.ColMajor (fn x => x * 2) a;; (* () *)
Array2.sub (a, 1, 1);; (* 12 *)
```

#### See Also

> [`Array`](array.md#Array:STR:SPEC), [`MONO_ARRAY2`](mono-array2.md#MONO_ARRAY2:SIG:SPEC)

#### Discussion

Note that the indices passed to argument functions in [`appi`](array2.md#SIG:ARRAY2.appi:VAL:SPEC), [`foldi`](array2.md#SIG:ARRAY2.foldi:VAL:SPEC), and [`modifyi`](array2.md#SIG:ARRAY2.modifyi:VAL:SPEC) are with respect to the underlying matrix and not based on the region. This is different from the convention for the analogous functions on 1-dimensional slices.

> **Rationale:**
>
> It was clear that 2-dimensional arrays needed to be provided, but the interface is fairly rudimentary, largely due to the lack of experience with their uses in SML programs. Thus, we kept regions concrete, as opposed to the `slice` types, their 1-dimensional cousins. In addition, we felt it best, at this time, to avoid picking among the vast number of possible matrix functions.

> **Implementation note:**
>
> Unlike one-dimensional types, the signature for 2-dimensional arrays does not specify any bounds on possible arrays. Implementations should support a total number of elements that is at least as large as the total number of elements in the corresponding single dimension array type.
