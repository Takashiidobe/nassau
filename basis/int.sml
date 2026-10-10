structure Int =
struct
  type int = int

  val precision = SOME 31
  val minInt = SOME (~1073741824)
  val maxInt = SOME 1073741823

  fun toLarge (i: int) = i
  fun fromLarge (i: int) = i
  fun toInt (i: int) = i
  fun fromInt (i: int) = i

  val op + = op +
  val op - = op -
  val op * = op *
  val op div = op div
  val op mod = op mod
  val op ~ = op ~
  val op < = op <
  val op <= = op <=
  val op > = op >
  val op >= = op >=

  fun quot (a, b) =
    let
      val q = a div b
      val r = a mod b
    in
      if (a < 0) <> (b < 0) andalso r <> 0 then q + 1 else q
    end

  fun rem (a, b) =
    let val r = a mod b
    in if (a < 0) <> (b < 0) andalso r <> 0 then r - b else r
    end

  fun compare (a: int, b) =
    if a < b then LESS else if a = b then EQUAL else GREATER

  fun min (a: int, b) =
    if a < b then a else b

  fun max (a: int, b) =
    if a < b then b else a

  fun abs (a: int) =
    if a < 0 then ~a else a

  fun sign i = if i < 0 then ~1 else if i > 0 then 1 else 0

  fun sameSign (a, b) = sign a = sign b

  fun radixValue StringCvt.BIN = 2
    | radixValue StringCvt.OCT = 8
    | radixValue StringCvt.DEC = 10
    | radixValue StringCvt.HEX = 16

  fun digit c =
    let val n = Prim.ord c
    in
      if n >= 48 andalso n <= 57 then n - 48
      else if n >= 65 andalso n <= 70 then n - 55
      else if n >= 97 andalso n <= 102 then n - 87
      else ~1
    end

  fun digitChar n =
    if n < 10 then Prim.chr (n + 48) else Prim.chr (n + 55)

  fun fmt radix value =
    let
      val base = radixValue radix
      val negative = value < 0
      fun digits n acc =
        let
          val remainder = rem (n, base)
          val quotient = quot (n, base)
          val c = digitChar (abs remainder)
          val acc = c :: acc
        in
          if quotient = 0 then acc else digits quotient acc
        end
      val chars = digits value []
      val chars = if negative then #"~" :: chars else chars
      fun join [] = ""
        | join (c :: rest) = Prim.charToString c ^ join rest
    in
      join chars
    end

  val toString = fmt StringCvt.DEC

  fun scan radix getc source =
    let
      val base = radixValue radix
      fun next state = getc state
      fun after state =
        case next state of SOME (_, rest) => rest | NONE => state
      fun skip state =
        case next state of
          SOME (c, rest) =>
            let val n = Prim.ord c
            in
              if n = 32 orelse (n >= 9 andalso n <= 13)
              then skip rest
              else (c, state)
            end
        | NONE => (#"\000", state)
      val (first, start) = skip source
      val (negative, afterSign) =
        case first of
          #"~" => (true, after start)
        | #"-" => (true, after start)
        | #"+" => (false, after start)
        | _ => (false, start)
      val limit = if negative then ~1073741824 else ~1073741823
      fun add (acc, d) =
        if acc < limit div base then raise Overflow
        else
          let val product = acc * base
          in
            if product < limit + d then raise Overflow
            else product - d
          end
      val initial =
        case next afterSign of
          SOME (#"0", afterZero) =>
            if radix = StringCvt.HEX then
              (case next afterZero of
                 SOME (#"x", afterPrefix) =>
                   (case next afterPrefix of
                      SOME (c, afterDigit) =>
                        let val d = digit c
                        in if d >= 0 andalso d < base
                           then SOME (add (0, d), afterDigit)
                           else SOME (0, afterZero)
                        end
                    | NONE => SOME (0, afterZero))
               | SOME (#"X", afterPrefix) =>
                   (case next afterPrefix of
                      SOME (c, afterDigit) =>
                        let val d = digit c
                        in if d >= 0 andalso d < base
                           then SOME (add (0, d), afterDigit)
                           else SOME (0, afterZero)
                        end
                    | NONE => SOME (0, afterZero))
               | _ => SOME (0, afterZero))
            else SOME (0, afterZero)
        | SOME (c, rest) =>
            let val d = digit c
            in if d >= 0 andalso d < base
               then SOME (add (0, d), rest)
               else NONE
            end
        | NONE => NONE
      fun consume (state, acc, seen) =
        case next state of
          SOME (c, rest) =>
            let val d = digit c
            in
              if d < 0 orelse d >= base then
                if seen then SOME (if negative then acc else ~acc, state)
                else NONE
              else consume (rest, add (acc, d), true)
            end
        | NONE =>
            if seen then SOME (if negative then acc else ~acc, state)
            else NONE
    in
      case initial of
        NONE => NONE
      | SOME (acc, state) => consume (state, acc, true)
    end

  fun fromString s =
    StringCvt.scanString (scan StringCvt.DEC) s
end

structure LargeInt = Int
structure Position = Int
