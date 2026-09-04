fun show xs = foldl (fn (n, acc) => acc ^ Int.toString n ^ ";") "" xs
val evens = List.filter (fn n => n mod 2 = 0) [1, 2, 3, 4, 5, 6]
val () = print (show evens ^ "\n")
val () = print (show (List.filter (fn _ => true) [3, 1, 2]) ^ "|" ^ show (List.filter (fn _ => false) [3, 1, 2]) ^ "|" ^ show (List.filter (fn _ => true) []) ^ "\n")
val short = List.filter (fn s => size s > 1) ["a", "bb", "c", "ddd"]
val () = print (foldl (fn (s, acc) => acc ^ s ^ ";") "" short ^ "\n")
val positive = List.filter (fn n => n > 0)
val () = print (show (positive [~1, 2, ~3, 4]) ^ " " ^ show (positive []) ^ "\n")
val keep = List.filter
val () = print (show (keep (fn n => n < 3) [1, 2, 3, 4]) ^ "\n")
fun upto n = if n = 0 then [] else n :: upto (n - 1)
val () = print (Int.toString (length (List.filter (fn n => n mod 1000 = 0) (upto 20000))) ^ "\n")
val calls = ref 0
val () = print (show (List.filter (fn n => (calls := !calls + 1; n > 1)) [1, 2, 3]) ^ " " ^ Int.toString (!calls) ^ "\n")
val result = (List.filter (fn n => if n = 2 then raise Fail "stop" else true) [1, 2, 3]; "wrong") handle Fail text => text
val () = print (result ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2;4;6; *)
(* CHECK-STDOUT-NEXT: 3;1;2;|| *)
(* CHECK-STDOUT-NEXT: bb;ddd; *)
(* CHECK-STDOUT-NEXT: 2;4; *)
(* CHECK-STDOUT-NEXT: 1;2; *)
(* CHECK-STDOUT-NEXT: 20 *)
(* CHECK-STDOUT-NEXT: 2;3; 3 *)
(* CHECK-STDOUT-NEXT: stop *)
