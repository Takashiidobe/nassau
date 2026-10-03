signature SHAPES = sig
  datatype shape = Circle of int | Square of int * int | Empty
  val area : shape -> int
end
structure Shapes : SHAPES = struct
  datatype shape = Circle of int | Square of int * int | Empty
  fun area (Circle r) = 3 * r * r
    | area (Square (w, h)) = w * h
    | area Empty = 0
end
val a = Shapes.area (Shapes.Circle 2)
val b = Shapes.Square (1, 2)
fun describe Shapes.Empty = "empty"
  | describe (Shapes.Circle _) = "circle"
  | describe (Shapes.Square _) = "square"
val c = describe b
structure Sealed :> SHAPES = Shapes
val d = Sealed.area Sealed.Empty
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : Shapes.shape *)
(* CHECK-STDOUT-NEXT: val describe : Shapes.shape -> string *)
(* CHECK-STDOUT-NEXT: val c : string *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-RUN-EXIT: 0 *)
