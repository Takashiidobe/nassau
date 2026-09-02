(* mlton regression/fail/datatype-where-complex.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      datatype t = T
   end where type t = int * int
(* CHECK-ERR: × cannot refine t: it is not an abstract type of the signature *)
(* CHECK-ERR: :3:4] *)
