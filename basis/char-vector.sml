structure CharVector :> MONO_VECTOR
where type vector = String.string
where type elem = char =
struct
  type vector = String.string
  type elem = char

  val maxLen = String.maxSize
  val fromList = String.implode
  val length = String.size
  val sub = String.sub
  val concat = String.concat

  fun tabulate (n, f) =
    if n < 0 orelse n > maxLen then raise Size
    else String.implode (List.tabulate (n, f))

  fun update (vector, i, elem) =
    if i < 0 orelse i >= length vector then raise Subscript
    else
      String.concat
        [String.substring (vector, 0, i), String.str elem,
         String.extract (vector, i + 1, NONE)]

  fun appi f vector =
    let
      fun loop i =
        if i = length vector then ()
        else (f (i, sub (vector, i)); loop (i + 1))
    in loop 0 end

  fun app f vector = appi (fn (_, elem) => f elem) vector

  fun mapi f vector =
    tabulate (length vector, fn i => f (i, sub (vector, i)))

  fun map f vector = mapi (fn (_, elem) => f elem) vector

  fun foldli f init vector =
    let
      fun loop (i, acc) =
        if i = length vector then acc
        else loop (i + 1, f (i, sub (vector, i), acc))
    in loop (0, init) end

  fun foldri f init vector =
    let
      fun loop (i, acc) =
        if i < 0 then acc
        else loop (i - 1, f (i, sub (vector, i), acc))
    in loop (length vector - 1, init) end

  fun foldl f init vector =
    foldli (fn (_, elem, acc) => f (elem, acc)) init vector

  fun foldr f init vector =
    foldri (fn (_, elem, acc) => f (elem, acc)) init vector

  fun findi pred vector =
    let
      fun loop i =
        if i = length vector then NONE
        else
          let val elem = sub (vector, i)
          in if pred (i, elem) then SOME (i, elem) else loop (i + 1) end
    in loop 0 end

  fun find pred vector =
    case findi (fn (_, elem) => pred elem) vector of
      NONE => NONE
    | SOME (_, elem) => SOME elem

  fun exists pred vector =
    case findi (fn (_, elem) => pred elem) vector of
      NONE => false
    | SOME _ => true

  fun all pred vector =
    case findi (fn (_, elem) => not (pred elem)) vector of
      NONE => true
    | SOME _ => false

  fun collate cmp (left, right) = String.collate cmp (left, right)
end
