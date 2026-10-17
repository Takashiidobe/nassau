signature A = sig type t end
signature S = sig structure N:sig include A A end end
(* CHECK-ERR: × duplicate type name 't' in signature *)
(* CHECK-ERR: :2:45] *)
