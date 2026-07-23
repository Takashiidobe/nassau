signature S = sig
  structure A : sig type t end
  structure B : sig type t type u end
  sharing A = B
end
signature T = sig
  structure C : sig structure D : sig type v end end
  structure E : sig structure D : sig type v end end
  sharing C.D = E.D
end
(* CHECK-STDOUT: (signature (S (sig (structure (A (sig (type (t ()))))) (structure (B (sig (type (t ())) (type (u ()))))) (sharing-structures A B)))) *)
(* CHECK-STDOUT-NEXT: (signature (T (sig (structure (C (sig (structure (D (sig (type (v ())))))))) (structure (E (sig (structure (D (sig (type (v ())))))))) (sharing-structures C.D E.D)))) *)
