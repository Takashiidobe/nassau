exception Chr

structure Char =
struct
  val maxOrd = 255

  val ord = Prim.ord

  fun chr i =
    if i < 0 orelse i > maxOrd then raise Chr else Prim.chr i
end

val ord = Char.ord
val chr = Char.chr
