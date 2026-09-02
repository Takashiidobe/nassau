(* mlton regression/array2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
open Array2

val x = ref 0

val i2s = Int.toString
   
fun test trv =
   let
      val a =
         tabulate trv
         (3, 4, fn (r, c) =>
          (x := !x + 1
           ; concat["(", i2s r, ", ", i2s c, ", ", i2s(!x), ")"]))
      val _ = app trv (fn s => (print s; print "\n")) a
   in ()
   end

val _ = (test RowMajor; test ColMajor)

(* Check that Size is correctly raised when constructing large arrays. *)
val m = valOf Int.maxInt

val _ =
   (array (m, 2, 13)
    ; print "FAIL")
   handle Size => print "OK"
      
val _ =
   (tabulate RowMajor (m, 2, fn _ => 13)
    ; print "FAIL")
   handle Size => print "OK\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: (0, 0, 1) *)
(* CHECK-STDOUT-NEXT: (0, 1, 2) *)
(* CHECK-STDOUT-NEXT: (0, 2, 3) *)
(* CHECK-STDOUT-NEXT: (0, 3, 4) *)
(* CHECK-STDOUT-NEXT: (1, 0, 5) *)
(* CHECK-STDOUT-NEXT: (1, 1, 6) *)
(* CHECK-STDOUT-NEXT: (1, 2, 7) *)
(* CHECK-STDOUT-NEXT: (1, 3, 8) *)
(* CHECK-STDOUT-NEXT: (2, 0, 9) *)
(* CHECK-STDOUT-NEXT: (2, 1, 10) *)
(* CHECK-STDOUT-NEXT: (2, 2, 11) *)
(* CHECK-STDOUT-NEXT: (2, 3, 12) *)
(* CHECK-STDOUT-NEXT: (0, 0, 13) *)
(* CHECK-STDOUT-NEXT: (1, 0, 14) *)
(* CHECK-STDOUT-NEXT: (2, 0, 15) *)
(* CHECK-STDOUT-NEXT: (0, 1, 16) *)
(* CHECK-STDOUT-NEXT: (1, 1, 17) *)
(* CHECK-STDOUT-NEXT: (2, 1, 18) *)
(* CHECK-STDOUT-NEXT: (0, 2, 19) *)
(* CHECK-STDOUT-NEXT: (1, 2, 20) *)
(* CHECK-STDOUT-NEXT: (2, 2, 21) *)
(* CHECK-STDOUT-NEXT: (0, 3, 22) *)
(* CHECK-STDOUT-NEXT: (1, 3, 23) *)
(* CHECK-STDOUT-NEXT: (2, 3, 24) *)
(* CHECK-STDOUT-NEXT: OKOK *)
