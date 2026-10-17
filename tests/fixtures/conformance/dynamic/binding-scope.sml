val x = 7
val x = 8 and y = x
val _ = print (Int.toString x ^ ":" ^ Int.toString y ^ "\n")
val captured = let val x = 10 in fn y => x + y end
val x = 99
val _ = print (Int.toString (captured 2) ^ "\n")
local val hidden = 3 in val public = hidden + 1 end
val _ = print (Int.toString public ^ "\n")
fun even 0 = true | even n = odd (n - 1)
and odd 0 = false | odd n = even (n - 1)
val _ = print (Bool.toString (even 12) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 8:7 *)
(* CHECK-STDOUT-NEXT: 12 *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: true *)
