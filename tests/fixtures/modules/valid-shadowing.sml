structure S = struct val x = 1 val x = "shadowed" end
val a = S.x
structure T = struct val x = 1 end
structure T = struct val x = true end
val b = T.x
val x = 10
structure U = struct val y = x + 1 val x = "s" end
val c = (U.y, U.x)
val () = print (if a = "shadowed" andalso b andalso c = (11, "s") then "valid-shadowing\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-shadowing *)
