fun count 0 = 0
  | count n = 1 + count "n"
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :2:25] *)
