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
  val fmt : int -> time -> string
  val toString : time -> string
  val scan : (char, 'a) StringCvt.reader -> (time, 'a) StringCvt.reader
  val fromString : string -> time option
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

  fun fmt n t =
    if n < 0 then raise Size
    else
      let
        fun pow10 0 = 1 | pow10 k = 10 * pow10 (k - 1)
        val scale = pow10 n
        val negative = t < 0.0
        val magnitude = if negative then ~t else t
        val ticks = Prim.realFloor (magnitude * Prim.intToReal scale + 0.5)
        val whole = Int.toString (ticks div scale)
        val frac = Int.toString (ticks mod scale)
        fun pad s = if size s < n then pad ("0" ^ s) else s
      in
        (if negative then "~" else "") ^ whole ^ (if n = 0 then "" else "." ^ pad frac)
      end

  fun toString t = fmt 3 t

  fun scan getc source =
    let
      fun isDigit c = c >= #"0" andalso c <= #"9"
      fun skipWS state =
        case getc state of
          SOME (c, rest) => if Char.isSpace c then skipWS rest else state
        | NONE => state
      fun digits (state, acc, seen) =
        case getc state of
          SOME (c, rest) =>
            if isDigit c then digits (rest, acc * 10.0 + Prim.intToReal (Prim.ord c - 48), true)
            else (acc, seen, state)
        | NONE => (acc, seen, state)
      fun fraction (state, scale, acc, seen) =
        case getc state of
          SOME (c, rest) =>
            if isDigit c then
              fraction (rest, scale / 10.0, acc + scale * Prim.intToReal (Prim.ord c - 48), true)
            else (acc, seen, state)
        | NONE => (acc, seen, state)
      val start = skipWS source
      val (negative, body) =
        case getc start of
          SOME (c, rest) => if c = #"~" orelse c = #"-" then (true, rest) else (false, start)
        | NONE => (false, start)
      val (whole, wholeSeen, afterWhole) = digits (body, 0.0, false)
      val (value, seen, finish) =
        case getc afterWhole of
          SOME (#".", rest) =>
            let val (part, fracSeen, last) = fraction (rest, 0.1, 0.0, false)
            in
              if wholeSeen orelse fracSeen then (whole + part, true, last) else (0.0, false, afterWhole)
            end
        | _ => (whole, wholeSeen, afterWhole)
    in
      if seen then SOME (if negative then ~value else value, finish) else NONE
    end

  fun fromString s = StringCvt.scanString scan s
end
end
