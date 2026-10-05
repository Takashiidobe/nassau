(* mlton regression/time4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)

val t = valOf (Time.fromString "0.417")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString "0.999")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString ".417")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString ".999")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString "~0.417")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString "~0.999")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString "~.417")
val () = print (concat [Time.toString t, "\n"])

val t = valOf (Time.fromString "~.999")
val () = print (concat [Time.toString t, "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 0.417 *)
(* CHECK-STDOUT-NEXT: 0.999 *)
(* CHECK-STDOUT-NEXT: 0.417 *)
(* CHECK-STDOUT-NEXT: 0.999 *)
(* CHECK-STDOUT-NEXT: ~0.417 *)
(* CHECK-STDOUT-NEXT: ~0.999 *)
(* CHECK-STDOUT-NEXT: ~0.417 *)
(* CHECK-STDOUT-NEXT: ~0.999 *)
