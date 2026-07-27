(* = on strings and lists compares structure; on ints, the value. *)
val strings = "abc" = "abc"
val differentStrings = "abc" = "abd"
val lists = [1, 2] = [1, 2]
val nested = [[1], []] <> [[1], [2]]
val longer = [1] = [1, 2]
val ints = 3 = 3
val score =
  (if strings then 1 else 0) + (if differentStrings then 2 else 0) + (if lists then 4 else 0)
  + (if nested then 8 else 0) + (if longer then 16 else 0) + (if ints then 32 else 0)
val _ = Posix.Process.exit (Word8.fromInt score)
(* CHECK-EXIT: 45 *)
