val x = "\u00xz"
(* CHECK-ERR: × unsupported string escape *)
(* CHECK-ERR: :1:9] *)
