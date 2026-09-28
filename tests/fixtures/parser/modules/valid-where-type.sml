signature S = sig type t type u val x : t * u end
signature T = S where type t = int
signature U = S where type t = int and type u = string
signature V = S where type t = int where type u = bool
structure A :> S where type u = int = struct type t = bool type u = int val x = (true, 1) end
signature W = sig type (*a*) 'a t end where type 'a t = 'a list
(* CHECK-STDOUT: (signature (S (sig (type (t ())) (type (u ())) (val (x {{[(]}}* t u)))))) *)
(* CHECK-STDOUT-NEXT: (signature (T (where S (type () t int)))) *)
(* CHECK-STDOUT-NEXT: (signature (U (where S (type () t int) (type () u string)))) *)
(* CHECK-STDOUT-NEXT: (signature (V (where (where S (type () t int)) (type () u bool)))) *)
(* CHECK-STDOUT-NEXT: (structure (A (:> (struct (type (t () bool)) (type (u () int)) (val x (tuple true 1))) (where S (type () u int))))) *)
(* CHECK-STDOUT-NEXT: (signature (W (where (sig (type (t ('a)))) (type ('a) t (tycon list 'a))))) *)
