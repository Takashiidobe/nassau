(* mlton regression/fail/pat.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val {x = y, x = z} = {x = 13};
(* CHECK-ERR: × expected {x:'a, x:'b}, found {x:int} *)
(* CHECK-ERR: :2:22] *)
