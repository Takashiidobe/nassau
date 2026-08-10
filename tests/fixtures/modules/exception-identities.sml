structure A = struct exception E of int exception N fun fail n = raise E n end
structure B = struct exception E of int exception N end
structure Alias = A
exception Copy = A.E
val () = print ((A.fail 3 handle B.E _ => "wrong" | Alias.E n => Int.toString n) ^ "\n")
val () = print (((raise Copy 4) handle A.E n => Int.toString n) ^ "\n")
val () = print (((raise B.N) handle A.N => "wrong" | B.N => "distinct") ^ "\n")
structure Fresh = let exception E of int in
  struct val value = E 7 fun test (E n) = n | test _ = ~1 end
end
structure Other = let exception E of int in struct val value = E 7 end end
val () = print (Int.toString (Fresh.test Fresh.value) ^ " " ^ Int.toString (Fresh.test Other.value) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: distinct *)
(* CHECK-STDOUT-NEXT: 7 ~1 *)
