(* mlton regression/fail/modules.7.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type 'a t
      type u
      sharing type t = u
   end
(* CHECK-ERR: × cannot refine u: the types take different numbers of parameters *)
(* CHECK-ERR: :6:7] *)
