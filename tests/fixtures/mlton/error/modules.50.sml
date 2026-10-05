(* mlton regression/fail/modules.50.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG =
   sig
      type t
   end

structure S:
   sig
      structure S1: SIG
   end where type S1.t = int =
   struct
      structure S1: SIG =
         struct
            type t = real
         end
   end
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :11:4] *)
