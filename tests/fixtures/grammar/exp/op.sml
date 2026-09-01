(* SML'97 grammar, expression: op. *)
fun pi n = print (Int.toString n ^ "\n")
infix add
fun a add b = a + b
val cons = op ::
val _ = pi (length (cons (1, [])))
val _ = pi (op add (1, 2))
val _ = print (if op = (1, 1) then "eq\n" else "ne\n")
val r = ref 0
val _ = op := (r, 5)
val _ = pi (!r)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: eq *)
(* CHECK-STDOUT-NEXT: 5 *)
