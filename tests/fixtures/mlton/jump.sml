(* mlton regression/jump.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)

fun doit 2 = "zero"
  | doit 6 = "one" 
  | doit 10 = "two"
  | doit 14 = "three"
  | doit 18 = "four"
  | doit 22 = "five"
  | doit 26 = "six"
  | doit 30 = "seven"
  | doit 34 = "eight"
  | doit 38 = "nine"
  | doit 42 = "ten"
  | doit _ = "big"

val l = List.tabulate(50,fn i => i)

val _ = List.app (fn i => print ((doit i) ^ "\n")) l
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: zero *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: one *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: two *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: three *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: four *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: five *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: six *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: seven *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: eight *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: nine *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: ten *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
(* CHECK-STDOUT-NEXT: big *)
