val i = 1 + 2 * 3
val r = 1.5 * 2.0 - 0.5
val q = 7.0 / 2.0
val d = 9 div 2
val m = 9 mod 4
val s = "nassau"
val c = #"a"
val u = ()
val b = 1 < 2 andalso "a" < "b"
val either = false orelse 1.0 >= 2.0
val t = (1, "two", 3.0)
val nested = ((1, 2), (3, (4, 5)))
val rc = {name = "x", size = 3, ratio = 0.5}
val xs = [1, 2, 3]
val ys = [[1], [], [2, 3]]
val branch = if 1 = 1 then "yes" else "no"
val bound = let val a = 1 val b = a + 1 in (a, b) end
val seq = (1; 2; "last")
(* CHECK-STDOUT: val i : int *)
(* CHECK-STDOUT-NEXT: val r : real *)
(* CHECK-STDOUT-NEXT: val q : real *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-STDOUT-NEXT: val m : int *)
(* CHECK-STDOUT-NEXT: val s : string *)
(* CHECK-STDOUT-NEXT: val c : char *)
(* CHECK-STDOUT-NEXT: val u : unit *)
(* CHECK-STDOUT-NEXT: val b : bool *)
(* CHECK-STDOUT-NEXT: val either : bool *)
(* CHECK-STDOUT-NEXT: val t : int * string * real *)
(* CHECK-STDOUT-NEXT: val nested : (int * int) * (int * (int * int)) *)
(* CHECK-STDOUT-NEXT: val rc : {name:string, ratio:real, size:int} *)
(* CHECK-STDOUT-NEXT: val xs : int list *)
(* CHECK-STDOUT-NEXT: val ys : int list list *)
(* CHECK-STDOUT-NEXT: val branch : string *)
(* CHECK-STDOUT-NEXT: val bound : int * int *)
(* CHECK-STDOUT-NEXT: val seq : string *)
