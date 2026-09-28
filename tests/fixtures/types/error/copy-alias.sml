type t = int
datatype u = datatype t
(* CHECK-ERR: × unbound type 't' *)
(* CHECK-ERR: :2:1] *)
