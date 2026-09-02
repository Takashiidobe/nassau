(* mlton regression/fail/modules.12.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
signature S =
   sig
      type 'a t
   end where type t = int
