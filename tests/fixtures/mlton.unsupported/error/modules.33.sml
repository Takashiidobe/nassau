(* mlton regression/fail/modules.33.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type t = int
   end
signature S =
   sig
      structure S1: S
      structure S2: S
      sharing type S1.t = S2.t
   end
