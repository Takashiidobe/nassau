(* mlton regression/fail/modules.36.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG =
   sig
      include sig type t end where type t = int
   end where type t = bool
(* CHECK-ERR: × cannot refine t: it is not an abstract type of the signature *)
(* CHECK-ERR: :3:4] *)
