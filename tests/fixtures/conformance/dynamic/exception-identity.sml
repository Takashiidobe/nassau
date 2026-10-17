exception Outer of int
exception Alias = Outer
val _ = print (Int.toString ((raise Alias 4) handle Outer n => n) ^ "\n")
fun fresh () =
  let exception Local
  in (Local, fn e => ((raise e) handle Local => true | _ => false)) end
val (first, isFirst) = fresh ()
val (second, isSecond) = fresh ()
val _ = print (Bool.toString (isFirst first andalso isSecond second) ^ "\n")
val _ = print (Bool.toString (isFirst second orelse isSecond first) ^ "\n")
val state = ref 0
val _ = ((state := 1; raise Outer 2; state := 3) handle Outer n => state := !state + n)
val _ = print (Int.toString (!state) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
(* CHECK-STDOUT-NEXT: true *)
(* CHECK-STDOUT-NEXT: false *)
(* CHECK-STDOUT-NEXT: 3 *)
