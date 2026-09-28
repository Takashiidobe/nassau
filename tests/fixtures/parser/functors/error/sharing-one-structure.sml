signature S = sig structure A : sig end sharing A end
(* CHECK-ERR: × expected = and another name *)
(* CHECK-ERR: :1:51] *)
