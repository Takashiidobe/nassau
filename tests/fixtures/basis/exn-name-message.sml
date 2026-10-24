exception E
exception F of int
fun p x = print (exnName x ^ " | " ^ exnMessage x ^ "\n")
val _ = p Div
val _ = p Overflow
val _ = p Subscript
val _ = p (Fail "boom")
val _ = p (Fail "a\"b\n")
val _ = p Match
val _ = p Bind
val _ = p Option
val _ = p Empty
val _ = p Size
val _ = p Chr
val _ = p Domain
val _ = p E
val _ = print (exnName (F 3) ^ "\n")
val _ = print (General.exnName (Fail "x") ^ "\n")
(* CHECK-STDOUT: Div | Div *)
(* CHECK-STDOUT-NEXT: Overflow | Overflow *)
(* CHECK-STDOUT-NEXT: Subscript | Subscript *)
(* CHECK-STDOUT-NEXT: Fail | Fail "boom" *)
(* CHECK-STDOUT-NEXT: Fail | Fail "a\"b\n" *)
(* CHECK-STDOUT-NEXT: Match | Match *)
(* CHECK-STDOUT-NEXT: Bind | Bind *)
(* CHECK-STDOUT-NEXT: Option | Option *)
(* CHECK-STDOUT-NEXT: Empty | Empty *)
(* CHECK-STDOUT-NEXT: Size | Size *)
(* CHECK-STDOUT-NEXT: Chr | Chr *)
(* CHECK-STDOUT-NEXT: Domain | Domain *)
(* CHECK-STDOUT-NEXT: E | E *)
(* CHECK-STDOUT-NEXT: F *)
(* CHECK-STDOUT-NEXT: Fail *)
(* CHECK-EXIT: 0 *)
