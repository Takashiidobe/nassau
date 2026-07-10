val unit = ()
val pair = (1, "x")
val triple = (1, 2.5, true)
val record = {a = 1, b = "two"}
val numbered = {1 = "one", 2 = "two"}
val empty = {}
val selected = #b record
val nested = ((1, 2), [3])
val parenthesised = (((1)))
(* CHECK-STDOUT: (val unit ()) *)
(* CHECK-STDOUT-NEXT: (val pair (tuple 1 "x")) *)
(* CHECK-STDOUT-NEXT: (val triple (tuple 1 2.5 true)) *)
(* CHECK-STDOUT-NEXT: (val record (record (a 1) (b "two"))) *)
(* CHECK-STDOUT-NEXT: (val numbered (record (1 "one") (2 "two"))) *)
(* CHECK-STDOUT-NEXT: (val empty ()) *)
(* CHECK-STDOUT-NEXT: (val selected (app (# b) record)) *)
(* CHECK-STDOUT-NEXT: (val nested (tuple (tuple 1 2) (list 3))) *)
(* CHECK-STDOUT-NEXT: (val parenthesised 1) *)
