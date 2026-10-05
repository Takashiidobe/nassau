(* SML'97 grammar, constant: string escape unicode (nassau-ugc.12). *)
val _ = print "[\u0041]\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: [A] *)
