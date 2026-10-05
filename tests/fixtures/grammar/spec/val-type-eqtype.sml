(* SML'97 grammar, specification: val type eqtype. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t eqtype e type ('a, 'b) p val v : t -> int and w : e val q : (int, int) p end
structure A : S = struct type t = int type e = int type ('a, 'b) p = 'a * 'b fun v x = x val w = 2 val q = (1, 2) end
val _ = pi (A.v 1) val _ = print (if A.w = A.w then "eq\n" else "")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
(* CHECK-STDOUT-NEXT: eq *)
