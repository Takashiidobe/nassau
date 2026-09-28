fun f (SOME) = 0 | f NONE = 1
(* CHECK-ERR: × constructor 'SOME' requires an argument *)
(* CHECK-ERR: :1:8] *)
