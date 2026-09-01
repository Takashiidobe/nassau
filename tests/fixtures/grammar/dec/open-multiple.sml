(* SML'97 grammar, declaration: open multiple. *)
fun pi n = print (Int.toString n ^ "\n")
structure A = struct val x = 1 end structure B = struct val y = 2 end
open A B val _ = pi (x + y)
val _ = pi (let open A in x end)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 1 *)
