(* val rec binds a recursive fn. *)
val rec count = fn 0 => "done"
                 | n => count (n - 1)
val (first, second) = (count 3, size "four")
val _ = print (first ^ " " ^ Int.toString second ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: done 4 *)
