fun f {a, ...} = a
(* CHECK-ERR: × unresolved flex record (can't tell what fields there are besides #a) *)
(* CHECK-ERR: :1:1] *)
