signature S = sig type t val x : t end
structure A : S = struct type t = int val x = 1 end
structure B :> S = struct type t = int val x = 1 end
structure C : sig val y : int end = struct val y = 2 val hidden = 3 end
structure D = A : S
structure E = struct type t = int val x = 2 end :> S
structure F :> S where type t = int = struct type t = int val x = 3 end
(* CHECK-STDOUT: (signature (S (sig (type (t ())) (val (x t))))) *)
(* CHECK-STDOUT-NEXT: (structure (A (: (struct (type (t () int)) (val x 1)) S))) *)
(* CHECK-STDOUT-NEXT: (structure (B (:> (struct (type (t () int)) (val x 1)) S))) *)
(* CHECK-STDOUT-NEXT: (structure (C (: (struct (val y 2) (val hidden 3)) (sig (val (y int)))))) *)
(* CHECK-STDOUT-NEXT: (structure (D (: A S))) *)
(* CHECK-STDOUT-NEXT: (structure (E (:> (struct (type (t () int)) (val x 2)) S))) *)
(* CHECK-STDOUT-NEXT: (structure (F (:> (struct (type (t () int)) (val x 3)) (where S (type () t int))))) *)
