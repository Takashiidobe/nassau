datatype expr = Number of int
              | Add of expr * expr
              | Multiply of expr * expr;

fun eval (Number n) = n
  | eval (Add (a, b)) = eval a + eval b
  | eval (Multiply (a, b)) = eval a * eval b;

val expression = Multiply (Add (Number 3, Number 4), Number 6);
eval expression;
