(* A val binding binds every variable of its pattern. *)
val (a, b) = (1, 2)
val x :: rest = [3, 4, 5]
val {name, age = years} = {name = "Ada", age = 36}
val (p as (q, _), SOME r) = ((6, 7), SOME 8)
val [c, d] = [9, 10]
val ref e = ref 11
val () = print (Int.toString (a + b + x) ^ " " ^ name ^ " " ^ Int.toString years ^ "\n")
val () = print (Int.toString (#2 p + q + r + c + d + e) ^ "\n")
fun total xs = let val first :: others = xs; val SOME n = SOME (length others) in first + n end
and length [] = 0 | length (_ :: t) = 1 + length t
val () = print (Int.toString (total [5, 1, 1]) ^ "\n")
val (s, t) = (a, b) and (u, v) = (b, a)
val () = print (Int.toString (s * 1000 + t * 100 + u * 10 + v) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 Ada 36 *)
(* CHECK-STDOUT-NEXT: 51 *)
(* CHECK-STDOUT-NEXT: 7 *)
(* CHECK-STDOUT-NEXT: 1221 *)
