structure S : sig type t val x : t end = struct datatype t = A | B val x = A end
val y = S.A
(* CHECK-ERR: × unbound variable 'S.A' *)
(* CHECK-ERR: :2:9] *)
