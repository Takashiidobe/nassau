(* mlton regression/int-inf.4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun dump (x: IntInf.int): unit =
       let val rest = IntInf.quot (x, 10)
       in (print o Int.toString o IntInf.toInt o IntInf.rem) (x, 10);
          if rest = 0
             then print "\n"
             else dump rest
       end
                  
val _ = dump 12345678901234567890
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 09876543210987654321 *)
