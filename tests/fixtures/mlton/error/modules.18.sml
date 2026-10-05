(* mlton regression/fail/modules.18.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure T =
   struct
      structure X =
         struct
            type t = int
         end
   end
signature S =
   sig
      structure T: sig end
      val x: T.X.t
   end
(* CHECK-ERR: × unbound structure 'T.X' *)
(* CHECK-ERR: :12:14] *)
