val first = #1 (1, "a")
val name = #name {name = "x", size = 2}
val sizes = fn (r : {name : string, size : int}) => #size r
fun area ({width, height, ...} : {width : int, height : int, depth : int}) = width * height
fun both (p : int * string) = (#1 p, #2 p)
val selector = #2 : int * bool -> bool
(* CHECK-STDOUT: val first : int *)
(* CHECK-STDOUT-NEXT: val name : string *)
(* CHECK-STDOUT-NEXT: val sizes : {name:string, size:int} -> int *)
(* CHECK-STDOUT-NEXT: val area : {depth:int, height:int, width:int} -> int *)
(* CHECK-STDOUT-NEXT: val both : int * string -> int * string *)
(* CHECK-STDOUT-NEXT: val selector : int * bool -> bool *)
