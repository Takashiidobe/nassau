(* The REPL shows a reference's contents. *)
val r = ref 3;
val () = r := !r + 1;
val cells = [ref 1, ref 2];
val nested = ref (ref (SOME 5));
val s = ref "text";
(* CHECK-REPL: val r = ref 3 : int ref *)
(* CHECK-REPL-NEXT: val cells = [ref 1,ref 2] : int ref list *)
(* CHECK-REPL-NEXT: val nested = ref (ref (SOME 5)) : int option ref ref *)
(* CHECK-REPL-NEXT: val s = ref "text" : string ref *)
