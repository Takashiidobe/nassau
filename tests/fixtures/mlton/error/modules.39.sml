(* mlton regression/fail/modules.39.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG =
   sig
      type u
      type v = u
   end where type v = int
structure S: SIG =
   struct
      type u = real
      type v = real
   end
(* CHECK-ERR: × cannot refine v: it is not an abstract type of the signature *)
(* CHECK-ERR: :3:4] *)
