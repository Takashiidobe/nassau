(* Arithmetic up to the limits of a 31-bit int does not overflow. *)
val largest = 1073741823
val smallest = ~1073741824
val a = (largest - 1) + 1
val b = (smallest + 1) - 1
val c = smallest div ~2
val d = smallest div 1073741823
val ok = if a = largest then if b = smallest then c = 536870912 else false else false
val _ = Posix.Process.exit (Word8.fromInt (if ok then d + 9 else 1))
(* CHECK-EXIT: 7 *)
