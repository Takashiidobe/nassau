(* mlton regression/fail/modules.6.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type ('a, 'b) t
      type ('a, 'b) u
      type ('a, 'b) v = ('b, 'a) t
      sharing type u = v
   end
