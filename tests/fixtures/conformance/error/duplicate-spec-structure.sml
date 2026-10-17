signature S = sig structure A:sig end structure A:sig end end
(* CHECK-ERR: × duplicate structure name 'A' in signature *)
(* CHECK-ERR: :1:39] *)
