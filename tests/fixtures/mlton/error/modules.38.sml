(* mlton regression/fail/modules.38.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      type t
   end where type t = int * int
   =
   struct
      type t = int
   end
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :7:4] *)
