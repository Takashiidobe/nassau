val text = "\000\127\128\255"
val _ = print (Int.toString (size text) ^ "\n")
val _ = print (if text = "\000\127\128\255" andalso size "\255" = 1 then "equal\n" else "different\n")
val _ = Posix.Process.exit (Word8.fromInt 0)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
(* CHECK-STDOUT-NEXT: equal *)
