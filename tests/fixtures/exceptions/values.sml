(* Exceptions are values of the open datatype exn: they can be stored,
   passed, and taken apart by matching on their constructors. *)
exception Oops
exception Code of int
exception Message of string * int
exception Again = Oops
fun describe Oops = "oops"
  | describe (Code n) = "code " ^ Int.toString n
  | describe (Message (text, n)) = text ^ " " ^ Int.toString n
  | describe (Fail text) = "fail " ^ text
  | describe Div = "div"
  | describe _ = "something else"
fun map f [] = [] | map f (x :: xs) = f x :: map f xs
fun join [] = "" | join [x] = x | join (x :: xs) = x ^ ", " ^ join xs
val all = [Oops, Code 3, Message ("hi", 2), Fail "no", Div, Overflow, Again]
val () = print (join (map describe all) ^ "\n")
(* Constructors with an argument are functions. *)
val codes = map Code [1, 2]
val () = print (join (map describe codes) ^ "\n")
(* Each evaluation of an exception declaration makes a new exception. *)
fun fresh () =
  let exception Local of int
  in (Local 1, fn Local n => "mine " ^ Int.toString n | _ => "not mine")
  end
val (first, isFirst) = fresh ()
val (second, _) = fresh ()
val () = print (isFirst first ^ ", " ^ isFirst second ^ "\n")
(* A replication shares its original's identity. (SML/NJ 110.79 fails to
   match a value built with the original against the replication's name in
   case and fn, though the Definition says it should, so only this direction
   is checked here.) *)
exception Copy = Code
val () = print (describe (Copy 9) ^ "\n")
local exception Hidden in val hidden = Hidden fun isHidden Hidden = true | isHidden _ = false end
val () = print (if isHidden hidden andalso not (isHidden Oops) then "hidden\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: oops, code 3, hi 2, fail no, div, something else, oops *)
(* CHECK-STDOUT-NEXT: code 1, code 2 *)
(* CHECK-STDOUT-NEXT: mine 1, not mine *)
(* CHECK-STDOUT-NEXT: code 9 *)
(* CHECK-STDOUT-NEXT: hidden *)
