structure Int =
struct
  val toString = Prim.intToString

  fun compare (a: int, b) =
    if a < b then LESS else if a = b then EQUAL else GREATER

  fun min (a: int, b) =
    if a < b then a else b

  fun max (a: int, b) =
    if a < b then b else a

  fun abs (a: int) =
    if a < 0 then ~a else a
end
