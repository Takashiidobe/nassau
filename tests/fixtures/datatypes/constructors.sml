(* Several constructors with arguments carry a tag; nullary ones are
   immediates beside them. *)
datatype shape = Circle of int | Rect of int * int | Empty | Square of int | Point
fun area (Circle r) = 3 * r * r
  | area (Rect (w, h)) = w * h
  | area (Square s) = s * s
  | area Empty = 0
  | area Point = 0
fun describe Empty = "empty" | describe Point = "point" | describe s = "area " ^ Int.toString (area s)
fun map f [] = [] | map f (x :: xs) = f x :: map f xs
fun join [] = "" | join [x] = x | join (x :: xs) = x ^ ", " ^ join xs
val () = print (join (map describe [Circle 2, Rect (3, 4), Empty, Square 5, Point]) ^ "\n")
(* Constructors used as functions are closures. *)
val squares = map Square [1, 2, 3]
val () = print (join (map describe squares) ^ "\n")
val options = map SOME [1, 2]
val () = print (join (map (fn SOME n => Int.toString n | NONE => "none") (NONE :: options)) ^ "\n")
(* A datatype with one constructor needs no tag. *)
datatype wrapper = Wrap of string * int
fun unwrap (Wrap (s, n)) = s ^ Int.toString n
val () = print (unwrap (Wrap ("w", 1)) ^ "\n")
(* A datatype may shadow a built-in constructor. *)
datatype maybe = NONE | SOME of string | BOTH of string * string
fun show NONE = "nothing" | show (SOME s) = s | show (BOTH (a, b)) = a ^ b
val () = print (join (map show [SOME "x", NONE, BOTH ("y", "z")]) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: area 12, area 12, empty, area 25, point *)
(* CHECK-STDOUT-NEXT: area 1, area 4, area 9 *)
(* CHECK-STDOUT-NEXT: none, 1, 2 *)
(* CHECK-STDOUT-NEXT: w1 *)
(* CHECK-STDOUT-NEXT: x, nothing, yz *)
