val _ = print "[\u0041\u0062]\n"
val _ = print (if #"\u00fF" = #"\255" then "byte\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: [Ab] *)
(* CHECK-STDOUT: byte *)
