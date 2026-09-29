val x = (fn a => a) = (fn b => b)
(* CHECK-ERR: × type 'a -> 'a does not admit equality *)
(* CHECK-ERR: :1:9] *)
