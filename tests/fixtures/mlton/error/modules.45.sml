(* mlton regression/fail/modules.45.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      type 'a t
   end where type 'a t = int * 'b
(* CHECK-ERR: × unbound type variable 'b in type declaration *)
(* CHECK-ERR: :5:26] *)
