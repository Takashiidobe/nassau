datatype t = A | B
val first = A
val inside = let datatype u = C | D in case D of C => 1 | D => 2 end
local
  datatype hidden = H | I
in
  datatype shown = S | T
  fun pick S = 1 | pick T = 2
  fun hid H = 1 | hid I = 2
  val size_of_hidden = hid I
end
val s = S
datatype u = B | C
val shadowed = B
val again = C
fun name B = "b" | name C = "c"
val kept = first
(* CHECK-STDOUT: val first : t *)
(* CHECK-STDOUT-NEXT: val inside : int *)
(* CHECK-STDOUT-NEXT: val pick : shown -> int *)
(* CHECK-STDOUT-NEXT: val hid : ?.hidden -> int *)
(* CHECK-STDOUT-NEXT: val size_of_hidden : int *)
(* CHECK-STDOUT-NEXT: val s : shown *)
(* CHECK-STDOUT-NEXT: val shadowed : u *)
(* CHECK-STDOUT-NEXT: val again : u *)
(* CHECK-STDOUT-NEXT: val name : u -> string *)
(* CHECK-STDOUT-NEXT: val kept : t *)
