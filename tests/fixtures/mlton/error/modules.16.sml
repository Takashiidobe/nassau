(* mlton regression/fail/modules.16.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      eqtype t
      structure Z:
         sig
            datatype u = U
         end
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
(* CHECK-ERR: × the structure does not provide datatype u, which the signature specifies *)
(* CHECK-ERR: :10:4] *)
