fun pi n = print (Int.toString n ^ "\n")
val () = pi (Word8.toInt (Word8.fromInt 300))
val () = pi (Word8.toInt (Word8.fromInt ~1))
val () = pi (Word8.toInt (Word8.fromInt 255))
fun leave code = Posix.Process.exit (Word8.fromInt code)
val () = print (if 1 + 1 = 2 then "leaving\n" else "never\n")
val _ = leave 300
val () = print "unreachable\n"
(* CHECK-EXIT: 44 *)
(* CHECK-STDOUT: 44 *)
(* CHECK-STDOUT-NEXT: 255 *)
(* CHECK-STDOUT-NEXT: 255 *)
(* CHECK-STDOUT-NEXT: leaving *)
