(* mlton regression/known-case1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)

fun nlist 0 = 0::nil
  | nlist n = n::(nlist (n-1))

val rec last =
   fn nil => 0
    | x::nil => x
    | y::x::nil => y
    | _ :: l => last l

val n = 1 + (last (nlist (10)))

val _ = print ((Int.toString n) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
