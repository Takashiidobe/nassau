(* mlton regression/fail/modules.41.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
signature SIG =
   sig
      type t
      structure S: sig type u = t end where type u = t * t
   end
