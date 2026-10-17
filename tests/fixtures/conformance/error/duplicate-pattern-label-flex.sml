fun f {a=x, a=y, ...} = x
(* CHECK-ERR: × duplicate record label 'a' *)
(* CHECK-ERR: :1:23] *)
