signature A = sig val x:int end
signature S = sig include A A end
(* CHECK-ERR: × duplicate value name 'x' in signature *)
(* CHECK-ERR: :2:29] *)
