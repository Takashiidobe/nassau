(* The first rule that matches wins, however the rules overlap. *)
fun classify (0, _) = "zero first"
  | classify (_, 0) = "zero second"
  | classify (1, 1) = "ones"
  | classify (x, y) = if x = y then "same" else "different"
val () = print (classify (0, 0) ^ "\n")
val () = print (classify (5, 0) ^ "\n")
val () = print (classify (1, 1) ^ "\n")
val () = print (classify (2, 2) ^ "\n")
val () = print (classify (2, 3) ^ "\n")
fun zip ([], _) = []
  | zip (_, []) = []
  | zip (x :: xs, y :: ys) = (x, y) :: zip (xs, ys)
fun sum [] = 0 | sum ((a, b) :: rest) = a * b + sum rest
val () = print (Int.toString (sum (zip ([1, 2, 3], [4, 5]))) ^ "\n")
fun last [x] = x
  | last (_ :: rest) = last rest
  | last [] = ~1
val () = print (Int.toString (last [1, 2, 3]) ^ " " ^ Int.toString (last []) ^ "\n")
fun merge (xs, []) = xs
  | merge ([], ys) = ys
  | merge (l as x :: xs, r as y :: ys) =
      if x <= y then x :: merge (xs, r) else y :: merge (l, ys)
fun join [] = "" | join [x] = Int.toString x | join (x :: rest) = Int.toString x ^ "," ^ join rest
val () = print (join (merge ([1, 4, 9], [2, 3, 10, 11])) ^ "\n")
fun lookup (key, []) = NONE
  | lookup (key, (k, v) :: rest) = if k = key then SOME v else lookup (key, rest)
val table = [("one", 1), ("two", 2)]
val () =
  case (lookup ("two", table), lookup ("six", table)) of
      (SOME a, NONE) => print ("found " ^ Int.toString a ^ "\n")
    | (SOME _, SOME _) => print "both\n"
    | (NONE, _) => print "neither\n"
fun describe (SOME (x :: _), _) = "some list starting " ^ Int.toString x
  | describe (SOME [], true) = "some empty, flag"
  | describe (NONE, true) = "none, flag"
  | describe (_, false) = "no flag"
val () = print (describe (SOME [7, 8], false) ^ "\n")
val () = print (describe (SOME [], true) ^ "\n")
val () = print (describe (NONE, true) ^ "\n")
val () = print (describe (NONE, false) ^ "\n")
fun compare (a : int, b) = if a < b then LESS else if a > b then GREATER else EQUAL
val () = print (case compare (3, 2) of GREATER => "greater\n" | _ => "not greater\n")
val r = ref (SOME 3)
val () = case r of ref (SOME n) => print (Int.toString n ^ "\n") | ref NONE => ()
fun opts f = (f NONE, f (SOME 1))
val (a, b) = opts (fn NONE => 0 | SOME n => n + 1)
val () = print (Int.toString (a + b) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: zero first *)
(* CHECK-STDOUT-NEXT: zero second *)
(* CHECK-STDOUT-NEXT: ones *)
(* CHECK-STDOUT-NEXT: same *)
(* CHECK-STDOUT-NEXT: different *)
(* CHECK-STDOUT-NEXT: 14 *)
(* CHECK-STDOUT-NEXT: 3 ~1 *)
(* CHECK-STDOUT-NEXT: 1,2,3,4,9,10,11 *)
(* CHECK-STDOUT-NEXT: found 2 *)
(* CHECK-STDOUT-NEXT: some list starting 7 *)
(* CHECK-STDOUT-NEXT: some empty, flag *)
(* CHECK-STDOUT-NEXT: none, flag *)
(* CHECK-STDOUT-NEXT: no flag *)
(* CHECK-STDOUT-NEXT: greater *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 2 *)
