(* SML'97 grammar, type: record. *)
fun pi n = print (Int.toString n ^ "\n")
val r : {a : int, b : string} = {a = 1, b = ""}
val _ = pi (#a r)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
