(* mlton regression/fail/modules.21.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S: sig val x: bool end =
   struct
      val x = 13
   end
(* CHECK-ERR: × value x does not match its specification: the structure has type int but *)
(* CHECK-ERR: :3:4] *)
