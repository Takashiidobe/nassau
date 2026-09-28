val x = fn 1.5 => 0 | _ => 1
(* CHECK-ERR: × expected a pattern; real constants cannot be patterns *)
(* CHECK-ERR: :1:12] *)
