(* mlton regression/fail/modules.37.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG =
   sig
      structure S: sig type t end where type t = int
   end where type S.t = bool
(* CHECK-ERR: × cannot refine S.t: it is not an abstract type of the signature *)
(* CHECK-ERR: :3:4] *)
