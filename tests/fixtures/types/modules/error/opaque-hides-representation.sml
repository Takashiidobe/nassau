structure Stack :> sig type t val empty : t end = struct type t = int list val empty = [] end
val x = 1 :: Stack.empty
(* CHECK-ERR: × expected int * int list, found int * Stack.t *)
(* CHECK-ERR: :2:9] *)
