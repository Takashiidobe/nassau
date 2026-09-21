# The `List` structure

## Synopsis

```sml
signature LIST
structure List :> LIST
```

The `List` structure provides a collection of utility functions for manipulating polymorphic lists, traditionally an important datatype in functional programming. Following the concrete syntax provided by the list `::` operator, the head of a list appears leftmost. Thus, a traversal of a list from left to right starts with the head, then recurses on the tail. In addition, as a sequence type, a list has an indexing of its elements, with the head having index 0, the second element having index 1, etc.

## Interface

```sml
datatype 'a list = nil | :: of 'a * 'a list
exception Empty
val null : 'a list -> bool
val length : 'a list -> int
val @ : 'a list * 'a list -> 'a list
val hd : 'a list -> 'a
val tl : 'a list -> 'a list
val last : 'a list -> 'a
val getItem : 'a list -> ('a * 'a list) option
val nth : 'a list * int -> 'a
val take : 'a list * int -> 'a list
val drop : 'a list * int -> 'a list
val rev : 'a list -> 'a list
val concat : 'a list list -> 'a list
val revAppend : 'a list * 'a list -> 'a list
val app : ('a -> unit) -> 'a list -> unit
val map : ('a -> 'b) -> 'a list -> 'b list
val mapPartial : ('a -> 'b option) -> 'a list -> 'b list
val find : ('a -> bool) -> 'a list -> 'a option
val filter : ('a -> bool) -> 'a list -> 'a list
val partition : ('a -> bool) -> 'a list -> 'a list * 'a list
val foldl : ('a * 'b -> 'b) -> 'b -> 'a list -> 'b
val foldr : ('a * 'b -> 'b) -> 'b -> 'a list -> 'b
val exists : ('a -> bool) -> 'a list -> bool
val all : ('a -> bool) -> 'a list -> bool
val tabulate : int * (int -> 'a) -> 'a list
val collate : ('a * 'a -> order) -> 'a list * 'a list -> order
```

## Description

### `Empty`

This exception indicates that an empty list was given to a function requiring a non-empty list.

```repl
List.hd [];;
```

### `null`

`null l` returns `true` if `l` is empty.

```repl
List.null [];;
```

### `length`

`length l` returns the number of elements in `l`.

```repl
List.length [10, 20, 30];;
```

### `@`

`l1 @ l2` returns the concatenation of `l1` and `l2`.

```repl
[1, 2] @ [3, 4];;
```

### `hd`

`hd l` returns the first element of `l`. It raises `Empty` if `l` is `nil`.

```repl
List.hd [10, 20, 30];;
```

### `tl`

`tl l` returns all but the first element of `l`. It raises `Empty` if `l` is `nil`.

```repl
List.tl [10, 20, 30];;
```

### `last`

`last l` returns the last element of `l`. It raises `Empty` if `l` is `nil`.

```repl
List.last [10, 20, 30];;
```

### `getItem`

`getItem l` returns `NONE` if `l` is empty, and `SOME (hd l, tl l)` otherwise. This function is particularly useful for creating value readers from lists of characters. For example, `Int.scan StringCvt.DEC getItem` has type `(int, char list) StringCvt.reader` and can be used to scan decimal integers from lists of characters.

```repl
List.getItem [10, 20, 30];;
```

### `nth`

`nth (l, i)` returns the `i`th element of `l`, counting from 0. It raises `Subscript` if `i < 0` or `i >= length l`. We have `nth (l, 0) = hd l`, ignoring exceptions.

```repl
List.nth ([10, 20, 30], 1);;
```

### `take`

`take (l, i)` returns the first `i` elements of `l`. It raises `Subscript` if `i < 0` or `i > length l`. We have `take (l, length l) = l`.

```repl
List.take ([10, 20, 30, 40], 2);;
```

### `drop`

`drop (l, i)` returns what is left after dropping the first `i` elements of `l`. It raises `Subscript` if `i < 0` or `i > length l`. For `0 <= i <= length l`, `take (l, i) @ drop (l, i) = l`. We also have `drop (l, length l) = []`.

```repl
List.drop ([10, 20, 30, 40], 2);;
```

### `rev`

`rev l` returns a list consisting of the elements of `l` in reverse order.

```repl
List.rev [1, 2, 3];;
```

### `concat`

`concat l` returns the concatenation of all the lists in `l`, in order. In other words, `concat [l1, l2, ..., ln] = l1 @ l2 @ ... @ ln`.

```repl
List.concat [[1, 2], [], [3, 4]];;
```

### `revAppend`

`revAppend (l1, l2)` returns `(rev l1) @ l2`.

```repl
List.revAppend ([1, 2, 3], [4, 5]);;
```

### `app`

`app f l` applies `f` to the elements of `l`, from left to right.

```repl
List.app (fn n => print (Int.toString n ^ " ")) [1, 2, 3];;
```

### `map`

`map f l` applies `f` to each element of `l` from left to right, returning the list of results.

```repl
List.map (fn n => n * n) [1, 2, 3];;
```

### `mapPartial`

`mapPartial f l` applies `f` to each element of `l` from left to right, returning a list of results with `SOME` stripped wherever `f` is defined. `f` is not defined for an element of `l` if applying `f` to that element returns `NONE`. This is equivalent to `((map valOf) o (filter isSome) o (map f)) l`.

```repl
List.mapPartial (fn n => if n mod 2 = 0 then SOME (n * n) else NONE) [1, 2, 3, 4];;
```

### `find`

`find f l` applies `f` to each element `x` of `l`, from left to right, until `f x` evaluates to `true`. It returns `SOME x` if such an `x` exists; otherwise it returns `NONE`.

```repl
List.find (fn n => n > 3) [1, 2, 3, 4, 5];;
```

### `filter`

`filter f l` applies `f` to each element `x` of `l`, from left to right, and returns the elements for which `f x` evaluates to `true`, in their original order.

```repl
List.filter (fn n => n mod 2 = 0) [1, 2, 3, 4];;
```

### `partition`

`partition f l` applies `f` to each element `x` of `l`, from left to right, and returns a pair `(pos, neg)`. `pos` contains the elements for which `f x` evaluates to `true`; `neg` contains those for which it evaluates to `false`. Both lists preserve the relative order of their elements in `l`.

```repl
List.partition (fn n => n mod 2 = 0) [1, 2, 3, 4];;
```

### `foldl`

`foldl f init [x1, x2, ..., xn]` returns `f (xn, ..., f (x2, f (x1, init))...)`, or `init` if the list is empty.

```repl
List.foldl (fn (n, total) => n - total) 0 [1, 2, 3];;
```

### `foldr`

`foldr f init [x1, x2, ..., xn]` returns `f (x1, f (x2, ..., f (xn, init)...))`, or `init` if the list is empty.

```repl
List.foldr (fn (n, total) => n - total) 0 [1, 2, 3];;
```

### `exists`

`exists f l` applies `f` to each element `x` of `l`, from left to right, until `f x` evaluates to `true`. It returns `true` if such an `x` exists and `false` otherwise.

```repl
List.exists (fn n => n > 3) [1, 2, 3, 4];;
```

### `all`

`all f l` applies `f` to each element `x` of `l`, from left to right, until `f x` evaluates to `false`. It returns `false` if such an `x` exists and `true` otherwise. It is equivalent to `not (exists (not o f) l)`.

```repl
List.all (fn n => n > 0) [1, 2, 3];;
```

### `tabulate`

`tabulate (n, f)` returns a list of length `n` equal to `[f(0), f(1), ..., f(n-1)]`, created from left to right. It raises `Size` if `n < 0`.

```repl
List.tabulate (5, fn i => i * i);;
```

### `collate`

`collate f (l1, l2)` performs lexicographic comparison of the two lists using the given ordering `f` on list elements.

```repl
List.collate Int.compare ([1, 2], [1, 3]);;
```

## See also

[`General`](https://smlfamily.github.io/Basis/general.html), [`ListPair`](https://smlfamily.github.io/Basis/list-pair.html)

## Discussion

The `list` type is considered primitive and is defined in the top-level environment. It is rebound here for consistency.

> **Rationale:** Lists are usually supported with a large collection of library functions. Here, we provide a somewhat smaller collection of operations that reflect common usage. We feel the collection is moderately complete, in that most programs will not need to define additional list operations. We have tried to adopt names that reflect a consensus from various existing libraries and texts. We have avoided functions relying on equality types.
>
> Different SML implementations may still desire to provide list utility library modules, though if the design of `List` is right, they should be small.

Source: [The Standard ML Basis Library, `List` structure](https://smlfamily.github.io/Basis/list.html#section:0). Generated April 12, 2004; last modified May 24, 2000.
