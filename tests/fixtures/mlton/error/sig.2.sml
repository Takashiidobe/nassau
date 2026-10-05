(* mlton regression/fail/sig.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S = sig type t end
signature S1 = S where type t = int
signature S2 = S where type t = real

structure S1: S1 =
   struct
      type t = int
   end

structure S2: S2 = S1
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :11:20] *)
