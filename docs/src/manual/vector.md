# <span id="section:0"></span>The `Vector` structure

---

#### Synopsis

<span id="VECTOR:SIG:SPEC"></span>
<span id="Vector:STR:SPEC"></span>

```sml
signature VECTOR
structure Vector :> VECTOR
```

The `Vector` structure defines polymorphic vectors, immutable sequences with constant-time access.

---

#### Interface

<span id="SIG:VECTOR.vector:TY:SPEC"></span>
<span id="SIG:VECTOR.maxLen:VAL:SPEC"></span>
<span id="SIG:VECTOR.fromList:VAL:SPEC"></span>
<span id="SIG:VECTOR.tabulate:VAL:SPEC"></span>
<span id="SIG:VECTOR.length:VAL:SPEC"></span>
<span id="SIG:VECTOR.sub:VAL:SPEC"></span>
<span id="SIG:VECTOR.update:VAL:SPEC"></span>
<span id="SIG:VECTOR.concat:VAL:SPEC"></span>
<span id="SIG:VECTOR.appi:VAL:SPEC"></span>
<span id="SIG:VECTOR.app:VAL:SPEC"></span>
<span id="SIG:VECTOR.mapi:VAL:SPEC"></span>
<span id="SIG:VECTOR.map:VAL:SPEC"></span>
<span id="SIG:VECTOR.foldli:VAL:SPEC"></span>
<span id="SIG:VECTOR.foldri:VAL:SPEC"></span>
<span id="SIG:VECTOR.foldl:VAL:SPEC"></span>
<span id="SIG:VECTOR.foldr:VAL:SPEC"></span>
<span id="SIG:VECTOR.findi:VAL:SPEC"></span>
<span id="SIG:VECTOR.find:VAL:SPEC"></span>
<span id="SIG:VECTOR.exists:VAL:SPEC"></span>
<span id="SIG:VECTOR.all:VAL:SPEC"></span>
<span id="SIG:VECTOR.collate:VAL:SPEC"></span>

```sml
eqtype 'a vector = 'a vector
val maxLen : int
val fromList : 'a list -> 'a vector
val tabulate : int * (int -> 'a) -> 'a vector
val length : 'a vector -> int
val sub : 'a vector * int -> 'a
val update : 'a vector * int * 'a -> 'a vector
val concat : 'a vector list -> 'a vector
val appi : (int * 'a -> unit) -> 'a vector -> unit
val app : ('a -> unit) -> 'a vector -> unit
val mapi : (int * 'a -> 'b) -> 'a vector -> 'b vector
val map : ('a -> 'b) -> 'a vector -> 'b vector
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
val foldl : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
val foldr : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
val findi : (int * 'a -> bool) -> 'a vector -> (int * 'a) option
val find : ('a -> bool) -> 'a vector -> 'a option
val exists : ('a -> bool) -> 'a vector -> bool
val all : ('a -> bool) -> 'a vector -> bool
val collate : ('a * 'a -> order) -> 'a vector * 'a vector -> order
```

#### Description

<span id="SIG:VECTOR.maxLen:VAL"></span>

### `maxLen`

```sml
val maxLen : int
```
**`val`**` maxLen `**`:`**` int`  
The maximum length of vectors supported by this implementation. Attempts to create larger vectors will result in the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception being raised.



```repl
Vector.maxLen;; (* maximum supported length *)
```

<span id="SIG:VECTOR.fromList:VAL"></span>

### `fromList`

```sml
val fromList : 'a list -> 'a vector
```

`fromList ``l`` `  
creates a new vector from `l`, whose length is `length ``l` and with the `i`<sup>(th)</sup> element of `l` used as the `i`<sup>(th)</sup> element of the vector. If the length of the list is greater than [`maxLen`](vector.md#SIG:VECTOR.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.



```repl
Vector.fromList [1, 2, 3];; (* #[1, 2, 3] *)
```

<span id="SIG:VECTOR.tabulate:VAL"></span>

### `tabulate`

```sml
val tabulate : int * (int -> 'a) -> 'a vector
```

`tabulate (``n``, ``f``) `  
creates a vector of `n` elements, where the elements are defined in order of increasing index by applying `f` to the element's index. This is equivalent to the expression:

fromList (List.tabulate (`n`, `f`))

If `n` \< 0 or `maxLen` \< `n`, then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.



```repl
Vector.tabulate (3, fn i => i * i);; (* #[0, 1, 4] *)
```

<span id="SIG:VECTOR.length:VAL"></span>

### `length`

```sml
val length : 'a vector -> int
```

`length ``vec`` `  
returns \|`vec`\|, the length of the vector `vec`.



```repl
Vector.length (Vector.fromList [1, 2, 3]);; (* 3 *)
```

<span id="SIG:VECTOR.sub:VAL"></span>

### `sub`

```sml
val sub : 'a vector * int -> 'a
```

`sub (``vec``, ``i``) `  
returns the `i`<sup>(th)</sup> element of the vector `vec`. If `i` \< 0 or \|`vec`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.



```repl
Vector.sub (Vector.fromList [1, 2, 3], 1);; (* 2 *)
```

<span id="SIG:VECTOR.update:VAL"></span>

### `update`

```sml
val update : 'a vector * int * 'a -> 'a vector
```

`update (``vec``, ``i``, ``x``) `  
returns a new vector, identical to `vec`, except the `i`<sup>(th)</sup> element of `vec` is set to `x`. If `i` \< 0 or \|`vec`\| \<= `i`, then the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception is raised.



```repl
Vector.update (Vector.fromList [1, 2, 3], 1, 9);; (* #[1, 9, 3] *)
```

<span id="SIG:VECTOR.concat:VAL"></span>

### `concat`

```sml
val concat : 'a vector list -> 'a vector
```

`concat ``l`` `  
returns the vector that is the concatenation of the vectors in the list `l`. If the total length of these vectors exceeds [`maxLen`](vector.md#SIG:VECTOR.maxLen:VAL:SPEC), then the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception is raised.



```repl
Vector.concat [Vector.fromList [1, 2], Vector.fromList [3]];; (* #[1, 2, 3] *)
```

<span id="SIG:VECTOR.appi:VAL"></span>

### `appi`

```sml
val appi : (int * 'a -> unit) -> 'a vector -> unit
```

### `app`

```sml
val app : ('a -> unit) -> 'a vector -> unit
```

`appi ``f`` ``vec`` `
` app ``f`` ``vec`` `  
These apply the function `f` to the elements of a vector in left to right order (_i.e._, in order of increasing indices). The more general [`appi`](vector.md#SIG:VECTOR.appi:VAL:SPEC) function supplies both the element and the element's index to the function `f`. These are respectively equivalent to:

      List.app f (foldri (fn (i,a,l) =\> (i,a)::l) \[\] vec)
      List.app f (foldr (fn (a,l) =\> a::l) \[\] vec)




```repl
Vector.foldli (fn (i, x, acc) => (i, x) :: acc) [] (Vector.fromList [1, 2, 3]);; (* [(2, 3), (1, 2), (0, 1)] *)
```

```repl
Vector.foldr op:: [] (Vector.fromList [1, 2, 3]);; (* [1, 2, 3] *)
```

<span id="SIG:VECTOR.mapi:VAL"></span>

### `mapi`

```sml
val mapi : (int * 'a -> 'b) -> 'a vector -> 'b vector
```

### `map`

```sml
val map : ('a -> 'b) -> 'a vector -> 'b vector
```

`mapi ``f`` ``vec`` `
` map ``f`` ``vec`` `  
These functions produce new vectors by mapping the function `f` from left to right over the argument vector. The more general form [`mapi`](vector.md#SIG:VECTOR.mapi:VAL:SPEC) supplies `f` with the vector index of an element along with the element. These are respectively equivalent to:

      fromList (List.map f (foldri (fn (i,a,l) =\> (i,a)::l) \[\] vec))
      fromList (List.map f (foldr (fn (a,l) =\> a::l) \[\] vec))




```repl
Vector.mapi (fn (i, x) => i + x) (Vector.fromList [1, 2, 3]);; (* #[1, 3, 5] *)
```

```repl
Vector.map (fn x => x * 2) (Vector.fromList [1, 2, 3]);; (* #[2, 4, 6] *)
```

<span id="SIG:VECTOR.foldli:VAL"></span>

### `foldli`

```sml
val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
```

### `foldri`

```sml
val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
```

### `foldl`

```sml
val foldl : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
```

### `foldr`

```sml
val foldr : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
```

`foldli ``f`` ``init`` ``vec`` `
` foldri ``f`` ``init`` ``vec`` `
` foldl ``f`` ``init`` ``vec`` `
` foldr ``f`` ``init`` ``vec`` `  
These fold the function `f` over all the elements of a vector, using the value `init` as the initial value. The functions [`foldli`](vector.md#SIG:VECTOR.foldli:VAL:SPEC) and [`foldl`](vector.md#SIG:VECTOR.foldl:VAL:SPEC) apply the function `f` from left to right (increasing indices), while the functions [`foldri`](vector.md#SIG:VECTOR.foldri:VAL:SPEC) and [`foldr`](vector.md#SIG:VECTOR.foldr:VAL:SPEC) work from right to left (decreasing indices). The more general functions [`foldli`](vector.md#SIG:VECTOR.foldli:VAL:SPEC) and [`foldri`](vector.md#SIG:VECTOR.foldri:VAL:SPEC) supply both the element and the element's index to the function `f`.

Refer to the [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC) manual pages for reference implementations of the indexed versions.

The last two expressions are respectively equivalent to:

      foldli (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `vec`
      foldri (fn (\_, `a`, `x`) =\> `f`(`a`, `x`)) `init` `vec`




```repl
Vector.foldli (fn (i, x, n) => n + i + x) 0 (Vector.fromList [1, 2, 3]);; (* 9 *)
```

```repl
Vector.foldri (fn (i, x, n) => n + i + x) 0 (Vector.fromList [1, 2, 3]);; (* 9 *)
```

```repl
Vector.foldl (op +) 0 (Vector.fromList [1, 2, 3]);; (* 6 *)
```

```repl
Vector.foldr (op +) 0 (Vector.fromList [1, 2, 3]);; (* 6 *)
```

<span id="SIG:VECTOR.findi:VAL"></span>

### `findi`

```sml
val findi : (int * 'a -> bool) -> 'a vector -> (int * 'a) option
```

### `find`

```sml
val find : ('a -> bool) -> 'a vector -> 'a option
```

`findi ``f`` ``vec`` `
` find ``f`` ``vec`` `  
These apply `f` to each element of the vector `vec`, from left to right (_i.e._, increasing indices), until a `true` value is returned. If this occurs, the functions return the element; otherwise, they return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The more general version [`findi`](vector.md#SIG:VECTOR.findi:VAL:SPEC) also supplies `f` with the vector index of the element and, upon finding an entry satisfying the predicate, returns that index with the element.



```repl
Vector.findi (fn (_, x) => x > 1) (Vector.fromList [1, 2, 3]);; (* SOME (1, 2) *)
```

```repl
Vector.find (fn x => x > 2) (Vector.fromList [1, 2, 3]);; (* SOME 3 *)
```

<span id="SIG:VECTOR.exists:VAL"></span>

### `exists`

```sml
val exists : ('a -> bool) -> 'a vector -> bool
```

`exists ``f`` ``vec`` `  
applies `f` to each element `x` of the vector `vec`, from left to right (_i.e._, increasing indices), until `f``(``x``)` evaluates to `true`; it returns `true` if such an `x` exists and `false` otherwise.



```repl
Vector.exists (fn x => x = 2) (Vector.fromList [1, 2, 3]);; (* true *)
```

<span id="SIG:VECTOR.all:VAL"></span>

### `all`

```sml
val all : ('a -> bool) -> 'a vector -> bool
```

`all ``f`` ``vec`` `  
applies `f` to each element `x` of the vector `vec`, from left to right (_i.e._, increasing indices), until `f``(``x``)` evaluates to `false`; it returns `false` if such an `x` exists and `true` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`exists`](vector.md#SIG:VECTOR.exists:VAL:SPEC)` (`[`not`](bool.md#SIG:BOOL.not:VAL:SPEC)` o ``f`` ) ``vec``))`.



```repl
Vector.all (fn x => x > 0) (Vector.fromList [1, 2, 3]);; (* true *)
```

<span id="SIG:VECTOR.collate:VAL"></span>

### `collate`

```sml
val collate : ('a * 'a -> order) -> 'a vector * 'a vector -> order
```

`collate ``f`` (``v1``, ``v2``) `  
performs lexicographic comparison of the two vectors using the given ordering `f` on elements.



```repl
Vector.collate Int.compare (Vector.fromList [1, 2], Vector.fromList [1, 3]);; (* LESS *)
```

#### Examples

```repl
Vector.fromList [];; (* #[] *)
Vector.fromList [1, 2, 3];; (* #[1, 2, 3] *)
```

#### See Also

> [`Array`](array.md#Array:STR:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`VectorSlice`](vector-slice.md#VectorSlice:STR:SPEC)
