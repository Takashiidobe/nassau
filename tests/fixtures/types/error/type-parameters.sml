datatype 'a box = Box of 'a
val x : box = Box 1
(* CHECK-ERR: × type 'box' expects 1 type argument(s), found 0 *)
(* CHECK-ERR: :2:9] *)
