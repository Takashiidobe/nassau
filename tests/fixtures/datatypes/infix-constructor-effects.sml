val state = ref 0
fun tick n = (state := !state * 10 + n; n)
datatype t = Empty | Pair of int * int | Other of int
infix Pair
fun make () = tick 1 Pair tick 2
val _ = print (Int.toString (case make () of x Pair y => x + y | _ => 0) ^ "\n")
val _ = print (Int.toString (!state) ^ "\n")
exception Both of int * int
infix Both
val _ = print (Int.toString ((raise (3 Both 4)) handle x Both y => x + y) ^ "\n")
local datatype t = Joined of int * int
in val join = op Joined end
val _ = join (5, 6)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 12 *)
(* CHECK-STDOUT-NEXT: 7 *)
