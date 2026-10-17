(* POLYML-SKIP: record shape must be resolved at the generalizing declaration; Poly/ML permits later uses. *)
(* SML'97 grammar, pattern: flex record later use (nassau-ugc.21). *)
fun pi n = print (Int.toString n ^ "\n")
fun f {a, ...} = a + 0
val _ = pi (f {a = 1, b = 2})
(* CHECK-ERR: × unresolved flex record (can't tell what fields there are besides #a) *)
(* CHECK-ERR: :4:1] *)
