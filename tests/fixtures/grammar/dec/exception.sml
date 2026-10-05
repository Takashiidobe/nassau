(* SML'97 grammar, declaration: exception. *)
fun pi n = print (Int.toString n ^ "\n")
exception A exception B of int and C
exception D = B
structure S = struct exception E end exception F = S.E
val _ = pi ((raise D 3) handle B n => n)
val _ = pi ((raise F) handle S.E => 4)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 4 *)
