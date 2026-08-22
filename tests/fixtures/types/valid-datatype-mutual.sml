datatype tree = Tree of forest
and forest = Empty | Trees of tree * forest
datatype expr = Num of int | Add of pair
withtype pair = expr * expr
fun count (Tree f) = 1 + count_forest f
and count_forest Empty = 0
  | count_forest (Trees (t, f)) = count t + count_forest f
fun eval (Num n) = n | eval (Add (a, b)) = eval a + eval b
val small = Tree (Trees (Tree Empty, Empty))
val sum = Add (Num 1, Add (Num 2, Num 3))
val n = count small
(* CHECK-STDOUT: val count : tree -> int *)
(* CHECK-STDOUT-NEXT: val count_forest : forest -> int *)
(* CHECK-STDOUT-NEXT: val eval : expr -> int *)
(* CHECK-STDOUT-NEXT: val small : tree *)
(* CHECK-STDOUT-NEXT: val sum : expr *)
(* CHECK-STDOUT-NEXT: val n : int *)
(* CHECK-RUN-EXIT: 0 *)
