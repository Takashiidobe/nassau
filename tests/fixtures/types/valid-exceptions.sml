exception Empty
exception Failed of string
exception Pair of int * string
exception Alias = Failed
exception Fn of int -> int
val e = Empty
val f = Failed
val g = Pair
val alias_of = Alias
val values = [Empty, Failed "s", Pair (1, "a"), Fn (fn x => x)]
val raised = fn () => raise Empty
val poly = fn x => raise Failed x
val handled = (raise Empty) handle Empty => 0
val payload = (raise Failed "boom") handle Failed msg => size msg
val alias_handled = (raise Failed "x") handle Alias msg => msg
val builtin = (raise Fail "y") handle Fail m => m
val many = (raise Pair (1, "a")) handle Pair (n, s) => n + size s | Empty => 0 | _ => 1
fun risky n = if n < 0 then raise Failed "negative" else n
fun safe n = risky n handle Failed _ => 0
val result = safe 3 + safe (0 - 1)
val catch_all = fn x => (x + 1) handle _ => 0
val bound_exn = fn e => (raise e) handle Empty => 1
val fun_exn = case Fn (fn n => n + 1) of Fn f => f 1 | _ => 0
(* CHECK-STDOUT: val e : exn *)
(* CHECK-STDOUT-NEXT: val f : string -> exn *)
(* CHECK-STDOUT-NEXT: val g : int * string -> exn *)
(* CHECK-STDOUT-NEXT: val alias_of : string -> exn *)
(* CHECK-STDOUT-NEXT: val values : exn list *)
(* CHECK-STDOUT-NEXT: val raised : unit -> 'a *)
(* CHECK-STDOUT-NEXT: val poly : string -> 'a *)
(* CHECK-STDOUT-NEXT: val handled : int *)
(* CHECK-STDOUT-NEXT: val payload : int *)
(* CHECK-STDOUT-NEXT: val alias_handled : string *)
(* CHECK-STDOUT-NEXT: val builtin : string *)
(* CHECK-STDOUT-NEXT: val many : int *)
(* CHECK-STDOUT-NEXT: val risky : int -> int *)
(* CHECK-STDOUT-NEXT: val safe : int -> int *)
(* CHECK-STDOUT-NEXT: val result : int *)
(* CHECK-STDOUT-NEXT: val catch_all : int -> int *)
(* CHECK-STDOUT-NEXT: val bound_exn : exn -> int *)
(* CHECK-STDOUT-NEXT: val fun_exn : int *)
(* CHECK-RUN-EXIT: 0 *)
