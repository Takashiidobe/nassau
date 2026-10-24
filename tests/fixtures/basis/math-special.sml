val inf = 1.0 / 0.0
fun show x =
  print ((if Real.isNan x then "nan"
          else if Real.== (x, inf) then "inf"
          else if Real.== (x, ~inf) then "~inf"
          else Int.toString (Real.floor (x * 1000.0))) ^ "\n")
val nan = Math.sqrt ~1.0
val _ = show (Math.sqrt 0.0)
val _ = show nan
val _ = show (Math.ln 0.0)
val _ = show (Math.ln ~1.0)
val _ = show (Math.exp inf)
val _ = show (Math.exp (~inf))
val _ = show (Math.pow (0.0, 0.0))
val _ = show (Math.pow (~8.0, 1.0 / 3.0))
val _ = show (Math.asin 2.0)
val _ = show (Math.atan inf)
val _ = show (Math.atan2 (0.0, ~1.0))
val _ = show (Math.tanh inf)
val _ = show (Math.cosh (~inf))
val _ = show (Math.sin inf)
val _ = show (Math.log10 0.0)
(* CHECK-STDOUT: 0 *)
(* CHECK-STDOUT-NEXT: nan *)
(* CHECK-STDOUT-NEXT: ~inf *)
(* CHECK-STDOUT-NEXT: nan *)
(* CHECK-STDOUT-NEXT: inf *)
(* CHECK-STDOUT-NEXT: 0 *)
(* CHECK-STDOUT-NEXT: 1000 *)
(* CHECK-STDOUT-NEXT: nan *)
(* CHECK-STDOUT-NEXT: nan *)
(* CHECK-STDOUT-NEXT: 1570 *)
(* CHECK-STDOUT-NEXT: 3141 *)
(* CHECK-STDOUT-NEXT: 1000 *)
(* CHECK-STDOUT-NEXT: inf *)
(* CHECK-STDOUT-NEXT: nan *)
(* CHECK-STDOUT-NEXT: ~inf *)
(* CHECK-EXIT: 0 *)
