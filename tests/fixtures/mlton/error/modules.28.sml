(* mlton regression/fail/modules.28.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype t = A | B of unit
   end =
   struct
      datatype t = A | B of int

      val rec B = fn () => A
   end
(* CHECK-ERR: × constructor 'B' requires an argument *)
(* CHECK-ERR: :9:15] *)
