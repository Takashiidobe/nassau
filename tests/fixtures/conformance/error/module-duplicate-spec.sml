(* nassau-ugc.23 *)
signature S = sig val x:int val x:int end
(* CHECK-ERR: × duplicate value name 'x' in signature *)
(* CHECK-ERR: :2:29] *)
