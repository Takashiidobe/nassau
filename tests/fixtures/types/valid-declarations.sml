val a = 1 and b = "two" and c = 3.0
val (x, y) = (1, 2)
val {left = l, right = r} = {left = "l", right = 2}
val first :: rest = [1, 2, 3]
val whole as (p, q) = (1, "a")
val [only] = [true]
val _ = 5
infix 5 ++
fun a' ++ b' = a' + b'
val summed = 1 ++ 2
local
  val hidden = 10
in
  val exposed = hidden + 1
  fun bump n = n + hidden
end
val shadow = 1
val shadow = "now a string"
type pair = int * int
val alias_total = let val (a, b) : pair = (1, 2) in a + b end
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : string *)
(* CHECK-STDOUT-NEXT: val c : real *)
(* CHECK-STDOUT-NEXT: val x : int *)
(* CHECK-STDOUT-NEXT: val y : int *)
(* CHECK-STDOUT-NEXT: val l : string *)
(* CHECK-STDOUT-NEXT: val r : int *)
(* CHECK-STDOUT-NEXT: val first : int *)
(* CHECK-STDOUT-NEXT: val rest : int list *)
(* CHECK-STDOUT-NEXT: val whole : int * string *)
(* CHECK-STDOUT-NEXT: val p : int *)
(* CHECK-STDOUT-NEXT: val q : string *)
(* CHECK-STDOUT-NEXT: val only : bool *)
(* CHECK-STDOUT-NEXT: val ++ : int * int -> int *)
(* CHECK-STDOUT-NEXT: val summed : int *)
(* CHECK-STDOUT-NEXT: val exposed : int *)
(* CHECK-STDOUT-NEXT: val bump : int -> int *)
(* CHECK-STDOUT-NEXT: val shadow : int *)
(* CHECK-STDOUT-NEXT: val shadow : string *)
(* CHECK-STDOUT-NEXT: val alias_total : int *)
