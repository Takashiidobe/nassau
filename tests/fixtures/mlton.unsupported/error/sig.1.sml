(* mlton regression/fail/sig.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      datatype t = A
      and u = A
   end
