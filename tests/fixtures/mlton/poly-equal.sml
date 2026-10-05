(* mlton regression/poly-equal.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ =
   print(if ([1, 2, 3], [4, 5]) = ([1, 2, 3], [4])
            then "true\n"
         else "false\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: false *)
