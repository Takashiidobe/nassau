(* mlton regression/fail/modules.11.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype t = T
   end =
   struct
      type t = int
   end
(* CHECK-ERR: × the structure does not provide datatype t, which the signature specifies *)
(* CHECK-ERR: :6:4] *)
