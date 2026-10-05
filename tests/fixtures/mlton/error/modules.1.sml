(* mlton regression/fail/modules.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      datatype t = T
   end =
   struct
      datatype 'a t = T
   end
(* CHECK-ERR: × datatype t does not match its specification: the number of type parameters *)
(* CHECK-ERR: :6:4] *)
