(* Word arithmetic wraps, and comparisons are unsigned. *)
val sum = 0w3 + 0w4
val difference = 0w10 - 0w4
val product = 0w6 * 0w7
val quotient = 0w45 div 0w6
val remainder = 0w45 mod 0w6
val wrapped = 0w0 - 0w1
val checks = [sum = 0w7, difference = 0w6, product = 0w42, quotient = 0w7, remainder = 0w3,
              wrapped > 0w100, 0w2 < 0w3, 0w3 <= 0w3, 0w4 >= 0w5 = false]
val count = length (List.filter (fn b => b) checks)
val divided = (0w1 div 0w0; 0) handle Div => 1
val _ = Posix.Process.exit (Word8.fromInt (count * 10 + divided))
(* CHECK-EXIT: 91 *)
