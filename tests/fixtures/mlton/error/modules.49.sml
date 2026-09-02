(* mlton regression/fail/modules.49.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG =
   sig
      type t
   end

functor F (structure S1: SIG
           structure S2: SIG
           sharing S1 = S2) =
   struct
   end

structure S1: SIG = struct type t = int end
structure S2: SIG = struct type t = real end
structure Z = F (structure S1 = S1
                 structure S2 = S2)
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :15:18] *)
