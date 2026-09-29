(* The runtime's exceptions are raised like any other, so handlers catch
   them. *)
fun divide (a, b) = a div b handle Div => 0
val () = print (Int.toString (divide (7, 2)) ^ " " ^ Int.toString (divide (7, 0)) ^ "\n")
fun grow n = grow (n * 2) handle Overflow => n
val () = print (Int.toString (grow 1) ^ "\n")
fun first (x :: _) = x
val () = print (Int.toString (first [] handle Match => ~1) ^ "\n")
fun bound xs = let val [x] = xs in x end handle Bind => 0
val () = print (Int.toString (bound [4]) ^ " " ^ Int.toString (bound [1, 2]) ^ "\n")
fun safely f = f () handle Fail message => "failed: " ^ message
val () = print (safely (fn () => raise Fail "no") ^ "\n")
(* The same handler can tell them apart. *)
fun classify f =
  (Int.toString (f ()))
  handle Div => "div" | Overflow => "overflow" | Match => "match" | Bind => "bind"
val () = print (classify (fn () => 1 mod 0) ^ " " ^ classify (fn () => 1073741823 + 1)
  ^ " " ^ classify (fn () => first []) ^ " " ^ classify (fn () => 5) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 0 *)
(* CHECK-STDOUT-NEXT: 536870912 *)
(* CHECK-STDOUT-NEXT: ~1 *)
(* CHECK-STDOUT-NEXT: 4 0 *)
(* CHECK-STDOUT-NEXT: failed: no *)
(* CHECK-STDOUT-NEXT: div overflow match 5 *)
