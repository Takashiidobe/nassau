structure Counter :> sig type t val zero : t end = struct type t = int val zero = 0 end
val x = Counter.zero + 1
(* CHECK-ERR: × operator is not defined for type Counter.t *)
(* CHECK-ERR: :2:9] *)
