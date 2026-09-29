datatype color = Red | Green | Blue
datatype suit = Clubs | Diamonds | Hearts | Spades
fun name Red = "red" | name Green = "green" | name Blue = "blue"
fun next Red = Green | next Green = Blue | next Blue = Red
fun isRed Hearts = true | isRed Diamonds = true | isRed _ = false
val () = print (name (next (next Red)) ^ " " ^ name (next Blue) ^ "\n")
val () = print (if isRed Hearts andalso not (isRed Spades) then "ok\n" else "wrong\n")
fun count [] = 0 | count (Clubs :: rest) = 1 + count rest | count (_ :: rest) = count rest
val () = print (Int.toString (count [Clubs, Hearts, Clubs, Spades]) ^ "\n")
val () = print (case (Green, Spades) of (Green, Spades) => "both\n" | _ => "no\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: blue red *)
(* CHECK-STDOUT-NEXT: ok *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: both *)
