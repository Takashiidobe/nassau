val xs = [1, 2, 3]
val size = List.length xs
val curried = List.map (fn x => x + 1) xs
val nested = List.length (List.map (fn x => x) xs)
val tight = List.length xs + List.length xs * 2
val negated = ~ 3 + 1
val selected = #1 (1, "a") + 2
(* RUNTIME-SKIP: unbound variable '~' *)
(* CHECK-STDOUT: (val xs (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val size (app List.length xs)) *)
(* CHECK-STDOUT-NEXT: (val curried (app (app List.map (fn (x (+ x 1)))) xs)) *)
(* CHECK-STDOUT-NEXT: (val nested (app List.length (app (app List.map (fn (x x))) xs))) *)
(* CHECK-STDOUT-NEXT: (val tight (+ (app List.length xs) {{[(]}}* (app List.length xs) 2))) *)
(* CHECK-STDOUT-NEXT: (val negated (+ (app ~ 3) 1)) *)
(* CHECK-STDOUT-NEXT: (val selected (+ (app (# 1) (tuple 1 "a")) 2)) *)
(* CHECK-RUN-ERR: × unbound variable '~' *)
(* CHECK-RUN-ERR: :6:15] *)
