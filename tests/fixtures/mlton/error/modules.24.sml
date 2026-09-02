(* mlton regression/fail/modules.24.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype t = A | B
   end =
   struct
      datatype t = B | C
   end
(* CHECK-ERR: × datatype t does not match its specification: constructor A is missing *)
(* CHECK-ERR: :6:4] *)
