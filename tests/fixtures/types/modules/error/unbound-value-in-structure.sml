structure S = struct val x = 1 end
val y = S.z
(* CHECK-ERR: × unbound variable 'S.z' *)
(* CHECK-ERR: :2:9] *)
