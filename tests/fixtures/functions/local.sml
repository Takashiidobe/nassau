(* local hides its private declarations from what follows. *)
local
  val secret = 41
  fun helper n = n + secret
in
  val answer = helper 1
  fun addSecret n = helper n
end
val secret = "shadowed"
val _ = print (Int.toString answer ^ " " ^ Int.toString (addSecret 9) ^ " " ^ secret ^ "\n")
(* local inside let, with nested lets. *)
val total =
  let
    local
      val base = 100
    in
      fun bump n = let val m = n + 1 in base + m end
    end
  in
    bump 1 + bump 2
  end
val _ = print (Int.toString total ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 42 50 shadowed *)
(* CHECK-STDOUT-NEXT: 205 *)
