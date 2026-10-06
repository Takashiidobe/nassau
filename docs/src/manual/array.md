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

<span id="SIG:ARRAY.maxLen:VAL"></span>

### `maxLen`

```sml
val maxLen : int
```
The maximum length of arrays supported by this implementation. Attempts to create larger arrays will result in the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception being raised.
<span id="SIG:ARRAY.array:VAL"></span>

### `array`

```sml
val array : int * 'a -> 'a array
```
creates a new array of length `n`; each element is initialized to the value `init`. If `n` \< 0 or [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC) \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.


```repl
Array.array (3, 0);; (* array [0, 0, 0] *)
```
<span id="SIG:ARRAY.fromList:VAL"></span>

### `fromList`

```sml
val fromList : 'a list -> 'a array
```
creates a new array from `l`. The length of the array is [`length`](list.md#SIG:LIST.length:VAL:SPEC)` ``l` and the `i`<sup>(th)</sup> element of the array is the `i`<sup>(th)</sup> element of the the list. If the length of the list is greater than [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised. For example:

```sml
Array.fromList [3, 5, 8]
(* array containing 3, 5, and 8 *)
```


```repl
Array.fromList [3, 5, 8];; (* array [3, 5, 8] *)
```
<span id="SIG:ARRAY.tabulate:VAL"></span>

### `tabulate`

```sml
val tabulate : int * (int -> 'a) -> 'a array
```
creates an array of `n` elements, where the elements are defined in order of increasing index by applying `f` to the element's index. This is equivalent to the expression:

```sml
Array.fromList (List.tabulate (n, f))
```

If `n` \< 0 or [`maxLen`](array.md#SIG:ARRAY.maxLen:VAL:SPEC) \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.


```repl
Array.tabulate (3, fn i => i * i);; (* array [0, 1, 4] *)
```
<span id="SIG:ARRAY.length:VAL"></span>

### `length`

```sml
val length : 'a array -> int
```
returns \|`arr`\|, the length of the array `arr`.


```repl
Array.length (Array.fromList []);; (* 0 *)
Array.length (Array.fromList [4, 5]);; (* 2 *)
```
<span id="SIG:ARRAY.sub:VAL"></span>

### `sub`

```sml
val sub : 'a array * int -> 'a
```
returns the `i`<sup>(th)</sup> element of the array `arr`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
Array.sub (Array.fromList [4, 5, 6], 1);; (* 5 *)
```
<span id="SIG:ARRAY.update:VAL"></span>

### `update`

```sml
val update : 'a array * int * 'a -> unit
```
sets the `i`<sup>(th)</sup> element of the array `arr` to `x`. If `i` \< 0 or \|`arr`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.


```repl
val a = Array.fromList [4, 5, 6];
Array.update (a, 1, 9);; (* () *)
Array.sub (a, 1);; (* 9 *)
```
<span id="SIG:ARRAY.vector:VAL"></span>

### `vector`

```sml
val vector : 'a array -> 'a vector
```
generates a vector from `arr`. Specifically, the result is equivalent to

          Vector.tabulate (length `arr`, fn i =\> sub (`arr`, i))



```repl
Array.vector (Array.fromList [4, 5, 6]);; (* vector [4, 5, 6] *)
```
<span id="SIG:ARRAY.copy:VAL"></span>

### `copy`

```sml
val copy : {src : 'a array, dst : 'a array, di : int} -> unit
```

### `copyVec`

```sml
val copyVec : {src : 'a vector, dst : 'a array, di : int} -> unit
```
These functions copy the entire array or vector `src` into the array `dst`, with the `i`<sup>(th)</sup> element in `src`, for 0 \<= `i` \< \|`src`\|, being copied to position `di` + `i` in the destination array. If `di` \< 0 or if \|`dst`\| \< `di`+\|`src`\|, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.

> **Implementation note:**
>
> In `copy`, if `dst` and `src` are equal, we must have `di`` = 0` to avoid an exception, and `copy` is then the identity.



```repl
val dst = Array.array (3, 0);
Array.copy {src = Array.fromList [4, 5], dst = dst, di = 1};; (* () *)
Array.vector dst;; (* vector [0, 4, 5] *)
Array.copyVec {src = Vector.fromList [7], dst = dst, di = 0};; (* () *)
Array.vector dst;; (* vector [7, 4, 5] *)
```
<span id="SIG:ARRAY.appi:VAL"></span>

### `appi`

```sml
val appi : (int * 'a -> unit) -> 'a array -> unit
```

### `app`

```sml
val app : ('a -> unit) -> 'a array -> unit
```
These apply the function `f` to the elements of the array `arr` in order of increasing indices. The more general form [`appi`](array.md#SIG:ARRAY.appi:VAL:SPEC) supplies `f` with the array index of the corresponding element.


```repl
Array.appi (fn (i, x) => print (Int.toString i ^ ":" ^ Int.toString x)) (Array.fromList [4, 5]);; (* () *)
Array.app (fn x => print (Int.toString x)) (Array.fromList [4, 5]);; (* () *)
```
<span id="SIG:ARRAY.modifyi:VAL"></span>

### `modifyi`

```sml
val modifyi : (int * 'a -> 'a) -> 'a array -> unit
```

### `modify`

```sml
val modify : ('a -> 'a) -> 'a array -> unit
```
These apply the function `f` to the elements of the array `arr` in order of increasing indices, and replace each element with the result. The more general [`modifyi`](array.md#SIG:ARRAY.modifyi:VAL:SPEC) supplies `f` with the array index of the corresponding element. The expression `modify ``f`` ``arr` is equivalent to `modifyi (``f`` o #2) ``arr`.


```repl
val a = Array.fromList [4, 5];
Array.modifyi (fn (i, x) => i + x) a;; (* () *)
Array.vector a;; (* vector [4, 6] *)
Array.modify (fn x => x * 2) a;; (* () *)
Array.vector a;; (* vector [8, 12] *)
```
<span id="SIG:ARRAY.foldli:VAL"></span>

### `foldli`

```sml
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
```

### `foldri`

```sml
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
```

### `foldl`

```sml
val foldl : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
```

### `foldr`

```sml
val foldr : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
```
These fold the function `f` over all the elements of the array `arr`, using the value `init` as the initial value. The functions [`foldli`](array.md#SIG:ARRAY.foldli:VAL:SPEC) and [`foldl`](array.md#SIG:ARRAY.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) and [`foldr`](array.md#SIG:ARRAY.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](array.md#SIG:ARRAY.foldli:VAL:SPEC) and [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) supply `f` with the array index of the corresponding element.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The expression `foldl ``f`` ``init`` ``arr` is equivalent to:

```sml
Array.foldli (fn (_, a, x) => f (a, x)) init arr
```

The analogous equivalences hold for [`foldri`](array.md#SIG:ARRAY.foldri:VAL:SPEC) and [`foldr`](array.md#SIG:ARRAY.foldr:VAL:SPEC).


```repl
val a = Array.fromList [2, 6];
Array.foldli (fn (i, x, acc) => acc + i + x) 0 a;; (* 9 *)
Array.foldri (fn (i, x, acc) => acc + i + x) 0 a;; (* 9 *)
Array.foldl (op +) 0 a;; (* 8 *)
Array.foldr (op +) 0 a;; (* 8 *)
```
<span id="SIG:ARRAY.findi:VAL"></span>

### `findi`

```sml
val findi : (int * 'a -> bool) -> 'a array -> (int * 'a) option
```

### `find`

```sml
val find : ('a -> bool) -> 'a array -> 'a option
```
These functions apply `f` to each element of the array `arr`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](array.md#SIG:ARRAY.findi:VAL:SPEC) also supplies `f` with the array index of the element and, upon finding an entry satisfying the predicate, returns that index with the element.


```repl
val a = Array.fromList [2, 6];
Array.findi (fn (_, x) => x > 3) a;; (* SOME (1, 6) *)
Array.find (fn x => x > 3) a;; (* SOME 6 *)
```
<span id="SIG:ARRAY.exists:VAL"></span>

### `exists`

```sml
val exists : ('a -> bool) -> 'a array -> bool
```
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.


```repl
val a = Array.fromList [2, 6];
Array.exists (fn x => x = 6) a;; (* true *)
```
<span id="SIG:ARRAY.all:VAL"></span>

### `all`

```sml
val all : ('a -> bool) -> 'a array -> bool
```
applies `f` to each element `x` of the array `arr`, from left to right (_i.e._, increasing indices), until `f`` ``x` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](array.md#SIG:ARRAY.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f``) ``arr``))`.


```repl
Array.all (fn x => x > 0) (Array.fromList []);; (* true *)
Array.all (fn x => x > 0) (Array.fromList [2, 6]);; (* true *)
```
<span id="SIG:ARRAY.collate:VAL"></span>

### `collate`

```sml
val collate : ('a * 'a -> order) -> 'a array * 'a array -> order
```
performs lexicographic comparison of the two arrays using the given ordering `f` on elements.

```repl
Array.collate Int.compare (Array.fromList [1, 2], Array.fromList [1, 3]);; (* LESS *)
```

#### See Also

> [`ArraySlice`](array-slice.md#ArraySlice:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`Vector`](vector.md#Vector:STR:SPEC)
