val kept = 41;;
fun saved () = kept;;
clear;;
saved ();;
reset;;
kept;;
val kept = 7;;
kept;;
val clear = 99;;
clear + 1;;
val reset = 5;;
reset + 1;;
"clear;;";;
reset;;
val kept = 9;;
kept;;
(* REPL-COMMANDS *)
(* CHECK-REPL: val kept = 41 : int *)
(* CHECK-REPL-NEXT: val saved = fn : unit -> int *)
(* CHECK-REPL-NEXT: Cleared *)
(* CHECK-REPL-NEXT: val it = 41 : int *)
(* CHECK-REPL-NEXT: Reset *)
(* CHECK-REPL-NEXT: val kept = 7 : int *)
(* CHECK-REPL-NEXT: val it = 7 : int *)
(* CHECK-REPL-NEXT: val clear = 99 : int *)
(* CHECK-REPL-NEXT: val it = 100 : int *)
(* CHECK-REPL-NEXT: val reset = 5 : int *)
(* CHECK-REPL-NEXT: val it = 6 : int *)
(* CHECK-REPL-NEXT: val it = "clear;;" : string *)
(* CHECK-REPL-NEXT: Reset *)
(* CHECK-REPL-NEXT: val kept = 9 : int *)
(* CHECK-REPL-NEXT: val it = 9 : int *)
(* CHECK-ERR: unbound variable *)
