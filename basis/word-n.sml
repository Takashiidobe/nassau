functor MaskedWord (val wordSize : int) =
struct
  type word = Word.word

  val wordSize = wordSize
  val mask : word = Word.<< (0w1, Word.fromInt wordSize) - 0w1
  fun trim (w: word) : word = Word.andb (w, mask)

  fun fromInt i = trim (Word.fromInt i)
  fun toInt w = Word.toInt w
  fun toIntX w =
    if Word.andb (w, Word.<< (0w1, Word.fromInt (wordSize - 1))) = 0w0 then Word.toInt w
    else Word.toIntX (Word.orb (w, Word.notb mask))
  fun fromLargeInt i = fromInt (Int.fromLarge i)
  fun toLargeInt w = Int.toLarge (toInt w)
  fun toLargeIntX w = Int.toLarge (toIntX w)

  fun op + (a, b) = trim (Word.+ (a, b))
  fun op - (a, b) = trim (Word.- (a, b))
  fun op * (a, b) = trim (Word.* (a, b))
  fun op div (a, b) = Word.div (a, b)
  fun op mod (a, b) = Word.mod (a, b)
  fun op ~ a = trim (Word.~ a)

  val compare = Word.compare
  val op < = Word.<
  val op <= = Word.<=
  val op > = Word.>
  val op >= = Word.>=
  val min = Word.min
  val max = Word.max

  val andb = Word.andb
  val orb = Word.orb
  val xorb = Word.xorb
  fun notb a = trim (Word.notb a)
  fun << (a, n) = if Word.>= (n, Word.fromInt wordSize) then 0w0 else trim (Word.<< (a, n))
  fun >> (a, n) = if Word.>= (n, Word.fromInt wordSize) then 0w0 else Word.>> (a, n)
  fun ~>> (a, n) =
    let
      val n = if Word.>= (n, Word.fromInt wordSize) then Word.fromInt (wordSize - 1) else n
      val negative = Word.andb (a, Word.<< (0w1, Word.fromInt (wordSize - 1))) <> 0w0
    in
      if negative then trim (Word.orb (Word.>> (a, n), Word.notb (Word.>> (mask, n)))) else Word.>> (a, n)
    end

  val fmt = Word.fmt
  val toString = Word.toString
  fun scan radix getc source =
    case Word.scan radix getc source of
      SOME (w, rest) => if Word.> (w, mask) then raise Overflow else SOME (w, rest)
    | NONE => NONE
  fun fromString s = StringCvt.scanString (scan StringCvt.HEX) s
end

structure Word16 = MaskedWord (val wordSize = 16)
structure Word32 = MaskedWord (val wordSize = 32)
structure LargeWord = Word
