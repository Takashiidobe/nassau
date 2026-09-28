val cell = ref []
val a = (cell := [1])
val b = (cell := ["one"])
(* CHECK-ERR: × expected ?.X1 list ref * ?.X1 list, found ?.X1 list ref * int list *)
(* CHECK-ERR: :2:10] *)
