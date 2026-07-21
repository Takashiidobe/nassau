structure S : sig type t end = struct datatype t = A | B end
fun f S.A = 1 | f S.B = 2
(* CHECK-ERR: × unbound constructor 'S.A' *)
(* CHECK-ERR: :2:7] *)
