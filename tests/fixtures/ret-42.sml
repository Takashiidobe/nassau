(* exit_code: 42 *)
val _ = print "hello, world\n"
val _ = Posix.Process.exit (Word8.fromInt 42)
