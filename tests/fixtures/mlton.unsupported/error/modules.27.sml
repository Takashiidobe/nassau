(* mlton regression/fail/modules.27.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype t = A | B
   end =
   struct
      datatype t = A | B
      datatype u = B
   end
