(* mlton regression/fail/modules.14.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
type 'a u = unit
signature S =
   sig
      type 'a t
   end where type t = 'b u
(* CHECK-ERR: × cannot refine t: the number of type parameters differs *)
(* CHECK-ERR: :4:4] *)
