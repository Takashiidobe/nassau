(* mlton regression/fail/modules.48.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      datatype t = T
   end

structure S1: S =
   struct
      datatype t = T
    end

structure S2: S where type t = int = S1
(* CHECK-ERR: × cannot refine t: it is not an abstract type of the signature *)
(* CHECK-ERR: :12:15] *)
