val r = ref 1
val x = r = 1
(* CHECK-ERR: × expected int ref, found int *)
(* CHECK-ERR: :2:13] *)
