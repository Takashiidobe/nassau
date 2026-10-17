val cell = ref NONE
fun 'a bad (x : 'a) = cell := SOME x
(* CHECK-ERR: × expected ?.X1 option ref * ?.X1 option, found ?.X1 option ref * 'a option *)
(* CHECK-ERR: :2:23] *)
