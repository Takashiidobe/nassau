structure Word8 =
struct
  type word = word8

  val wordSize = 8

  fun fromInt (i: int) : word =
    Prim.word8OfInt (Int.mod (i, 256))

  fun toInt (w: word) : int = Prim.intOfWord8 w

  fun toIntX w =
    let val n = toInt w
    in if n >= 128 then n - 256 else n end

  fun fromLargeInt i = fromInt (Int.fromLarge i)
  fun toLargeInt w = Int.toLarge (toInt w)
  fun toLargeIntX w = Int.toLarge (toIntX w)

  fun op + (a, b) = fromInt (toInt a + toInt b)
  fun op - (a, b) = fromInt (toInt a - toInt b)
  fun op * (a, b) = fromInt (toInt a * toInt b)
  fun op div (a, b) = fromInt (toInt a div toInt b)
  fun op mod (a, b) = fromInt (Int.mod (toInt a, toInt b))
  fun op ~ a = fromInt (~ (toInt a))

  fun compare (a, b) = Int.compare (toInt a, toInt b)
  fun op < (a, b) = toInt a < toInt b
  fun op <= (a, b) = toInt a <= toInt b
  fun op > (a, b) = toInt a > toInt b
  fun op >= (a, b) = toInt a >= toInt b

  fun min (a, b) = if a <= b then a else b
  fun max (a, b) = if a >= b then a else b

  fun bitwise f (a, b) =
    let
      fun loop (x, y, place, acc) =
        if place = 256 then fromInt acc
        else
          let
            val bit = f (Int.mod (x, 2), Int.mod (y, 2))
            val acc = if bit then acc + place else acc
          in
            loop (Int.div (x, 2), Int.div (y, 2), place * 2, acc)
          end
    in
      loop (toInt a, toInt b, 1, 0)
    end

  fun andb pair = bitwise (fn (a, b) => a = 1 andalso b = 1) pair
  fun orb pair = bitwise (fn (a, b) => a = 1 orelse b = 1) pair
  fun xorb pair = bitwise (fn (a, b) => a <> b) pair
  fun notb a = fromInt (255 - toInt a)

  fun fmt radix w = Int.fmt radix (toInt w)
  fun toString w = fmt StringCvt.HEX w

  fun scan radix getc source =
    let
      val base =
        case radix of
          StringCvt.BIN => 2
        | StringCvt.OCT => 8
        | StringCvt.DEC => 10
        | StringCvt.HEX => 16

      fun next state = getc state

      fun digit c =
        let val n = Prim.ord c
        in
          if n >= 48 andalso n <= 57 then n - 48
          else if n >= 65 andalso n <= 70 then n - 55
          else if n >= 97 andalso n <= 102 then n - 87
          else ~1
        end

      fun startsWithDigit state =
        case next state of
          SOME (c, _) => let val d = digit c in d >= 0 andalso d < base end
        | NONE => false

      fun skipWS state =
        case next state of
          SOME (c, rest) =>
            let val n = Prim.ord c
            in if n = 32 orelse (n >= 9 andalso n <= 13) then skipWS rest else state end
        | NONE => state

      fun afterZero state =
        case next state of
          SOME (#"0", rest) => SOME rest
        | _ => NONE

      fun afterW state =
        case next state of
          SOME (#"w", rest) => SOME rest
        | _ => NONE

      fun afterHex state =
        case next state of
          SOME (#"x", rest) => SOME rest
        | SOME (#"X", rest) => SOME rest
        | _ => NONE

      fun prefixed state =
        case afterZero state of
          NONE => state
        | SOME zeroRest =>
            (case afterW zeroRest of
               SOME wRest =>
                 let
                   val candidate =
                     if radix = StringCvt.HEX then
                       (case afterHex wRest of SOME hexRest => hexRest | NONE => wRest)
                     else wRest
                 in if startsWithDigit candidate then candidate else state end
             | NONE =>
                 if radix = StringCvt.HEX then
                   (case afterHex zeroRest of
                      SOME hexRest => if startsWithDigit hexRest then hexRest else state
                    | NONE => state)
                 else state)

      val start = prefixed (skipWS source)

      fun consume (state, acc, seen) =
        case next state of
          SOME (c, rest) =>
            let val d = digit c
            in
              if d < 0 orelse d >= base then
                if seen then SOME (fromInt acc, state) else NONE
              else if acc > (255 - d) div base then raise Overflow
              else consume (rest, acc * base + d, true)
            end
        | NONE => if seen then SOME (fromInt acc, state) else NONE
    in
      consume (start, 0, false)
    end

  fun fromString s = StringCvt.scanString (scan StringCvt.HEX) s
end
