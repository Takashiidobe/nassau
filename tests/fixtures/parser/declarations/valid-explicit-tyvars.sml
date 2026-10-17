val 'a id = fn (x : 'a) => x
fun ('a, 'b) first (x : 'a, y : 'b) = x
val _ = print (id (first ("scoped\n", true)))
(* CHECK-STDOUT: (tyvars ('a) (val id (fn ((: x 'a) x)))) *)
(* CHECK-STDOUT-NEXT: (tyvars ('a 'b) (fun (first (((tuple (: x 'a) (: y 'b))) x)))) *)
(* CHECK-STDOUT-NEXT: (val _ (app print (app id (app first (tuple "scoped\n" true))))) *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: scoped *)
