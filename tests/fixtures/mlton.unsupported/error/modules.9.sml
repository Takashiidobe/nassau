(* mlton regression/fail/modules.9.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type t = int
      type u = int
      sharing type t = u
   end
