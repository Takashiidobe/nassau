structure IntInf =
struct
  datatype int = Z of bool * Int.int list

  val limbBits = 14
  val base = 16384

  fun strip [] = []
    | strip (x :: rest) =
        case strip rest of
          [] => if x = 0 then [] else [x]
        | tail => x :: tail

  fun cmpMag ([], []) = EQUAL
    | cmpMag ([], _) = LESS
    | cmpMag (_, []) = GREATER
    | cmpMag (a :: ra, b :: rb) =
        case cmpMag (ra, rb) of
          EQUAL => Int.compare (a, b)
        | other => other

  fun addMag (a, b) =
    let
      fun go ([], [], 0) = []
        | go ([], [], c) = [c]
        | go (x :: xs, [], c) = step (x, 0, xs, [], c)
        | go ([], y :: ys, c) = step (0, y, [], ys, c)
        | go (x :: xs, y :: ys, c) = step (x, y, xs, ys, c)
      and step (x, y, xs, ys, c) =
        let val s = x + y + c
        in (s mod base) :: go (xs, ys, s div base) end
    in go (a, b, 0) end

  fun subMag (a, b) =
    let
      fun go ([], _, _) = []
        | go (x :: xs, ys, borrow) =
            let
              val (y, rest) = case ys of [] => (0, []) | y :: r => (y, r)
              val d = x - y - borrow
            in
              if d < 0 then (d + base) :: go (xs, rest, 1) else d :: go (xs, rest, 0)
            end
    in strip (go (a, b, 0)) end

  fun mulSmall (a, k) =
    let
      fun go ([], c) = if c = 0 then [] else [c mod base] @ go ([], c div base)
        | go (x :: xs, c) =
            let val p = x * k + c in (p mod base) :: go (xs, p div base) end
    in if k = 0 then [] else go (a, 0) end

  fun mulMag ([], _) = []
    | mulMag (x :: xs, b) =
        strip (addMag (mulSmall (b, x), 0 :: mulMag (xs, b)))

  fun divSmall (a, k) =
    let
      fun go [] = ([], 0)
        | go (x :: xs) =
            let
              val (q, r) = go xs
              val cur = r * base + x
            in (cur div k :: q, cur mod k) end
      val (q, r) = go a
    in (strip q, r) end

  fun divModMag (a, b) =
    if cmpMag (a, b) = LESS then ([], a)
    else
      let
        fun digit (rem, bound) =
          let
            fun search (lo, hi) =
              if lo >= hi then lo
              else
                let val mid = (lo + hi + 1) div 2
                in
                  if cmpMag (mulSmall (b, mid), rem) = GREATER then search (lo, mid - 1)
                  else search (mid, hi)
                end
          in search (0, bound) end
        fun go ([], rem, acc) = (strip acc, rem)
          | go (x :: xs, rem, acc) =
              let
                val rem = strip (x :: rem)
                val q = digit (rem, base - 1)
              in go (xs, subMag (rem, mulSmall (b, q)), q :: acc) end
      in go (List.rev a, [], []) end

  fun make (neg, mag) =
    case strip mag of [] => Z (false, []) | m => Z (neg, m)

  fun magOfInt n =
    let
      fun go 0 = []
        | go m = (m mod base) :: go (m div base)
    in
      if n >= 0 then go n
      else addMag (go (~(n + 1)), [1])
    end

  fun fromInt n = make (n < 0, magOfInt n)

  fun toInt (Z (_, [])) = 0
    | toInt (Z (neg, mag)) =
        let
          val limit = magOfInt (if neg then valOf Int.minInt else valOf Int.maxInt)
        in
          if cmpMag (mag, limit) = GREATER then raise Overflow
          else
            let
              val negative = List.foldr (fn (limb, acc) => acc * base - limb) 0 mag
            in if neg then negative else ~negative end
        end

  val toLarge = toInt
  val fromLarge = fromInt
  val precision : Int.int option = NONE
  val minInt : int option = NONE
  val maxInt : int option = NONE

  fun negate (Z (neg, mag)) = make (not neg, mag)
  fun absolute (Z (_, mag)) = Z (false, mag)

  fun addSigned (Z (na, a), Z (nb, b)) =
    if na = nb then make (na, addMag (a, b))
    else
      case cmpMag (a, b) of
        EQUAL => Z (false, [])
      | GREATER => make (na, subMag (a, b))
      | LESS => make (nb, subMag (b, a))

  fun mulSigned (Z (na, a), Z (nb, b)) = make (na <> nb, mulMag (a, b))

  fun quotRemSigned (Z (na, a), Z (nb, b)) =
    if null b then raise Div
    else
      let val (q, r) = divModMag (a, b)
      in (make (na <> nb, q), make (na, r)) end

  fun isZero (Z (_, m)) = null m

  fun divModSigned (x, y) =
    let val (q, r) = quotRemSigned (x, y)
    in
      if isZero r orelse (case (r, y) of (Z (nr, _), Z (ny, _)) => nr = ny) then (q, r)
      else (addSigned (q, fromInt ~1), addSigned (r, y))
    end

  fun compareSigned (Z (na, a), Z (nb, b)) =
    case (na, nb) of
      (false, true) => GREATER
    | (true, false) => LESS
    | (false, false) => cmpMag (a, b)
    | (true, true) => cmpMag (b, a)

  fun signOf (Z (_, [])) = 0
    | signOf (Z (true, _)) = ~1
    | signOf _ = 1

  fun powerOfTwo n =
    let
      val limbs = n div limbBits
      val rest = n mod limbBits
      fun zeros 0 = [] | zeros k = 0 :: zeros (k - 1)
      fun shifted 0 = 1 | shifted k = 2 * shifted (k - 1)
    in make (false, zeros limbs @ [shifted rest]) end

  fun twos (Z (neg, mag), n) =
    let
      fun pad (xs, 0) = []
        | pad ([], k) = 0 :: pad ([], k - 1)
        | pad (x :: xs, k) = x :: pad (xs, k - 1)
    in
      if not neg then pad (mag, n)
      else List.map (fn limb => base - 1 - limb) (pad (subMag (mag, [1]), n))
    end

  fun bitwise f (a as Z (na, ma), b as Z (nb, mb)) =
    let
      val n = Int.max (length ma, length mb) + 1
      val limbs = ListPair.map (fn (x, y) => f (x, y)) (twos (a, n), twos (b, n))
      val ext = f (if na then base - 1 else 0, if nb then base - 1 else 0)
    in
      if ext = 0 then make (false, limbs)
      else make (true, addMag (List.map (fn limb => base - 1 - limb) limbs, [1]))
    end

  fun bitOp op1 (x, y) =
    let
      fun go (0, _, acc) = acc
        | go (k, (xa, ya), acc) =
            go (k - 1, (xa div 2, ya div 2), acc * 2 + op1 (xa mod 2, ya mod 2))
    in go (limbBits, (x, y), 0) end

  fun reverseBits (x) =
    let
      fun go (0, v, acc) = acc
        | go (k, v, acc) = go (k - 1, v div 2, acc * 2 + v mod 2)
    in go (limbBits, x, 0) end

  fun limbwise op1 (x, y) = reverseBits (bitOp op1 (x, y))

  fun digitChar d = if d < 10 then Char.chr (d + 48) else Char.chr (d + 55)

  fun radixOf StringCvt.BIN = 2
    | radixOf StringCvt.OCT = 8
    | radixOf StringCvt.DEC = 10
    | radixOf StringCvt.HEX = 16

  fun fmt radix (Z (neg, mag)) =
    let
      val b = radixOf radix
      fun digits ([], acc) = acc
        | digits (m, acc) =
            let val (q, r) = divSmall (m, b) in digits (q, digitChar r :: acc) end
      val body = case digits (mag, []) of [] => "0" | ds => String.implode ds
    in (if neg then "~" else "") ^ body end

  fun toString x = fmt StringCvt.DEC x

  fun scan radix getc source =
    let
      val b = radixOf radix
      fun skipWS state =
        case getc state of
          SOME (c, rest) => if Char.isSpace c then skipWS rest else state
        | NONE => state
      fun digit c =
        let val n = Char.ord c
        in
          if n >= 48 andalso n <= 57 then n - 48
          else if n >= 65 andalso n <= 70 then n - 55
          else if n >= 97 andalso n <= 102 then n - 87
          else ~1
        end
      fun valid c = let val d = digit c in d >= 0 andalso d < b end
      fun consume (state, acc, seen) =
        case getc state of
          SOME (c, rest) =>
            if valid c then consume (rest, addMag (mulSmall (acc, b), magOfInt (digit c)), true)
            else if seen then SOME (acc, state) else NONE
        | NONE => if seen then SOME (acc, state) else NONE
      val start = skipWS source
      val (neg, body) =
        case getc start of
          SOME (c, rest) => if c = #"~" orelse c = #"-" then (true, rest) else (false, start)
        | NONE => (false, start)
      val body =
        if radix = StringCvt.HEX then
          case getc body of
            SOME (#"0", r1) =>
              (case getc r1 of
                 SOME (c, r2) =>
                   if (c = #"x" orelse c = #"X") then
                     (case getc r2 of SOME (d, _) => if valid d then r2 else body | NONE => body)
                   else body
               | NONE => body)
          | _ => body
        else body
    in
      case consume (body, [], false) of
        SOME (mag, rest) => SOME (make (neg, mag), rest)
      | NONE => NONE
    end

  fun fromString s = StringCvt.scanString (scan StringCvt.DEC) s

  fun pow (x, n) =
    if n < 0 then raise Domain
    else
      let
        fun go (acc, _, 0) = acc
          | go (acc, sq, k) =
              go (if k mod 2 = 1 then mulSigned (acc, sq) else acc, mulSigned (sq, sq), k div 2)
      in go (fromInt 1, x, n) end

  fun log2 (Z (neg, mag)) =
    if neg orelse null mag then raise Domain
    else
      let
        val top = List.last mag
        fun bits (0, acc) = acc | bits (v, acc) = bits (v div 2, acc + 1)
      in (length mag - 1) * limbBits + bits (top, 0) - 1 end

  fun shiftLeft (x, n) = mulSigned (x, powerOfTwo (Word.toInt n))
  fun shiftRight (x, n) = #1 (divModSigned (x, powerOfTwo (Word.toInt n)))

  fun op + (a, b) = addSigned (a, b)
  fun op - (a, b) = addSigned (a, negate b)
  fun op * (a, b) = mulSigned (a, b)
  fun op div (a, b) = #1 (divModSigned (a, b))
  fun op mod (a, b) = #2 (divModSigned (a, b))
  fun quot (a, b) = #1 (quotRemSigned (a, b))
  fun rem (a, b) = #2 (quotRemSigned (a, b))
  fun divMod (a, b) = divModSigned (a, b)
  fun quotRem (a, b) = quotRemSigned (a, b)
  fun op ~ a = negate a
  fun abs a = absolute a
  fun compare (a, b) = compareSigned (a, b)
  fun op < (a, b) = compareSigned (a, b) = LESS
  fun op <= (a, b) = compareSigned (a, b) <> GREATER
  fun op > (a, b) = compareSigned (a, b) = GREATER
  fun op >= (a, b) = compareSigned (a, b) <> LESS
  fun min (a, b) = if compareSigned (a, b) = GREATER then b else a
  fun max (a, b) = if compareSigned (a, b) = LESS then b else a
  fun sign a = signOf a
  fun sameSign (a, b) = signOf a = signOf b

  fun andb (a, b) = bitwise (limbwise (fn (x, y) => if x = 1 andalso y = 1 then 1 else 0)) (a, b)
  fun orb (a, b) = bitwise (limbwise (fn (x, y) => if x = 1 orelse y = 1 then 1 else 0)) (a, b)
  fun xorb (a, b) = bitwise (limbwise (fn (x, y) => if x <> y then 1 else 0)) (a, b)
  fun notb a = addSigned (negate a, fromInt ~1)
  fun << (a, n) = shiftLeft (a, n)
  fun ~>> (a, n) = shiftRight (a, n)
end
