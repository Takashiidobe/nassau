structure S = struct datatype d = A | B | C end
fun f S.A = 1 | f S.B = 2 | f S.C = 3
fun g S.A = 1 | g S.B = 2
fun h x = case x of S.A => 1 | S.B => 2 | S.C => 3
open S
fun i A = 1 | i B = 2 | i C = 3
fun j A = 1 | j B = 2
(* CHECK-STDOUT: (structure (S (struct (datatype (d () (A) (B) (C)))))) *)
(* CHECK-STDOUT-NEXT: (fun (f ((S.A) 1) ((S.B) 2) ((S.C) 3))) *)
(* CHECK-STDOUT-NEXT: (fun (g ((S.A) 1) ((S.B) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (h ((x) (case x (S.A 1) (S.B 2) (S.C 3))))) *)
(* CHECK-STDOUT-NEXT: (open S) *)
(* CHECK-STDOUT-NEXT: (fun (i ((A) 1) ((B) 2) ((C) 3))) *)
(* CHECK-STDOUT-NEXT: (fun (j ((A) 1) ((B) 2))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 3:7 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 7:7 *)
(* CHECK-RUN-EXIT: 0 *)
