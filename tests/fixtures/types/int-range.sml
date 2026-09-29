(* The extremes of a 31-bit int are valid constants. *)
val largest = 1073741823
val smallest = ~1073741824
(* CHECK-STDOUT: val largest : int *)
(* CHECK-STDOUT-NEXT: val smallest : int *)
