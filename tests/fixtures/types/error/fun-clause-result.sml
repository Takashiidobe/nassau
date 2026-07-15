fun f 0 = "zero"
  | f n = n
(* CHECK-ERR: × expected int -> string, found int -> int *)
(* CHECK-ERR: :2:11] *)
