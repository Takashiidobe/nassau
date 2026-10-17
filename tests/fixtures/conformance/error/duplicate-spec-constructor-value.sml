signature S = sig datatype t=C val C:t end
(* CHECK-ERR: × duplicate value name 'C' in signature *)
(* CHECK-ERR: :1:32] *)
