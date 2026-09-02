(* mlton regression/exn-history.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML rejects this program *)
fun f x =
   if x = 0
      then raise Fail "ok"
   else f (x - 1) handle Overflow => 13

val _ = (f 10; ()) handle e => (List.app (fn s => print (concat [s, "\n"]))
                                (SMLofNJ.exnHistory e))
