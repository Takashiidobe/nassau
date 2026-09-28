datatype t = A | B
fun first A = 1 | first B = 2
fun unbound Foo = 1
fun lowercase a = a
fun after_a A = 1 | after_a other = 2
(* CHECK-STDOUT: (datatype (t () (A) (B))) *)
(* CHECK-STDOUT-NEXT: (fun (first ((A) 1) ((B) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (unbound ((Foo) 1))) *)
(* CHECK-STDOUT-NEXT: (fun (lowercase ((a) a))) *)
(* CHECK-STDOUT-NEXT: (fun (after_a ((A) 1) ((other) 2))) *)
