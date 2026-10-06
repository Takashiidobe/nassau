# <span id="section:0"></span>The `ArraySlice` structure

---

#### Synopsis

<span id="ARRAY_SLICE:SIG:SPEC"></span>
<span id="ArraySlice:STR:SPEC"></span>

```sml
signature ARRAY_SLICE
structure ArraySlice :> ARRAY_SLICE
```

The `ArraySlice` structure provides an abstraction of subarrays for polymorphic arrays. A `slice` value can be viewed as a triple `(``a``, ``i``, ``n``)`, where `a` is the underlying array, `i` is the starting index, and `n` is the length of the subarray, with the constraint that 0 \<= `i` \<= `i` + `n` \<= \|`a`\|. Slices provide a convenient notation for specifying and operating on a contiguous subset of elements in an array.

---

#### Interface

<span id="SIG:ARRAY_SLICE.slice:TY:SPEC"></span>
<span id="SIG:ARRAY_SLICE.length:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.sub:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.update:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.full:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.slice:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.subslice:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.base:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.vector:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.copy:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.copyVec:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.isEmpty:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.getItem:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.appi:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.app:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.modifyi:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.modify:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.foldli:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.foldri:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.foldl:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.foldr:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.findi:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.find:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.exists:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.all:VAL:SPEC"></span>
<span id="SIG:ARRAY_SLICE.collate:VAL:SPEC"></span>

```sml
type 'a slice
val length : 'a slice -> int
val sub : 'a slice * int -> 'a
val update : 'a slice * int * 'a -> unit
val full : 'a Array.array -> 'a slice
val slice : 'a Array.array * int * int option -> 'a slice
val subslice : 'a slice * int * int option -> 'a slice
val base : 'a slice -> 'a Array.array * int * int
val vector : 'a slice -> 'a Vector.vector
val copy : {
src : 'a slice,
dst : 'a Array.array,
di : int
} -> unit
val copyVec : {
src : 'a VectorSlice.slice,
dst : 'a Array.array,
di : int
} -> unit
val isEmpty : 'a slice -> bool
val getItem : 'a slice -> ('a * 'a slice) option
val appi : (int * 'a -> unit) -> 'a slice -> unit
val app : ('a -> unit) -> 'a slice -> unit
val modifyi : (int * 'a -> 'a) -> 'a slice -> unit
val modify : ('a -> 'a) -> 'a slice -> unit
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

<span id="SIG:ARRAY_SLICE.length:VAL"></span>

### `length`

```sml
val length : 'a slice -> int
```
returns \|`sl`\|, the length (_i.e._, number of elements) of the slice. This is equivalent to `#3 (base ``sl``)`.


```repl
ArraySlice.length (ArraySlice.slice (Array.fromList [], 0, NONE));; (* 0 *)
ArraySlice.length (ArraySlice.slice (Array.fromList [1, 2, 3], 1, SOME 2));; (* 2 *)
```
<span id="SIG:ARRAY_SLICE.sub:VAL"></span>

### `sub`

```sml
val sub : 'a slice * int -> 'a
```
returns the `i`<sup>(th)</sup> element of the slice `sl`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
ArraySlice.sub (ArraySlice.full (Array.fromList [1, 2, 3]), 1);; (* 2 *)
```
<span id="SIG:ARRAY_SLICE.update:VAL"></span>

### `update`

```sml
val update : 'a slice * int * 'a -> unit
```
sets the `i`<sup>(th)</sup> element of the slice `sl` to `a`. If `i` \< 0 or \|`sl`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
val a = Array.fromList [1, 2, 3];
ArraySlice.update (ArraySlice.full a, 1, 8);; (* () *)
Array.sub (a, 1);; (* 8 *)
```
<span id="SIG:ARRAY_SLICE.full:VAL"></span>

### `full`

```sml
val full : 'a Array.array -> 'a slice
```
creates a slice representing the entire array `arr`. It is equivalent to

[slice](array-slice.md#SIG:ARRAY_SLICE.slice:VAL:SPEC)(`arr`, 0, [NONE](option.md#SIG:OPTION.option:TY:SPEC))



```repl
ArraySlice.length (ArraySlice.full (Array.fromList [1, 2, 3]));; (* 3 *)
```
<span id="SIG:ARRAY_SLICE.slice:VAL"></span>

### `slice`

```sml
val slice : 'a Array.array * int * int option -> 'a slice
```
creates a slice based on the array `arr` starting at index `i` of the array. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the array, _i.e._, `arr`\[`i`..\|`arr`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`arr`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `arr``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`arr`\| \< `i` + `j`. Note that, if defined, [`slice`](array-slice.md#SIG:ARRAY_SLICE.slice:VAL:SPEC) returns an empty slice when `i` = \|`arr`\|.


```repl
val a = Array.fromList [1, 2, 3];
ArraySlice.length (ArraySlice.slice (a, 3, SOME 0));; (* 0 *)
ArraySlice.vector (ArraySlice.slice (a, 1, SOME 2));; (* vector [2, 3] *)
```
<span id="SIG:ARRAY_SLICE.subslice:VAL"></span>

### `subslice`

```sml
val subslice : 'a slice * int * int option -> 'a slice
```
creates a slice based on the given slice `sl` starting at index `i` of `sl`. If `sz` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), the slice includes all of the elements to the end of the slice, _i.e._, `sl`\[`i`..\|`sl`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`sl`\| \< `i`. If `sz` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``j``)`, the slice has length `j`, that is, it corresponds to `sl``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`sl`\| \< `i` + `j`. Note that, if defined, [`subslice`](array-slice.md#SIG:ARRAY_SLICE.subslice:VAL:SPEC) returns an empty slice when `i` = \|`sl`\|.


```repl
val s = ArraySlice.full (Array.fromList [1, 2, 3]);
ArraySlice.vector (ArraySlice.subslice (s, 1, SOME 2));; (* vector [2, 3] *)
```
<span id="SIG:ARRAY_SLICE.base:VAL"></span>

### `base`

```sml
val base : 'a slice -> 'a Array.array * int * int
```
returns a triple `(``arr``, ``i``, ``n``)` representing the concrete representation of the slice. `arr` is the underlying array, `i` is the starting index, and `n` is the length of the slice.


```repl
val a = Array.fromList [1, 2, 3];
val (_, start, len) = ArraySlice.base (ArraySlice.slice (a, 1, SOME 2));
(start, len);; (* (1, 2) *)
```
<span id="SIG:ARRAY_SLICE.vector:VAL"></span>

### `vector`

```sml
val vector : 'a slice -> 'a Vector.vector
```
generates a vector from the slice `sl`. Specifically, the result is equivalent to

          Vector.tabulate (length `sl`, fn i =\> sub (`sl`, i))



```repl
ArraySlice.vector (ArraySlice.slice (Array.fromList [1, 2, 3], 1, SOME 2));; (* vector [2, 3] *)
```
<span id="SIG:ARRAY_SLICE.copy:VAL"></span>

### `copy`

```sml
val copy : {src : 'a slice, dst : 'a Array.array, di : int} -> unit
```

### `copyVec`

```sml
val copyVec : {src : 'a VectorSlice.slice, dst : 'a Array.array, di : int} -> unit
```
These functions copy the given slice into the array `dst`, with the `i`<sup>(th)</sup> element of `src`, for 0 \<= `i` \< \|`src`\|, being copied to position `di` + `i` in the destination array. If `di` \< 0 or if \|`dst`\| \< `di`+\|`src`\|, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> The `copy` function must correctly handle the case in which `dst` and the base array of `src` are equal, and the source and destination slices overlap.



```repl
val dst = Array.array (4, 0);
ArraySlice.copy {src = ArraySlice.slice (Array.fromList [5, 6, 7], 1, SOME 2), dst = dst, di = 1};; (* () *)
Array.vector dst;; (* vector [0, 6, 7, 0] *)
ArraySlice.copyVec {src = VectorSlice.full (Vector.fromList [8, 9]), dst = dst, di = 0};; (* () *)
Array.vector dst;; (* vector [8, 9, 7, 0] *)
```
<span id="SIG:ARRAY_SLICE.isEmpty:VAL"></span>

### `isEmpty`

```sml
val isEmpty : 'a slice -> bool
```
returns `true` if `sl` has length 0.


```repl
ArraySlice.isEmpty (ArraySlice.slice (Array.fromList [], 0, NONE));; (* true *)
ArraySlice.isEmpty (ArraySlice.full (Array.fromList [1]));; (* false *)
```
<span id="SIG:ARRAY_SLICE.getItem:VAL"></span>

### `getItem`

```sml
val getItem : 'a slice -> ('a * 'a slice) option
```
returns the first item in `sl` and the rest of the slice, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `sl` is empty.


```repl
ArraySlice.getItem (ArraySlice.slice (Array.fromList [], 0, NONE));; (* NONE *)
ArraySlice.getItem (ArraySlice.full (Array.fromList [1, 2]));; (* SOME (1, slice [2]) *)
```
<span id="SIG:ARRAY_SLICE.appi:VAL"></span>

### `appi`

```sml
val appi : (int * 'a -> unit) -> 'a slice -> unit
```

### `app`

```sml
val app : ('a -> unit) -> 'a slice -> unit
```
These functions apply the function `f` to the elements of a slice in order of increasing indices. The more general [`appi`](array-slice.md#SIG:ARRAY_SLICE.appi:VAL:SPEC) function supplies `f` with the index of the corresponding element in the slice. The expression `app ``f`` ``sl` is equivalent to `appi (``f`` o #2) ``sl`.


```repl
val s = ArraySlice.full (Array.fromList [2, 3]);
ArraySlice.appi (fn (i, x) => print (Int.toString i ^ ":" ^ Int.toString x)) s;; (* () *)
ArraySlice.app (fn x => print (Int.toString x)) s;; (* () *)
```
<span id="SIG:ARRAY_SLICE.modifyi:VAL"></span>

### `modifyi`

```sml
val modifyi : (int * 'a -> 'a) -> 'a slice -> unit
```

### `modify`

```sml
val modify : ('a -> 'a) -> 'a slice -> unit
```
These functions apply the function `f` to the elements of a slice in order of increasing indices, and replace each element with the result. The more general [`modifyi`](array-slice.md#SIG:ARRAY_SLICE.modifyi:VAL:SPEC) supplies `f` with the index of the corresponding element in the slice. The expression `modify ``f`` ``sl` is equivalent to `modifyi (``f`` o #2) ``sl`.


```repl
val a = Array.fromList [2, 3];
ArraySlice.modifyi (fn (i, x) => i + x) (ArraySlice.full a);; (* () *)
Array.vector a;; (* vector [2, 4] *)
ArraySlice.modify (fn x => x * 2) (ArraySlice.full a);; (* () *)
Array.vector a;; (* vector [4, 8] *)
```
<span id="SIG:ARRAY_SLICE.foldli:VAL"></span>

### `foldli`

```sml
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a slice -> 'b
```

### `foldri`

```sml
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a slice -> 'b
```

### `foldl`

```sml
val foldl : ('a * 'b -> 'b) -> 'b -> 'a slice -> 'b
```

### `foldr`

```sml
val foldr : ('a * 'b -> 'b) -> 'b -> 'a slice -> 'b
```
These functions fold the function `f` over the elements of a slice, using the value `init` as the initial value. The functions [`foldli`](array-slice.md#SIG:ARRAY_SLICE.foldli:VAL:SPEC) and [`foldl`](array-slice.md#SIG:ARRAY_SLICE.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](array-slice.md#SIG:ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](array-slice.md#SIG:ARRAY_SLICE.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](array-slice.md#SIG:ARRAY_SLICE.foldli:VAL:SPEC) and [`foldri`](array-slice.md#SIG:ARRAY_SLICE.foldri:VAL:SPEC) supply `f` with the index of the corresponding element in the slice.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``sl` is equivalent to:

foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `sl`

The analogous equivalence holds for [`foldri`](array-slice.md#SIG:ARRAY_SLICE.foldri:VAL:SPEC) and [`foldr`](array-slice.md#SIG:ARRAY_SLICE.foldr:VAL:SPEC).


```repl
val s = ArraySlice.full (Array.fromList [2, 3]);
ArraySlice.foldli (fn (i, x, acc) => i + x + acc) 0 s;; (* 6 *)
ArraySlice.foldri (fn (i, x, acc) => i + x + acc) 0 s;; (* 6 *)
ArraySlice.foldl (op +) 0 s;; (* 5 *)
ArraySlice.foldr (op +) 0 s;; (* 5 *)
```
<span id="SIG:ARRAY_SLICE.findi:VAL"></span>

### `findi`

```sml
val findi : (int * 'a -> bool) -> 'a slice -> (int * 'a) option
```

### `find`

```sml
val find : ('a -> bool) -> 'a slice -> 'a option
```
These functions apply `f` to each element of the slice `sl`, in order of increasing indices, until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](array-slice.md#SIG:ARRAY_SLICE.findi:VAL:SPEC) also supplies `f` with the index of the element in the slice and, upon finding an entry satisfying the predicate, returns that index with the element.


```repl
val s = ArraySlice.full (Array.fromList [2, 3]);
ArraySlice.findi (fn (_, x) => x > 2) s;; (* SOME (1, 3) *)
ArraySlice.find (fn x => x > 2) s;; (* SOME 3 *)
```
<span id="SIG:ARRAY_SLICE.exists:VAL"></span>

### `exists`

```sml
val exists : ('a -> bool) -> 'a slice -> bool
```
applies `f` to each element `x` of the slice `sl`, in order of increasing indices, until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.


```repl
ArraySlice.exists (fn x => x = 3) (ArraySlice.full (Array.fromList [2, 3]));; (* true *)
```
<span id="SIG:ARRAY_SLICE.all:VAL"></span>

### `all`

```sml
val all : ('a -> bool) -> 'a slice -> bool
```
applies `f` to each element `x` of the slice `sl`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](array-slice.md#SIG:ARRAY_SLICE.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``l``))`.


```repl
ArraySlice.all (fn x => x > 0) (ArraySlice.full (Array.fromList []));; (* true *)
ArraySlice.all (fn x => x > 0) (ArraySlice.full (Array.fromList [2, 3]));; (* true *)
```
<span id="SIG:ARRAY_SLICE.collate:VAL"></span>

### `collate`

```sml
val collate : ('a * 'a -> order) -> 'a slice * 'a slice -> order
```
performs lexicographic comparison of the two slices using the given ordering `f` on elements.

```repl
ArraySlice.collate Int.compare (ArraySlice.full (Array.fromList [1, 2]), ArraySlice.full (Array.fromList [1, 3]));; (* LESS *)
```

#### See Also

> [`Array`](array.md#Array:STR:SPEC), [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC), [`Vector`](vector.md#Vector:STR:SPEC), [`VectorSlice`](vector-slice.md#VectorSlice:STR:SPEC)
