structure S = struct val x = 1 end
fun f S.x = 1
(* CHECK-ERR: × unbound constructor 'S.x' *)
(* CHECK-ERR: :2:7] *)
