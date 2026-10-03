val lengths = map length [[], [1], [1, 2, 3]]
val () = print (if lengths = [0, 1, 3] then "lengths\n" else "wrong\n")
val strings = map (fn n => Int.toString (n + 1)) [1, 2, 3]
val () = print (if strings = ["2", "3", "4"] then "mapped\n" else "wrong\n")
val total = foldl (fn (x, acc) => x + acc) 10 [1, 2, 3]
val reverse : int list -> int list = foldl (fn (x, acc) => x :: acc) []
val () = print (Int.toString total ^ " " ^ Int.toString (length (reverse [1, 2, 3])) ^ "\n")
val () = print (if reverse [1, 2, 3] = [3, 2, 1] then "reversed\n" else "wrong\n")
val calls = ref 0
val result = (map (fn n => (calls := !calls + 1; if n = 2 then raise Fail "stop" else n)) [1, 2, 3]; "wrong")
  handle Fail text => text
val () = print (result ^ " " ^ Int.toString (!calls) ^ "\n")
val sum = foldl (fn (x, acc) => x + acc)
val () = print (Int.toString (sum 3 [4, 5]) ^ " " ^ Int.toString (sum 7 []) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: lengths *)
(* CHECK-STDOUT-NEXT: mapped *)
(* CHECK-STDOUT-NEXT: 16 3 *)
(* CHECK-STDOUT-NEXT: reversed *)
(* CHECK-STDOUT-NEXT: stop 2 *)
(* CHECK-STDOUT-NEXT: 12 7 *)
