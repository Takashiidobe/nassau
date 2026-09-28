structure S : sig val x : int end = struct val x = 1 val hidden = 2 end
val y = S.hidden
(* CHECK-ERR: × unbound variable 'S.hidden' *)
(* CHECK-ERR: :2:9] *)
