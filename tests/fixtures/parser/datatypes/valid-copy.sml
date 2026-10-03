datatype color = Red | Green
datatype hue = datatype color
fun name Red = "red" | name Green = "green"
val h = Red
(* CHECK-STDOUT: (datatype (color () (Red) (Green))) *)
(* CHECK-STDOUT-NEXT: (datatype-copy hue color) *)
(* CHECK-STDOUT-NEXT: (fun (name ((Red) "red") ((Green) "green"))) *)
(* CHECK-STDOUT-NEXT: (val h Red) *)
(* CHECK-RUN-EXIT: 0 *)
