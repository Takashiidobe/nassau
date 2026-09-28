val r = ref 0
val x = (r := "one")
(* CHECK-ERR: × expected int ref * int, found int ref * string *)
(* CHECK-ERR: :2:10] *)
