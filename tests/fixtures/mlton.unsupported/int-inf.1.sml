(* mlton regression/int-inf.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val big: IntInf.int = 0x80000000

fun try (barg: IntInf.int): unit =
   let
      val bstr = IntInf.toString barg
      val _ = print (concat ["trying ", bstr, "\n"])
   in print (if ~ big <= barg
                then if barg < big
                        then "ok\n"
                     else "positive\n"
             else "negative\n")
   end

val _ = try 0
val _ = try 1
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: trying 0 *)
(* CHECK-STDOUT-NEXT: ok *)
(* CHECK-STDOUT-NEXT: trying 1 *)
(* CHECK-STDOUT-NEXT: ok *)
