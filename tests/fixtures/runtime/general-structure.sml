fun pi n = print (Int.toString n ^ "\n")
val inc = fn x => x + 1
val dbl = fn x => x * 2
val () = pi ((inc o dbl) 5)
val () = pi ((General.o (dbl, inc)) 5)
val () = pi (((inc o inc) o (dbl o dbl)) 1)
val calls = ref 0
val v = (calls := !calls + 1; 7) before (calls := !calls * 10)
val () = pi (v + !calls)
val () = pi (General.before (3, ()) + 1)
val () = ignore (pi 99)
val () = General.ignore 5
val units = map ignore [1, 2, 3]
val () = pi (length units)
val () = pi (foldl (op +) 0 (map (inc o dbl) [1, 2, 3]))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 11 *)
(* CHECK-STDOUT-NEXT: 12 *)
(* CHECK-STDOUT-NEXT: 6 *)
(* CHECK-STDOUT-NEXT: 17 *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: 99 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 15 *)
