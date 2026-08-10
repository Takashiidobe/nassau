structure Argument = struct
  datatype t = A | B
  fun get A = 0 | get B = 1
  val hidden = 9
end
functor Specs (type t val A : t val get : t -> int) = struct
  fun bind A = get A
  val value = get A
end
structure S = Specs (Argument)
val () = print (Int.toString (S.bind Argument.B) ^ " " ^ Int.toString S.value ^ "\n")
structure E = struct exception E exception F of int end
functor Values (val E : exn val F : int -> exn) = struct
  fun bind E = raise E
  val value = F 5
end
structure V = Values (E)
val () = print ((V.bind (Fail "bound") handle Fail text => text | _ => "wrong") ^ "\n")
val () = print (((raise V.value) handle E.F n => Int.toString n) ^ "\n")
functor Unused () = struct val () = print "wrong\n" end
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 0 *)
(* CHECK-STDOUT-NEXT: bound *)
(* CHECK-STDOUT-NEXT: 5 *)
