(* mlton regression/flat-vector.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val v = Vector.tabulate (10,
   (fn x =>
      let
         fun toChar x = Char.chr (Char.ord #"0" + x)
      in
         (toChar 0, toChar x)
      end))
val x = Vector.sub (v, 8)
val _ = print "should be 08\n"
val _ = print (str (#1 x) ^ str (#2 x) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: should be 08 *)
(* CHECK-STDOUT-NEXT: 08 *)
