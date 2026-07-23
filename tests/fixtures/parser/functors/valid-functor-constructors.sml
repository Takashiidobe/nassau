functor Make (X : sig datatype d = A | B end) = struct
  datatype e = C | D
  fun f X.A = C | f X.B = D
end
structure M = Make (struct datatype d = A | B end)
fun g M.C = 1 | g M.D = 2
fun h M.C = 1
(* CHECK-STDOUT: (functor (Make (X (sig (datatype (d () (A) (B))))) (struct (datatype (e () (C) (D))) (fun (f ((X.A) C) ((X.B) D)))))) *)
(* CHECK-STDOUT-NEXT: (structure (M (app Make (struct (datatype (d () (A) (B))))))) *)
(* CHECK-STDOUT-NEXT: (fun (g ((M.C) 1) ((M.D) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (h ((M.C) 1))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 7:7 *)
