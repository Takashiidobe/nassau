(* mlton regression/wordn-array.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML rejects this program *)
(* Make sure that arrays of odd-size words work. *)

functor F (W: WORD) =
   struct
      val v = Vector.tabulate (10, W.fromInt)
      val () = print (concat [W.toString (Vector.sub (v, 9)), "\n"])
   end

structure S = F (Word5)
structure S = F (Word7)
structure S = F (Word9)
structure S = F (Word14)
structure S = F (Word17)
structure S = F (Word22)
structure S = F (Word30)
