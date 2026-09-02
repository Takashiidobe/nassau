(* mlton regression/fail/modules.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype 'a t = T of int
   end =
   struct
      datatype 'a t = T of 'a
   end
(* CHECK-ERR: × constructor T does not match its specification: the structure has type 'a *)
(* CHECK-ERR: :6:4] *)
