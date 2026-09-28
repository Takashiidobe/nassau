val r = ref 0
val x = if true then (r := 1) else 2
(* CHECK-ERR: × expected unit, found int *)
(* CHECK-ERR: :2:36] *)
