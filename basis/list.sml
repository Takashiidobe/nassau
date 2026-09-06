fun op @ ([], ys) = ys
  | op @ (x :: xs, ys) = x :: (xs @ ys)

structure List =
struct
  exception Empty = Empty

  fun null [] = true
    | null _ = false

  val length = length

  val op @ = op @

  fun hd [] = raise Empty
    | hd (x :: _) = x

  fun tl [] = raise Empty
    | tl (_ :: xs) = xs

  fun last [] = raise Empty
    | last [x] = x
    | last (_ :: xs) = last xs

  fun getItem [] = NONE
    | getItem (x :: xs) = SOME (x, xs)

  fun nth (l, i) =
    let
      fun go ([], _) = raise Subscript
        | go (x :: xs, n) = if n = 0 then x else go (xs, n - 1)
    in
      if i < 0 then raise Subscript else go (l, i)
    end

  fun take (l, i) =
    let
      fun go (_, 0, acc) = rev' (acc, [])
        | go ([], _, _) = raise Subscript
        | go (x :: xs, n, acc) = go (xs, n - 1, x :: acc)
      and rev' ([], acc) = acc
        | rev' (x :: xs, acc) = rev' (xs, x :: acc)
    in
      if i < 0 then raise Subscript else go (l, i, [])
    end

  fun drop (l, i) =
    let
      fun go (xs, 0) = xs
        | go ([], _) = raise Subscript
        | go (_ :: xs, n) = go (xs, n - 1)
    in
      if i < 0 then raise Subscript else go (l, i)
    end

  fun revAppend ([], ys) = ys
    | revAppend (x :: xs, ys) = revAppend (xs, x :: ys)

  fun rev xs = revAppend (xs, [])

  fun concat [] = []
    | concat (l :: ls) = l @ concat ls

  fun app f [] = ()
    | app f (x :: xs) = (f x; app f xs)

  val map = map

  fun mapPartial f [] = []
    | mapPartial f (x :: xs) =
        (case f x of
           NONE => mapPartial f xs
         | SOME y => y :: mapPartial f xs)

  fun find p [] = NONE
    | find p (x :: xs) = if p x then SOME x else find p xs

  fun filter p [] = []
    | filter p (x :: xs) = if p x then x :: filter p xs else filter p xs

  fun partition p l =
    let
      fun go ([], yes, no) = (rev yes, rev no)
        | go (x :: xs, yes, no) =
            if p x then go (xs, x :: yes, no) else go (xs, yes, x :: no)
    in
      go (l, [], [])
    end

  val foldl = foldl

  fun foldr f init [] = init
    | foldr f init (x :: xs) = f (x, foldr f init xs)

  fun exists p [] = false
    | exists p (x :: xs) = p x orelse exists p xs

  fun all p [] = true
    | all p (x :: xs) = p x andalso all p xs

  fun tabulate (n, f) =
    let
      fun go i = if i >= n then [] else let val x = f i in x :: go (i + 1) end
    in
      if n < 0 then raise Size else go 0
    end

  fun collate cmp ([], []) = EQUAL
    | collate cmp ([], _) = LESS
    | collate cmp (_, []) = GREATER
    | collate cmp (x :: xs, y :: ys) =
        (case cmp (x, y) of
           EQUAL => collate cmp (xs, ys)
         | order => order)
end

val app = List.app
val hd = List.hd
val tl = List.tl
val null = List.null
val rev = List.rev
val foldr = List.foldr
