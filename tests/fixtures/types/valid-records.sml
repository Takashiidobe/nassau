val first = #1 (1, "a")
val name = #name {name = "x", size = 2}
val sizes = fn (r : {name : string, size : int}) => #size r
fun area ({width, height, ...} : {width : int, height : int, depth : int}) = width * height
fun both (p : int * string) = (#1 p, #2 p)
val selector = #2 : int * bool -> bool
val _ = print (if first = 1 andalso name = "x" andalso sizes {name="n",size=4} = 4 andalso area {width=3,height=5,depth=2} = 15 andalso both (7,"seven") = (7,"seven") andalso selector (1,true) then "records verified\n" else raise Fail "records")
(* CHECK-STDOUT: val first : int *)
(* CHECK-STDOUT-NEXT: val name : string *)
(* CHECK-STDOUT-NEXT: val sizes : {name:string, size:int} -> int *)
(* CHECK-STDOUT-NEXT: val area : {depth:int, height:int, width:int} -> int *)
(* CHECK-STDOUT-NEXT: val both : int * string -> int * string *)
(* CHECK-STDOUT-NEXT: val selector : int * bool -> bool *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: records verified *)
