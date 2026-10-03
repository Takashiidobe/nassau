signature VALUES = sig
  type t
  val A : t
  val B : int -> t
  val get : t -> int
  val E : exn
  val F : int -> exn
end
structure V :> VALUES = struct
  datatype t = A | B of int
  fun get A = 0 | get (B n) = n
  exception E
  exception F of int
end
val () = print (Int.toString (V.get V.A + V.get (V.B 7)) ^ "\n")
val caught = (raise V.F 3) handle _ => "caught"
val () = print (caught ^ "\n")
val () = print (let open V in (fn A => Int.toString (get A)) (B 9) end ^ "\n")
val () = print ((raise V.E) handle _ => "nullary\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 7 *)
(* CHECK-STDOUT-NEXT: caught *)
(* CHECK-STDOUT-NEXT: 9 *)
(* CHECK-STDOUT-NEXT: nullary *)
