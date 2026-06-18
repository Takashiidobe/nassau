(* exit_code: 42 *)
val _ = print "hello, world\n"
val answer = 40 + 2
val _ = Posix.Process.exit (Word8.fromInt answer)
