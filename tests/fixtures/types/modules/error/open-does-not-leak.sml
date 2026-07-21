structure S = struct structure T = struct val x = 1 end end
val y = let open S in x end
(* CHECK-ERR: × unbound variable 'x' *)
(* CHECK-ERR: :2:23] *)
