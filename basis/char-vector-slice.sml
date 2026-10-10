structure CharVectorSlice :> MONO_VECTOR_SLICE
where type slice = Substring.substring
where type vector = String.string
where type elem = char =
struct
  type elem = char
  type vector = String.string
  type slice = Substring.substring

  val length = Substring.size
  val sub = Substring.sub
  val full = Substring.full
  val slice = Substring.extract
  val subslice = Substring.slice
  val base = Substring.base
  val vector = Substring.string
  val concat = Substring.concat
  val isEmpty = Substring.isEmpty
  val getItem = Substring.getc
  val app = Substring.app
  val foldl = Substring.foldl
  val foldr = Substring.foldr
  val collate = Substring.collate

  fun appi f slice =
    let
      fun loop i =
        if i = length slice then ()
        else (f (i, sub (slice, i)); loop (i + 1))
    in loop 0 end

  fun mapi f slice =
    CharVector.tabulate (length slice, fn i => f (i, sub (slice, i)))

  fun map f slice = mapi (fn (_, elem) => f elem) slice

  fun foldli f init slice =
    let
      fun loop (i, acc) =
        if i = length slice then acc
        else loop (i + 1, f (i, sub (slice, i), acc))
    in loop (0, init) end

  fun foldri f init slice =
    let
      fun loop (i, acc) =
        if i < 0 then acc
        else loop (i - 1, f (i, sub (slice, i), acc))
    in loop (length slice - 1, init) end

  fun findi pred slice =
    let
      fun loop i =
        if i = length slice then NONE
        else
          let val elem = sub (slice, i)
          in if pred (i, elem) then SOME (i, elem) else loop (i + 1) end
    in loop 0 end

  fun find pred slice =
    case findi (fn (_, elem) => pred elem) slice of
      NONE => NONE
    | SOME (_, elem) => SOME elem

  fun exists pred slice =
    case findi (fn (_, elem) => pred elem) slice of
      NONE => false
    | SOME _ => true

  fun all pred slice =
    case findi (fn (_, elem) => not (pred elem)) slice of
      NONE => true
    | SOME _ => false
end
