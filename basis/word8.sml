structure Word8 =
struct
  type word = word8

  fun fromInt (i: int) : word =
    Prim.word8OfInt (i mod 256)

  fun toInt (w: word) : int = Prim.intOfWord8 w
end
