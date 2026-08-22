datatype shape = Circle of real | Rect of real * real | Point
fun area (Circle r) = 3.14 * r * r
  | area (Rect (w, h)) = w * h
  | area Point = 0.0
datatype named = Named of {name : string, size : int}
fun label (Named {name = n, size = _}) = n
datatype fn_box = Box of int -> int
fun run (Box f) = f 1
datatype unit_like = Only
val shapes = [Circle 1.0, Rect (2.0, 3.0), Point]
val total = area Point + area (Rect (1.0, 2.0))
val boxed = Box (fn x => x + 1)
val only = Only
(* CHECK-STDOUT: val area : shape -> real *)
(* CHECK-STDOUT-NEXT: val label : named -> string *)
(* CHECK-STDOUT-NEXT: val run : fn_box -> int *)
(* CHECK-STDOUT-NEXT: val shapes : shape list *)
(* CHECK-STDOUT-NEXT: val total : real *)
(* CHECK-STDOUT-NEXT: val boxed : fn_box *)
(* CHECK-STDOUT-NEXT: val only : unit_like *)
(* CHECK-RUN-EXIT: 0 *)
