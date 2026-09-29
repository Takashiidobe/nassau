(* raise and handle: a handler catches what its body raises, however deep in
   the calls it makes, and passes on what its rules do not match. *)
exception Oops
exception Code of int
exception Message of string * int
fun check n = if n > 3 then raise Code n else n
fun map f [] = [] | map f (x :: xs) = f x :: map f xs
val () = print (Int.toString (check 5 handle Code n => n * 10) ^ "\n")
(* The raise unwinds through map's recursion. *)
val s = (map check [1, 2, 7, 1]; "done") handle Code n => "code " ^ Int.toString n
val () = print (s ^ "\n")
(* An exception no rule matches reaches the enclosing handler. *)
fun inner () = (raise Oops) handle Code _ => 1
val () = print (Int.toString (inner () handle Oops => 2) ^ "\n")
fun safe f x = f x handle _ => ~1
val () = print (Int.toString (safe check 9) ^ " " ^ Int.toString (safe check 1) ^ "\n")
(* Handlers take exceptions apart like case. *)
fun describe f =
  f () handle Code 0 => "zero"
            | Code n => "code " ^ Int.toString n
            | Message (text, n) => text ^ Int.toString n
            | Fail text => "fail " ^ text
val () = print (describe (fn () => raise Code 0) ^ ", " ^ describe (fn () => raise Code 8)
  ^ ", " ^ describe (fn () => raise Message ("m", 3)) ^ ", " ^ describe (fn () => raise Fail "f")
  ^ ", " ^ describe (fn () => "none") ^ "\n")
(* A handler can raise, and a handled value can be a raise's argument. *)
val nested = ((raise Code 1) handle Code n => raise Code (n + 1)) handle Code n => n
val () = print (Int.toString nested ^ "\n")
fun rethrow f = f () handle e => raise e
val () = print ((rethrow (fn () => raise Fail "again")) handle Fail m => m ^ "\n")
(* A replication catches what its original raises. *)
exception Alias = Code
val () = print ((raise Code 6) handle Alias n => "alias " ^ Int.toString n ^ "\n")
(* Each evaluation of a declaration makes a new exception, so a handler for
   one does not catch another. *)
fun fresh () =
  let exception Local
  in (fn () => (raise Local) : unit, fn f => (f () : unit; "none") handle Local => "mine")
  end
val (raiseA, catchA) = fresh ()
val (raiseB, _) = fresh ()
val () = print (catchA raiseA ^ " " ^ (catchA raiseB handle _ => "other") ^ "\n")
(* Handlers do not stop a loop written as recursion from running in constant
   stack: only the handled call keeps its frame. *)
fun count 0 = raise Oops | count n = count (n - 1)
val () = print ((Int.toString (count 1000000)) handle Oops => "counted\n")
(* A handled body's value flows on when nothing is raised. *)
fun total [] = 0 | total (x :: xs) = (check x handle Code n => 0) + total xs
val () = print (Int.toString (total [1, 2, 9, 3]) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 50 *)
(* CHECK-STDOUT-NEXT: code 7 *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: ~1 1 *)
(* CHECK-STDOUT-NEXT: zero, code 8, m3, fail f, none *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: again *)
(* CHECK-STDOUT-NEXT: alias 6 *)
(* CHECK-STDOUT-NEXT: mine other *)
(* CHECK-STDOUT-NEXT: counted *)
(* CHECK-STDOUT-NEXT: 6 *)
