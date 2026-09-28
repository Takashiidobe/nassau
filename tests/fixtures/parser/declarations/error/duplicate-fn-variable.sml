val fn_case = fn (a, a) => a
(* CHECK-ERR: × duplicate variable 'a' in pattern *)
(* CHECK-ERR: :1:22] *)
