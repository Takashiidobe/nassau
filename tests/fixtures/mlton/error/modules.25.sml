(* mlton regression/fail/modules.25.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      structure T:
         sig
            datatype t = A | B
         end
   end =
   struct
      structure T =
         struct
            datatype t = B | C
         end
   end
(* CHECK-ERR: × datatype t does not match its specification: constructor A is missing *)
(* CHECK-ERR: :9:4] *)
