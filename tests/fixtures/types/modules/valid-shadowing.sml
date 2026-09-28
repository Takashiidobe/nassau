structure S = struct val x = 1 val x = "shadowed" end
val a = S.x
structure T = struct val x = 1 end
structure T = struct val x = true end
val b = T.x
val x = 10
structure U = struct val y = x + 1 val x = "s" end
val c = (U.y, U.x)
(* CHECK-STDOUT: val a : string *)
(* CHECK-STDOUT-NEXT: val b : bool *)
(* CHECK-STDOUT-NEXT: val x : int *)
(* CHECK-STDOUT-NEXT: val c : int * string *)
