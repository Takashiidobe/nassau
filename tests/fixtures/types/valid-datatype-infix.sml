infix 5 ++
infixr 6 ==>
datatype expr = Lit of int | ++ of expr * expr | ==> of expr * expr
fun size (Lit _) = 1
  | size (a ++ b) = size a + size b
  | size (a ==> b) = size a + size b
val built = Lit 1 ++ Lit 2 ==> Lit 3
val n = size built
val plus = op ++
(* RUNTIME-SKIP: the infix operator ++ are not supported by code generation yet *)
(* CHECK-STDOUT: val size : expr -> int *)
(* CHECK-STDOUT-NEXT: val built : expr *)
(* CHECK-STDOUT-NEXT: val n : int *)
(* CHECK-STDOUT-NEXT: val plus : expr * expr -> expr *)
(* CHECK-RUN-ERR: × the infix operator ++ are not supported by code generation yet *)
(* CHECK-RUN-ERR: :7:13] *)
