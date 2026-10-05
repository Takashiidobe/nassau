(* mlton regression/fail/modules.4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type t
      type u
      type v = t * t
      sharing type u = v
   end
