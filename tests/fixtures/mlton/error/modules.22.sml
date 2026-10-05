(* mlton regression/fail/modules.22.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      val x: 'a list
   end =
   struct
      val x: unit list = []
   end
(* CHECK-ERR: × value x does not match its specification: the structure has type unit list *)
(* CHECK-ERR: :6:4] *)
