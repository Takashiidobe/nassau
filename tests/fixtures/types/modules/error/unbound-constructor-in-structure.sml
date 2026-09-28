structure S = struct datatype d = A end
fun f S.B = 1
(* CHECK-ERR: × unbound constructor 'S.B' *)
(* CHECK-ERR: :2:7] *)
