(* mlton regression/fail/modules.15.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* u admits equality, but t does not, hence cannot substitute t for u. *)
signature S =
   sig
      type t
      structure Z:
         sig
            datatype u = U
         end where type u = t
   end
(* CHECK-ERR: × cannot refine u: it is not an abstract type of the signature *)
(* CHECK-ERR: :7:10] *)
