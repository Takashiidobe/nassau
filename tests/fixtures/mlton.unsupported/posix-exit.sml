(* mlton regression/posix-exit.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ = (TextIO.output (TextIO.stdOut, "hello")
         ; Posix.Process.exit 0w0)
(* CHECK-EXIT: 0 *)
