signature ORD = sig type t val le : t * t -> bool end
functor Sort (O : ORD) = struct fun min (a, b) = if O.le (a, b) then a else b end
functor Id (X : sig end) : sig end = X
functor Seal (X : ORD) :> ORD = X
functor Specs (type t val x : t) = struct val y = x end
functor Empty () = struct end
functor A (X : ORD) = struct end and B (Y : ORD) = struct end
(* CHECK-STDOUT: (signature (ORD (sig (type (t ())) (val (le (-> {{[(]}}* t t) bool)))))) *)
(* CHECK-STDOUT-NEXT: (functor (Sort (O ORD) (struct (fun (min (((tuple a b)) (if (app O.le (tuple a b)) a b))))))) *)
(* CHECK-STDOUT-NEXT: (functor (Id (X (sig)) (: X (sig)))) *)
(* CHECK-STDOUT-NEXT: (functor (Seal (X ORD) (:> X ORD))) *)
(* CHECK-STDOUT-NEXT: (functor (Specs (sig (type (t ())) (val (x t))) (struct (val y x)))) *)
(* CHECK-STDOUT-NEXT: (functor (Empty (sig) (struct))) *)
(* CHECK-STDOUT-NEXT: (functor (A (X ORD) (struct)) (B (Y ORD) (struct))) *)
