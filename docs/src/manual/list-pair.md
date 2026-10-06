# <span id="section:0"></span>The `ListPair` structure

---

#### Synopsis

<span id="LIST_PAIR:SIG:SPEC"></span>
<span id="ListPair:STR:SPEC"></span>

```sml
signature LIST_PAIR
structure ListPair :> LIST_PAIR
```

The `ListPair` structure provides operations on pairs of lists. The operations fall into two categories. Those in the first category, whose names do not end in `"Eq"`, do not require that the lists have the same length. When the lists are of uneven lengths, the excess elements from the tail of the longer list are ignored. The operations in the second category, whose names have the suffix `"Eq"`, differ from their similarly named operations in the first category only when the list arguments have unequal lengths, in which case they typically raise the [`UnequalLengths`](list-pair.md#SIG:LIST_PAIR.UnequalLengths:EXN:SPEC) exception.

---

#### Interface

<span id="SIG:LIST_PAIR.UnequalLengths:EXN:SPEC"></span>
<span id="SIG:LIST_PAIR.zip:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.zipEq:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.unzip:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.app:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.appEq:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.map:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.mapEq:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.foldl:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.foldr:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.foldlEq:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.foldrEq:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.all:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.exists:VAL:SPEC"></span>
<span id="SIG:LIST_PAIR.allEq:VAL:SPEC"></span>

```sml
exception UnequalLengths
val zip : 'a list * 'b list -> ('a * 'b) list
val zipEq : 'a list * 'b list -> ('a * 'b) list
val unzip : ('a * 'b) list -> 'a list * 'b list
val app : ('a * 'b -> unit) -> 'a list * 'b list -> unit
val appEq : ('a * 'b -> unit) -> 'a list * 'b list -> unit
val map : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
val mapEq : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
val foldl : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
val foldr : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
val foldlEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
val foldrEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
val all : ('a * 'b -> bool) -> 'a list * 'b list -> bool
val exists : ('a * 'b -> bool) -> 'a list * 'b list -> bool
val allEq : ('a * 'b -> bool) -> 'a list * 'b list -> bool
```

#### Description

<span id="SIG:LIST_PAIR.UnequalLengths:EXN"></span>

### `UnequalLengths`

```sml
exception UnequalLengths
```
This exception is raised by those functions that require arguments of identical length.


```repl
((ListPair.zipEq ([1], []); false) handle ListPair.UnequalLengths => true);; (* true *)
```

<span id="SIG:LIST_PAIR.zip:VAL"></span>

### `zip`

```sml
val zip : 'a list * 'b list -> ('a * 'b) list
```

### `zipEq`

```sml
val zipEq : 'a list * 'b list -> ('a * 'b) list
```
These functions combine the two lists `l1` and `l2` into a list of pairs, with the first element of each list comprising the first element of the result, the second elements comprising the second element of the result, and so on. If the lists are of unequal lengths, [`zip`](list-pair.md#SIG:LIST_PAIR.zip:VAL:SPEC) ignores the excess elements from the tail of the longer one, while [`zipEq`](list-pair.md#SIG:LIST_PAIR.zipEq:VAL:SPEC) raises the exception [`UnequalLengths`](list-pair.md#SIG:LIST_PAIR.UnequalLengths:EXN:SPEC).


```repl
ListPair.zip ([1, 2], ["a"]);; (* [(1, "a")] *)
ListPair.zipEq ([1, 2], ["a", "b"]);; (* [(1, "a"), (2, "b")] *)
((ListPair.zipEq ([1], []); false) handle ListPair.UnequalLengths => true);; (* true *)
```

<span id="SIG:LIST_PAIR.unzip:VAL"></span>

### `unzip`

```sml
val unzip : ('a * 'b) list -> 'a list * 'b list
```
returns a pair of lists formed by splitting the elements of `l`. This is the inverse of `zip` for equal length lists.


```repl
ListPair.unzip [(1, "a"), (2, "b")];; (* ([1, 2], ["a", "b"]) *)
```

<span id="SIG:LIST_PAIR.app:VAL"></span>

### `app`

```sml
val app : ('a * 'b -> unit) -> 'a list * 'b list -> unit
```

### `appEq`

```sml
val appEq : ('a * 'b -> unit) -> 'a list * 'b list -> unit
```
These apply the function `f` to the list of pairs of elements generated from left to right from the lists `l1` and `l2`. If the lists are of unequal lengths, the former ignores the excess elements from the tail of the longer one, and the latter raises [`UnequalLengths`](list-pair.md#SIG:LIST_PAIR.UnequalLengths:EXN:SPEC). The above expressions are respectively equivalent to:

      [List.app](list.md#SIG:LIST.app:VAL:SPEC) `f` (zip (`l1`, `l2`))
      [List.app](list.md#SIG:LIST.app:VAL:SPEC) `f` (zipEq (`l1`, `l2`))

ignoring possible side-effects of the function `f`.


```repl
ListPair.app (fn (n, s) => print (Int.toString n ^ s)) ([1, 2], ["a", "b"]);; (* () *)
ListPair.appEq (fn (n, s) => print (Int.toString n ^ s)) ([1], ["a"]);; (* () *)
```

<span id="SIG:LIST_PAIR.map:VAL"></span>

### `map`

```sml
val map : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
```

### `mapEq`

```sml
val mapEq : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
```
These map the function `f` over the list of pairs of elements generated from left to right from the lists `l1` and `l2`, returning the list of results. If the lists are of unequal lengths, the former ignores the excess elements from the tail of the longer one, and the latter raises [`UnequalLengths`](list-pair.md#SIG:LIST_PAIR.UnequalLengths:EXN:SPEC). The above expressions are respectively equivalent to:

      [List.map](list.md#SIG:LIST.map:VAL:SPEC) `f` (zip (`l1`, `l2`))
      [List.map](list.md#SIG:LIST.map:VAL:SPEC) `f` (zipEq (`l1`, `l2`))

ignoring possible side-effects of the function `f`.


```repl
ListPair.map (op +) ([1, 2], [10]);; (* [11] *)
ListPair.mapEq (op +) ([1, 2], [10, 20]);; (* [11, 22] *)
```

<span id="SIG:LIST_PAIR.foldl:VAL"></span>

### `foldl`

```sml
val foldl : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
```

### `foldr`

```sml
val foldr : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
```

### `foldlEq`

```sml
val foldlEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
```

### `foldrEq`

```sml
val foldrEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
```
These return the result of folding the function `f` in the specified direction over the pair of lists `l1` and `l2` starting with the value `init`. They are respectively equivalent to:

      [List.foldl](list.md#SIG:LIST.foldl:VAL:SPEC) `f'` `init` (zip (`l1`, `l2`))
      [List.foldr](list.md#SIG:LIST.foldr:VAL:SPEC) `f'` `init` (zip (`l1`, `l2`))
      [List.foldl](list.md#SIG:LIST.foldl:VAL:SPEC) `f'` `init` (zipEq (`l1`, `l2`))
      [List.foldr](list.md#SIG:LIST.foldr:VAL:SPEC) `f'` `init` (zipEq (`l1`, `l2`))

where `f'` is `fn ((a,b),c) => f(a,b,c)` and ignoring possible side-effects of the function `f`.


```repl
ListPair.foldl (fn (x, y, a) => x + y + a) 0 ([1, 2], [10, 20]);; (* 33 *)
ListPair.foldr (fn (x, y, a) => x + y + a) 0 ([1, 2], [10, 20]);; (* 33 *)
ListPair.foldlEq (fn (x, y, a) => x + y + a) 0 ([1, 2], [10, 20]);; (* 33 *)
ListPair.foldrEq (fn (x, y, a) => x + y + a) 0 ([1, 2], [10, 20]);; (* 33 *)
```

<span id="SIG:LIST_PAIR.all:VAL"></span>

### `all`

```sml
val all : ('a * 'b -> bool) -> 'a list * 'b list -> bool
```

### `exists`

```sml
val exists : ('a * 'b -> bool) -> 'a list * 'b list -> bool
```
These functions provide short-circuit testing of a predicate over a pair of lists. They are respectively equivalent to:

      [List.all](list.md#SIG:LIST.all:VAL:SPEC) `f` (zip (`l1`, `l2`))
      [List.exists](list.md#SIG:LIST.exists:VAL:SPEC) `f` (zip (`l1`, `l2`))



```repl
ListPair.all (op =) ([1, 2], [1, 2]);; (* true *)
ListPair.exists (op =) ([1, 2], [3, 2]);; (* true *)
```

<span id="SIG:LIST_PAIR.allEq:VAL"></span>

### `allEq`

```sml
val allEq : ('a * 'b -> bool) -> 'a list * 'b list -> bool
```
returns `true` if `l1` and `l2` have equal length and all pairs of elements satisfy the predicate `f`. That is, the expression is equivalent to:

        ([List.length](list.md#SIG:LIST.length:VAL:SPEC) `l1` = [List.length](list.md#SIG:LIST.length:VAL:SPEC) `l2`) andalso
      ([List.all](list.md#SIG:LIST.all:VAL:SPEC) `f` (zip (`l1`, `l2`)))

This function does not appear to have any nice algebraic relation with the other functions, but it is included as providing a useful notion of equality, analogous to the notion of equality of lists over equality types.

> **Implementation note:**
>
> The implementation is simple:
>
>         fun allEq p (\[\], \[\]) = true
>           \| allEq p (x::xs, y::ys) = p(x,y) andalso allEq p (xs,ys)
>           \| allEq _ _ = false
>       


```repl
ListPair.allEq (op =) ([1, 2], [1, 2]);; (* true *)
ListPair.allEq (op =) ([1], [1, 2]);; (* false *)
```


#### See Also

> [`List`](list.md#List:STR:SPEC)

#### Discussion

Note that a function requiring equal length arguments should determine this lazily, _i.e._, it should act as though the lists have equal length and invoke the user-supplied function argument, but raise the exception if it arrives at the end of one list before the end of the other.
