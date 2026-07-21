val a = let structure S = struct val x = 1 end in S.x end
val b = S.x
(* CHECK-ERR: × unbound structure 'S' *)
(* CHECK-ERR: :2:9] *)
