abstype counter = C of int
with
  val zero = C 0
  fun tick (C n) = C (n + 1)
  fun value (C n) = n
end
val () = print (Int.toString (value (tick (tick zero))) ^ "\n")
datatype 'a result = Ok of 'a | Error of string
datatype outcome = datatype result
fun twice (x : int outcome) = case x of Ok n => Ok (n * 2) | e => e
fun get (Ok x) = x | get (Error _) = ~1
val () = print (Int.toString (get (twice (Ok 3)) + get (Error "no")) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
(* CHECK-STDOUT-NEXT: 5 *)
