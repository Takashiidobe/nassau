val p = (1, "two", #"3");
val r = {name = "Ada", age = 36};
val n = #name r;
val t = #3 p;
val {age, ...} = r;
fun swap (a, b) = (b, a);
val s = swap (1, true);
val e = {};
(* CHECK-REPL: val p = (1,"two",#"3") : int * string * char *)
(* CHECK-REPL-NEXT: val r = {age=36,name="Ada"} : {age:int, name:string} *)
(* CHECK-REPL-NEXT: val n = "Ada" : string *)
(* CHECK-REPL-NEXT: val t = #"3" : char *)
(* CHECK-REPL-NEXT: val age = 36 : int *)
(* CHECK-REPL-NEXT: val swap = fn : 'a * 'b -> 'b * 'a *)
(* CHECK-REPL-NEXT: val s = (true,1) : bool * int *)
(* CHECK-REPL-NEXT: val e = () : unit *)
