fun select (SOME (x, _)) = x
val _ = print (Int.toString (select (SOME (4, true))) ^ "\n")
val _ = print ((Int.toString (select NONE)) handle Match => "Match\n")
val _ = print ((let val SOME x = NONE in x end) handle Bind => "Bind\n")
val r = ref (SOME 7)
val ref (SOME n) = r
val _ = print (Int.toString n ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
(* CHECK-STDOUT-NEXT: Match *)
(* CHECK-STDOUT-NEXT: Bind *)
(* CHECK-STDOUT-NEXT: 7 *)
