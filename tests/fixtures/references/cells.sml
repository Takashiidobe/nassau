(* A reference is a mutable cell: ref makes one, ! reads it and := writes
   it. Cells are shared, not copied, and compare equal only to themselves. *)
val r = ref 0
val () = r := !r + 5
val () = print (Int.toString (!r) ^ "\n")
val alias = r
val () = alias := 7
val () = print (Int.toString (!r) ^ "\n")
val () = print (if r = alias andalso r <> ref 7 then "same cell\n" else "wrong\n")
(* Cells hold any value, including other cells and functions. *)
val s = ref "a"
val () = s := !s ^ "b"
val nested = ref (ref 1)
val () = !nested := 2
val step = ref (fn x => x + 1)
val () = step := (fn x => x * 10)
val () = print (!s ^ " " ^ Int.toString (! (!nested)) ^ " " ^ Int.toString (!step 3) ^ "\n")
(* A closure keeps its cell between calls. *)
fun counter () = let val n = ref 0 in fn () => (n := !n + 1; !n) end
val tick = counter ()
val _ = tick ()
val _ = tick ()
val () = print (Int.toString (tick ()) ^ " " ^ Int.toString (counter () ()) ^ "\n")
(* ! and := are functions too. *)
val get = !
val set = op :=
val () = set (r, get r + 1)
fun app f [] = () | app f (x :: xs) = (f x; app f xs)
val cells = [ref 1, ref 2]
val () = app (fn c => c := !c * 10) cells
val () = app (fn c => print (Int.toString (get c) ^ " ")) (r :: cells)
val () = print "\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5 *)
(* CHECK-STDOUT-NEXT: 7 *)
(* CHECK-STDOUT-NEXT: same cell *)
(* CHECK-STDOUT-NEXT: ab 2 30 *)
(* CHECK-STDOUT-NEXT: 3 1 *)
(* CHECK-STDOUT-NEXT: 8 10 20 *)
