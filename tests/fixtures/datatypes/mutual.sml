datatype expr = Num of int | Add of expr * expr | Let of decl * expr | Var of string
and decl = Val of string * expr
withtype env = (string * int) list
fun lookup (name, (n, v) :: rest : env) = if n = name then v else lookup (name, rest)
  | lookup (_, []) = 0
fun eval (env, Num n) = n
  | eval (env, Add (a, b)) = eval (env, a) + eval (env, b)
  | eval (env, Var x) = lookup (x, env)
  | eval (env, Let (d, body)) = eval (bind (env, d), body)
and bind (env, Val (x, e)) = (x, eval (env, e)) :: env
val program = Let (Val ("x", Num 4), Add (Var "x", Let (Val ("y", Add (Var "x", Num 1)), Add (Var "y", Var "x"))))
val () = print (Int.toString (eval ([], program)) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 13 *)
