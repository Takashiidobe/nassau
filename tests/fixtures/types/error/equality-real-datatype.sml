datatype 'a box = Box of 'a
val x = Box 1.0 = Box 2.0
(* CHECK-ERR: × type real does not admit equality *)
(* CHECK-ERR: :2:9] *)
