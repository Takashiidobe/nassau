(* POLYML-SKIP: Poly/ML accepts an implicit final semicolon; batch grammar requires exp ;. *)
print "missing\n"
(* CHECK-ERR: × expected ; after a top-level expression *)
(* CHECK-ERR: :2:7] *)
