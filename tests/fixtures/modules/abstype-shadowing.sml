val A = 100
val B = "outer"
structure S = struct
  abstype t = A | B of int with
    val zero = A
    fun make n = B n
    fun get A = 0 | get (B n) = n
  end
end
val () = print (Int.toString (S.get S.zero + S.get (S.make 7)) ^ "\n")
val () = print (Int.toString A ^ " " ^ B ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 7 *)
(* CHECK-STDOUT-NEXT: 100 outer *)
