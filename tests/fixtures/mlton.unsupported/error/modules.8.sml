(* mlton regression/fail/modules.8.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type t = int
      type u
      sharing type t = u
   end
