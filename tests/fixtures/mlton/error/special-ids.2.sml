(* mlton regression/fail/special-ids.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG1 =
   sig
      datatype t = = of unit
   end
signature SIG2 =
   sig
      exception =
   end
signature SIG3 =
   sig
      val = : unit
   end

local
datatype t = = of unit
in end
local
exception =
in end

local
val op= = ()
in end

local
val rec f = fn op= => fn () => ()
val rec op= = fn () => ()
in end

local
fun f op= () = ()
fun op= () = ()
in end
(* CHECK-ERR: × expected a constructor name *)
(* CHECK-ERR: :4:20] *)
