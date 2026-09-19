val () = print (if OS.Process.isSuccess OS.Process.success then "success\n" else "wrong\n")
val () = print (if OS.Process.isSuccess OS.Process.failure then "wrong\n" else "failure\n")
fun finish ok = (print "done\n"; OS.Process.exit (if ok then OS.Process.success else OS.Process.failure))
val _ = finish false
val () = print "unreachable\n"
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: success *)
(* CHECK-STDOUT-NEXT: failure *)
(* CHECK-STDOUT-NEXT: done *)
