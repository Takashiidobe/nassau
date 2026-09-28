val r = ref 0
val x = while !r do r := 1
(* CHECK-ERR: × expected bool, found int *)
(* CHECK-ERR: :2:15] *)
