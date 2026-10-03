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
val () = print (if a = 12 andalso c = "square" andalso d = 0 then "valid-datatype-specs\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-datatype-specs *)
