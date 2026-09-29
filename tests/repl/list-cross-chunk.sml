val xs = [1, 2]
val nested = [xs]
val empty = []
(* CHECK-REPL: val xs = [1,2] : int list *)
(* CHECK-REPL-NEXT: val nested = {{[[]}}[1,2]] : int list list *)
(* CHECK-REPL-NEXT: val empty = [] : 'a list *)
