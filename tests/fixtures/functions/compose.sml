(* o composes closures; functions can be built from functions. *)
fun compose (f, g) = fn x => f (g x)
val inc = fn x => x + 1
val double = fn x => x * 2
val incThenDouble = compose (double, inc)
val doubleThenInc = double o inc o inc
val _ = print (Int.toString (incThenDouble 5) ^ " " ^ Int.toString (doubleThenInc 5) ^ "\n")
val shout = (fn s => s ^ "!") o Int.toString
val _ = print (shout 42 ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 12 14 *)
(* CHECK-STDOUT-NEXT: 42! *)
