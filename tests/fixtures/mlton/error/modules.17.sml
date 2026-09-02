(* mlton regression/fail/modules.17.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      eqtype t
      structure Z:
         sig
            datatype u = U
         end where type u = t
   end =
   struct
      structure Z =
         struct
            datatype u = U
         end
      datatype t = datatype Z.u
      structure Z =
         struct
            type u = Z.u
            datatype z = datatype Z.u
         end
   end
(* CHECK-ERR: × cannot refine u: it is not an abstract type of the signature *)
(* CHECK-ERR: :6:10] *)
