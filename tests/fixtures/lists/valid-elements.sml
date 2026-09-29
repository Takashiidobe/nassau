(* Lists of every element type the backend supports, stored as cons cells. *)
val ints = [1, ~2, 1073741823]
val reals = [1.5, 2.5]
val bools = [true, false]
val lists = [[1], [], [2, 3]]
(* CHECK-EXIT: 0 *)
