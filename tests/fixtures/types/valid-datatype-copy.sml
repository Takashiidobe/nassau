datatype color = Red | Green
datatype hue = datatype color
val h = Red
fun name Red = "red" | name Green = "green"
val same_type = [h, Green]
(* CHECK-STDOUT: val h : color *)
(* CHECK-STDOUT-NEXT: val name : color -> string *)
(* CHECK-STDOUT-NEXT: val same_type : color list *)
(* CHECK-RUN-EXIT: 0 *)
