(* let binds values and functions for its body, shadowing outer names. *)
val x = 1
val y =
  let
    val x = x + 10
    val x = x * 2
  in
    x + 1
  end
val _ = print (Int.toString x ^ " " ^ Int.toString y ^ "\n")
(* Local functions capture the variables around them. *)
fun scaledSum (factor, n) =
  let
    fun go 0 acc = acc
      | go i acc = go (i - 1) (acc + i * factor)
  in
    go n 0
  end
val _ = print (Int.toString (scaledSum (3, 4)) ^ "\n")
(* Mutually recursive local functions, and a let inside a fn. *)
fun parity n =
  let
    fun even 0 = "even" | even k = odd (k - 1)
    and odd 0 = "odd" | odd k = even (k - 1)
    val result = even n
  in
    result ^ "!"
  end
val _ = print (parity 7 ^ " " ^ parity 10 ^ "\n")
val makeGreeter =
  fn greeting =>
    let
      val prefix = greeting ^ ", "
    in
      fn name => let val message = prefix ^ name in message ^ "\n" end
    end
val _ = print (makeGreeter "hello" "world")
(* Patterns in let bindings, and a let that shadows a function. *)
val (a, b) = let val pair = (2, 3) val (p, q) = pair in (q, p) end
fun double n = n * 2
val z = let fun double n = n * 3 in double 5 end
val _ = print (Int.toString a ^ Int.toString b ^ " " ^ Int.toString z ^ " " ^ Int.toString (double 5) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 23 *)
(* CHECK-STDOUT-NEXT: 30 *)
(* CHECK-STDOUT-NEXT: odd! even! *)
(* CHECK-STDOUT-NEXT: hello, world *)
(* CHECK-STDOUT-NEXT: 32 15 10 *)
