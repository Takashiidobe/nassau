(* mlton regression/fact.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val rec fact =
   fn 0 => 1
    | n => n * fact(n - 1)

val _ = print(concat[Int.toString(fact 10), "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3628800 *)
