functor Name (X : sig datatype color = Red | Green val all : color list end) = struct
  fun name X.Red = "red" | name X.Green = "green"
  val names = map name X.all
end
structure Colors = struct datatype color = Red | Green val all = [Red, Green] end
structure N = Name (Colors)
val a = N.names
val b = N.name Colors.Red
(* CHECK-STDOUT: val a : string list *)
(* CHECK-STDOUT-NEXT: val b : string *)
