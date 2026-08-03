(* ref patterns read a cell's contents while matching. *)
fun get (ref v) = v
val r = ref 4
val () = print (Int.toString (get r) ^ "\n")
val f = fn ref (a, b) => a + b
val () = print (Int.toString (f (ref (2, 3))) ^ "\n")
fun describe (ref []) = "empty"
  | describe (ref [x]) = "one " ^ Int.toString x
  | describe (ref (_ :: _)) = "many"
val () = print (describe (ref []) ^ ", " ^ describe (ref [9]) ^ ", " ^ describe (ref [1, 2]) ^ "\n")
val ref (ref inner) = ref (ref "deep")
val () = print (inner ^ "\n")
(* The pattern reads the cell when it is matched, not when it was made. *)
val cell = ref 1
val () = cell := 2
val ref now = cell
val () = print (Int.toString now ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
(* CHECK-STDOUT-NEXT: 5 *)
(* CHECK-STDOUT-NEXT: empty, one 9, many *)
(* CHECK-STDOUT-NEXT: deep *)
(* CHECK-STDOUT-NEXT: 2 *)
