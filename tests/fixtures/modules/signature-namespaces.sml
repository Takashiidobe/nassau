signature S = sig type t val t:t structure t:sig val t:int end end
structure M:S = struct type t=int val t=1 structure t=struct val t=2 end end
val _ = print (Int.toString (M.t + M.t.t) ^ "\n")
signature A = sig val x:int end
signature B = sig val y:bool end
signature Both = sig include A B end
structure N:Both = struct val x=3 val y=true end
val _ = print (Int.toString N.x ^ Bool.toString N.y ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 3true *)
