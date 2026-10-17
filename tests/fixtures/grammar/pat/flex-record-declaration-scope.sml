type pair = {a:int, b:bool}
fun f ({a, ...} : pair) = a
val _ = print (Int.toString (f {a=1, b=true}) ^ "\n")
val _ = print (Int.toString (let fun g (r as {a, ...}) = a + #a (r : pair)
                               in g {a=3, b=true} end) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
(* CHECK-STDOUT-NEXT: 6 *)
