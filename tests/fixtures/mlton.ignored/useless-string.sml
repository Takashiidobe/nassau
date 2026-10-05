(* mlton regression/useless-string.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML exits with status 1 *)
val x = "abc"
val y = "defg"
val _ =
   if 3 = (String.size
           (if 0 = length (CommandLine.arguments ())
               then x
            else y))
      then ()
   else raise Fail "bug"
