(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* POLYML-SKIP: deliberate exhaustion of Nassau's configured 8 MiB heap *)
fun fill 0 values = ()
  | fill n values = fill (n - 1) (ref n :: values)
val () = fill 1000000 []
(* CHECK-EXIT: 1 *)
(* CHECK-STDERR: nassau: out of memory *)
