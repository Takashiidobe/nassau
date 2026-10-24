datatype node = End | Node of node ref
val live = ref End
val () = live := Node live
fun allocate 0 = ()
  | allocate n =
      let val dead = ref End
          val () = dead := Node dead
      in allocate (n - 1) end
val () = allocate 50000
val () = case !live of Node link => print (if link = live then "cycle survived\n" else "lost\n")
                    | End => raise Fail "lost cycle"
fun sum 0 = 0 | sum n = 1 + sum (n - 1)
val () = print (Int.toString (sum 20000) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: cycle survived *)
(* CHECK-STDOUT-NEXT: 20000 *)
(* WASM-SKIP: nassau-gph: deep non-tail recursion traps in the wasm build *)
(* WASM-SKIP: deep non-tail recursion traps in the wasm build (nassau-gph) *)
