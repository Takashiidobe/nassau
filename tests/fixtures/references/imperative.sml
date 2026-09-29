(* while loops, sequences, before and ignore. *)
fun sum n =
  let
    val total = ref 0
    val i = ref 1
  in
    while !i <= n do (total := !total + !i; i := !i + 1);
    !total
  end
val () = print (Int.toString (sum 100) ^ "\n")
(* A loop runs in constant stack however long it runs. *)
val i = ref 0
val () = while !i < 1000000 do i := !i + 1
val () = print (Int.toString (!i) ^ "\n")
(* before evaluates both sides in order and keeps the left one. *)
val x = (print "left\n"; 1) before print "right\n"
val () = print (Int.toString x ^ "\n")
fun pop stack = case !stack of [] => 0 | top :: rest => top before stack := rest
val stack = ref [3, 2, 1]
val () = print (Int.toString (pop stack) ^ Int.toString (pop stack) ^ "\n")
(* ignore evaluates its argument for its effects. *)
val () = ignore (pop stack)
val () = print (Int.toString (pop stack) ^ "\n")
val () = ignore (print "ignored\n")
(* A while loop's value is (), and it can end by raising. *)
exception Found of int
fun find p xs =
  let val rest = ref xs
  in (while true do
        case !rest of
          [] => raise Found ~1
        | x :: more => if p x then raise Found x else rest := more;
      0) handle Found x => x
  end
val () = print (Int.toString (find (fn x => x > 2) [1, 2, 3, 4]) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5050 *)
(* CHECK-STDOUT-NEXT: 1000000 *)
(* CHECK-STDOUT-NEXT: left *)
(* CHECK-STDOUT-NEXT: right *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: 32 *)
(* CHECK-STDOUT-NEXT: 0 *)
(* CHECK-STDOUT-NEXT: ignored *)
(* CHECK-STDOUT-NEXT: 3 *)
