structure M = struct
  datatype d = A | B of int
  val v = B 1
  fun value (B n) = n | value A = 0
  structure N = struct val w = 2 end
end
val a = let open M in value v end
val b = let open M.N in w + 1 end
local
  open M
in
  val c = value (B 5)
  val d = A
end
structure O = struct open M val extra = N.w end
val e = O.extra
val f = O.v
val _ = print (if (a,b,c,e) = (1,3,5,2) andalso M.value f = 1 then "open verified\n" else raise Fail "open")
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : M.d *)
(* CHECK-STDOUT-NEXT: val e : int *)
(* CHECK-STDOUT-NEXT: val f : M.d *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: open verified *)
