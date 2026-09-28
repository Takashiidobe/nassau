structure M = struct
  datatype d = A | B of int
  val v = B 1
  structure N = struct val w = 2 end
end
open M
val a = v
val b = N.w
val c = M.N.w
val z = let open M.N in w + 1 end
open M M.N
fun f M.A = 0 | f (M.B n) = n
type t = M.d
val g : M.d -> int = f
(* CHECK-STDOUT: (structure (M (struct (datatype (d () (A) (B int))) (val v (app B 1)) (structure (N (struct (val w 2))))))) *)
(* CHECK-STDOUT-NEXT: (open M) *)
(* CHECK-STDOUT-NEXT: (val a v) *)
(* CHECK-STDOUT-NEXT: (val b N.w) *)
(* CHECK-STDOUT-NEXT: (val c M.N.w) *)
(* CHECK-STDOUT-NEXT: (val z (let ((open M.N)) (+ w 1))) *)
(* CHECK-STDOUT-NEXT: (open M M.N) *)
(* CHECK-STDOUT-NEXT: (fun (f ((M.A) 0) (((con M.B n)) n))) *)
(* CHECK-STDOUT-NEXT: (type (t () M.d)) *)
(* CHECK-STDOUT-NEXT: (val (: g (-> M.d int)) f) *)
