structure S = struct type t = int end
val x : S.u = 1
(* CHECK-ERR: × unbound type 'S.u' *)
(* CHECK-ERR: :2:9] *)
