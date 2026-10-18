structure S = struct datatype t=C end
val S.C as x = S.C
(* CHECK-ERR: × expected = after val pattern *)
(* CHECK-ERR: :2:9] *)
