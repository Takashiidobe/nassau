(* mlton regression/fail/sharing.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      structure T: sig type t = int end
      sharing T = T
   end
