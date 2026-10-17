val 'a bad = ref ([] : 'a list)
(* CHECK-ERR: × explicit type variable cannot be generalized *)
(* CHECK-ERR: :1:1] *)
