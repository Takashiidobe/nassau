signature S = sig exception E val E:exn end
(* CHECK-ERR: × duplicate value name 'E' in signature *)
(* CHECK-ERR: :1:31] *)
