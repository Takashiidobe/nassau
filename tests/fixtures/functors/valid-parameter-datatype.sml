functor Name (X : sig datatype color = Red | Green val all : color list end) = struct
  fun name X.Red = "red" | name X.Green = "green"
  val names = map name X.all
end
structure Colors = struct datatype color = Red | Green val all = [Red, Green] end
structure N = Name (Colors)
val a = N.names
val b = N.name Colors.Red
val () = print (if a = ["red", "green"] andalso b = "red" then "valid-parameter-datatype\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-parameter-datatype *)
