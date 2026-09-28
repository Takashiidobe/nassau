infix 5 ++
datatype t = ++ of int * int
fun f x = case x of ++ (a, b) => a
(* CHECK-ERR: × expected a pattern *)
(* CHECK-ERR: :3:21] *)
