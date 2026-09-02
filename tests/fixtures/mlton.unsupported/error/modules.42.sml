(* mlton regression/fail/modules.42.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S = struct end
and S = struct end

functor F () = struct end
and F () = struct end

signature S = sig end
and S = sig end
