structure Word =
struct
  type word = word

  val wordSize = 63

  fun fromInt (i: int) : word = Prim.wordOfInt i

  val largestInt : word = 0w4611686018427387903

  fun toInt (w: word) : int =
    if w > largestInt then raise Overflow else Prim.intOfWord w

  fun notb (w: word) : word = Prim.wordXor (w, fromInt (Int.~ 1))
  fun andb (a: word, b: word) : word = Prim.wordAnd (a, b)
  fun orb (a: word, b: word) : word = Prim.wordOr (a, b)
  fun xorb (a: word, b: word) : word = Prim.wordXor (a, b)

  fun << (w: word, n: word) : word = Prim.wordShl (w, n)
  fun >> (w: word, n: word) : word = Prim.wordLShr (w, n)
  fun ~>> (w: word, n: word) : word = Prim.wordAShr (w, n)

  fun toIntX (w: word) : int =
    if w <= largestInt then Prim.intOfWord w
    else if notb w <= largestInt then Int.~ (Prim.intOfWord (notb w)) - 1
    else raise Overflow

  fun fromLargeInt i = fromInt (Int.fromLarge i)
  fun toLargeInt w = Int.toLarge (toInt w)
  fun toLargeIntX w = Int.toLarge (toIntX w)

  fun op + (a: word, b: word) : word = a + b
  fun op - (a: word, b: word) : word = a - b
  fun op * (a: word, b: word) : word = a * b
  fun op div (a: word, b: word) : word = a div b
  fun op mod (a: word, b: word) : word = a mod b
  fun op ~ (a: word) : word = notb a + 0w1

  fun op < (a: word, b: word) = a < b
  fun op <= (a: word, b: word) = a <= b
  fun op > (a: word, b: word) = a > b
  fun op >= (a: word, b: word) = a >= b

  fun compare (a: word, b: word) =
    if a < b then LESS else if a = b then EQUAL else GREATER

  fun min (a: word, b: word) = if a <= b then a else b
  fun max (a: word, b: word) = if a >= b then a else b

  fun base radix =
    case radix of
      StringCvt.BIN => 0w2
    | StringCvt.OCT => 0w8
    | StringCvt.DEC => 0w10
    | StringCvt.HEX => 0w16

  fun digitChar (n: word) =
    let val d = Prim.intOfWord n
    in if d < 10 then Prim.chr (d + 48) else Prim.chr (d + 55) end

  fun fmt radix (w: word) =
    let
      val b = base radix
      fun digits (n: word, acc) =
        let val acc = digitChar (n mod b) :: acc
        in if n < b then acc else digits (n div b, acc) end
      fun join [] = ""
        | join (c :: rest) = Prim.charToString c ^ join rest
    in
      join (digits (w, []))
    end

  fun toString w = fmt StringCvt.HEX w

  fun scan radix getc source =
    let
      val b = base radix
      val largest = notb 0w0

      fun digit c =
        let val n = Prim.ord c
        in
          if n >= 48 andalso n <= 57 then n - 48
          else if n >= 65 andalso n <= 70 then n - 55
          else if n >= 97 andalso n <= 102 then n - 87
          else Int.~ 1
        end

      fun valid c = let val d = digit c in d >= 0 andalso fromInt d < b end

      fun startsWithDigit state =
        case getc state of
          SOME (c, _) => valid c
        | NONE => false

      fun skipWS state =
        case getc state of
          SOME (c, rest) =>
            let val n = Prim.ord c
            in if n = 32 orelse (n >= 9 andalso n <= 13) then skipWS rest else state end
        | NONE => state

      fun afterChar (ok, state) =
        case getc state of
          SOME (c, rest) => if ok c then SOME rest else NONE
        | NONE => NONE

      fun isX c = c = #"x" orelse c = #"X"

      fun prefixed state =
        case afterChar (fn c => c = #"0", state) of
          NONE => state
        | SOME zeroRest =>
            let
              val wRest = afterChar (fn c => c = #"w", zeroRest)
              val afterW = case wRest of SOME rest => rest | NONE => zeroRest
              val candidate =
                if radix = StringCvt.HEX then
                  (case afterChar (isX, afterW) of SOME rest => rest | NONE => afterW)
                else afterW
            in
              if startsWithDigit candidate then candidate else state
            end

      fun consume (state, acc: word, seen) =
        case getc state of
          SOME (c, rest) =>
            let val d = digit c
            in
              if d < 0 orelse fromInt d >= b then
                if seen then SOME (acc, state) else NONE
              else if acc > (largest - fromInt d) div b then raise Overflow
              else consume (rest, acc * b + fromInt d, true)
            end
        | NONE => if seen then SOME (acc, state) else NONE
    in
      consume (prefixed (skipWS source), 0w0, false)
    end

  fun fromString s = StringCvt.scanString (scan StringCvt.HEX) s
end

structure SysWord = Word
