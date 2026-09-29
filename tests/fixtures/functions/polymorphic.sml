(* Polymorphic functions are compiled once and work at every type. *)
fun id x = x
fun length [] = 0
  | length (_ :: rest) = 1 + length rest
fun map f [] = []
  | map f (x :: rest) = f x :: map f rest
fun swap (a, b) = (b, a)
fun first (x :: _) = x
fun last [x] = x
  | last (_ :: rest) = last rest
val one = id 1
val half = id 0.5
val word = id "word"
val list = id [1, 2, 3]
val tuple = id (1, "one", 1.5)
val _ = print (Int.toString one ^ " " ^ word ^ "\n")
val _ = print (if half < 1.0 andalso half > 0.25 then "real kept\n" else "real lost\n")
val _ = print (Int.toString (length list) ^ " " ^ Int.toString (length ["a", "b"]) ^ " "
               ^ Int.toString (length [[1], [], [2, 3]]) ^ " " ^ Int.toString (length [1.5, 2.5]) ^ "\n")
val (text, number) = swap (7, "seven")
val _ = print (text ^ " " ^ Int.toString number ^ "\n")
val reals = map (fn r => r * 2.0) [1.25, 2.5]
val _ = print (if first reals > 2.4 andalso last reals > 4.9 then "reals doubled\n" else "wrong\n")
val lengths = map length [[1, 2], [], [3]]
val _ = print (Int.toString (first lengths + last lengths) ^ "\n")
val (_, name, _) = tuple
val _ = print (name ^ " " ^ last (map (fn (s, n) => s ^ Int.toString n) [("a", 1), ("b", 2)]) ^ "\n")
(* Equality-polymorphic functions compare with the runtime's equality. *)
fun member (x, []) = false
  | member (x, y :: rest) = x = y orelse member (x, rest)
fun count (x, []) = 0
  | count (x, y :: rest) = (if x = y then 1 else 0) + count (x, rest)
fun yes b = if b then "yes" else "no"
val _ = print (yes (member (3, [1, 2, 3])) ^ " " ^ yes (member ("b", ["a", "c"])) ^ " "
               ^ yes (member ([1], [[], [1]])) ^ " " ^ yes (member ((1, "x"), [(1, "y"), (1, "x")])) ^ "\n")
val _ = print (Int.toString (count (#"a", [#"a", #"b", #"a"])) ^ " " ^ Int.toString (count (true, [false, true])) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 word *)
(* CHECK-STDOUT-NEXT: real kept *)
(* CHECK-STDOUT-NEXT: 3 2 3 2 *)
(* CHECK-STDOUT-NEXT: seven 7 *)
(* CHECK-STDOUT-NEXT: reals doubled *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: one b2 *)
(* CHECK-STDOUT-NEXT: yes no yes yes *)
(* CHECK-STDOUT-NEXT: 2 1 *)
