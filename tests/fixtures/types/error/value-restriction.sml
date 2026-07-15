val cell = ref []
val _ = cell := [1]
(* CHECK-ERR: × expected ?.X1 list ref * ?.X1 list, found ?.X1 list ref * int list *)
(* CHECK-ERR: :2:9] *)
