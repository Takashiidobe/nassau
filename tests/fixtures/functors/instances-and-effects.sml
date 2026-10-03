val state = ref 0
val base = 10
structure Captured = struct val n = 100 end
functor F (X : sig val step : int end) = struct
  val () = state := !state + X.step
  val value = base + Captured.n + X.step
  exception E of int
  fun fail n = raise E n
  datatype t = A | B of int
  fun get A = 0 | get (B n) = n
end
val base = "shadowed"
structure Captured = struct val n = 1000 end
structure One = F (struct val step = (state := !state + 10; 1) end)
structure Two = F (struct val step = 2 end)
val () = print (Int.toString (!state) ^ " " ^ Int.toString (One.value + Two.value) ^ "\n")
val () = print ((One.fail 4 handle Two.E _ => "wrong" | One.E n => Int.toString n) ^ "\n")
val () = print (Int.toString (One.get (One.B 5) + Two.get (Two.B 6)) ^ "\n")
local
  structure Impl = struct val n = 7 end
  functor Private (X : sig end) = struct val n = Impl.n end
in
  functor Public (X : sig end) = Private (X)
end
structure P = Public (struct end)
val () = print (Int.toString P.n ^ "\n")
(* MLTON-SKIP: MLton rejects functors declared inside local *)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 13 223 *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: 11 *)
(* CHECK-STDOUT-NEXT: 7 *)
