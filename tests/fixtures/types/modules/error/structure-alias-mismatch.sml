structure A = struct val x = 1 end
structure B : sig val x : string end = A
(* CHECK-ERR: × value x does not match its specification: the structure has type int but *)
(* CHECK-ERR: :2:40] *)
