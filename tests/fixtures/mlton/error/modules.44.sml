(* mlton regression/fail/modules.44.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      datatype t = T of 'a
   end
(* CHECK-ERR: × unbound type variable 'a in type declaration *)
(* CHECK-ERR: :4:25] *)
