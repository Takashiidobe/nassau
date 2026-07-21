val f = fn ref 0 => "zero" | ref "one" => "one"
(* CHECK-ERR: × expected int ref, found string ref *)
(* CHECK-ERR: :1:30] *)
