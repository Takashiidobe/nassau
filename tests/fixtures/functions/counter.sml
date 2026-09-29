(* A counter captures a ref cell; each call updates it. *)
fun makeCounter start =
  (fn cell => fn () => (cell := !cell + 1; !cell)) (ref start)
val next = makeCounter 10
val other = makeCounter 0
val a = next ()
val b = next ()
val c = other ()
val _ = print (Int.toString a ^ " " ^ Int.toString b ^ " " ^ Int.toString c ^ "\n")
val r = ref 0
val _ = while !r < 5 do r := !r + 1
val _ = print (Int.toString (!r) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 11 12 1 *)
(* CHECK-STDOUT-NEXT: 5 *)
