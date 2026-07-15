val same = (fn x => x) = (fn y => y)
(* CHECK-ERR: × type 'a -> 'a does not admit equality *)
(* CHECK-ERR: :1:13] *)
