fun make n = let val cell = ref n in fn delta => (cell := !cell + delta; !cell) end
val counter = make 3
val saved = ref (SOME (counter, [1,2,3], "retained"))
fun allocate 0 = ()
  | allocate n = let val r = ref [n, n + 1] in if !r = [n, n + 1] then allocate (n - 1) else raise Fail "allocation" end
val _ = allocate 50000
val _ = case !saved of
    SOME (f, xs, text) => print (Int.toString (f 4 + f 5) ^ ":" ^ text ^ ":" ^ Int.toString (length xs) ^ "\n")
  | NONE => raise Fail "saved"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 19:retained:3 *)
