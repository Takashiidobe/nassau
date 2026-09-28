structure S :> sig datatype d = A | B end = struct datatype d = A | B end
fun f S.A = 1 | f S.B = 2
fun g S.A = 1
structure T : sig type d end = struct datatype d = A | B end
val x = 1
(* CHECK-STDOUT: (structure (S (:> (struct (datatype (d () (A) (B)))) (sig (datatype (d () (A) (B))))))) *)
(* CHECK-STDOUT-NEXT: (fun (f ((S.A) 1) ((S.B) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (g ((S.A) 1))) *)
(* CHECK-STDOUT-NEXT: (structure (T (: (struct (datatype (d () (A) (B)))) (sig (type (d ())))))) *)
(* CHECK-STDOUT-NEXT: (val x 1) *)
(* CHECK-STDERR: warning: match nonexhaustive at 3:7 *)
