val x = fn [] => 0 | x :: _ => x | [y] => y
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:36] *)
