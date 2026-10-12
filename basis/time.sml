local
  fun add (a: real, b: real) = a + b
  fun sub (a: real, b: real) = a - b
  fun lt (a: real, b: real) = a < b
  fun le (a: real, b: real) = a <= b
  fun gt (a: real, b: real) = a > b
  fun ge (a: real, b: real) = a >= b
in
structure Time :>
sig
  type time
  val zeroTime : time
  val fromReal : real -> time
  val toReal : time -> real
  val toSeconds : time -> LargeInt.int
  val toMilliseconds : time -> LargeInt.int
  val toMicroseconds : time -> LargeInt.int
  val fromSeconds : LargeInt.int -> time
  val fromMilliseconds : LargeInt.int -> time
  val fromMicroseconds : LargeInt.int -> time
  val + : time * time -> time
  val - : time * time -> time
  val compare : time * time -> order
  val < : time * time -> bool
  val <= : time * time -> bool
  val > : time * time -> bool
  val >= : time * time -> bool
  val now : unit -> time
end =
struct
  type time = real

  val zeroTime = 0.0
  fun fromReal r = r
  fun toReal r = r

  fun truncate r =
    if r < 0.0 then ~(Prim.realFloor (~r)) else Prim.realFloor r

  fun toSeconds t = truncate t
  fun toMilliseconds t = truncate (t * 1000.0)
  fun toMicroseconds t = truncate (t * 1000000.0)
  fun fromSeconds n = Prim.intToReal n
  fun fromMilliseconds n = Prim.intToReal n / 1000.0
  fun fromMicroseconds n = Prim.intToReal n / 1000000.0

  fun a + b = add (a, b)
  fun a - b = sub (a, b)
  fun compare (a, b) = if lt (a, b) then LESS else if gt (a, b) then GREATER else EQUAL
  fun a < b = lt (a, b)
  fun a <= b = le (a, b)
  fun a > b = gt (a, b)
  fun a >= b = ge (a, b)

  fun now () = Prim.osNow ()
end
end
