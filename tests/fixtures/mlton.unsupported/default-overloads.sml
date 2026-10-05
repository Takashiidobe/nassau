(* mlton regression/default-overloads.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f x = ~ x;
fun g (x: int) = f x;
fun f x = abs x;
fun g (x: int) = f x;
fun f x = x + x;
fun g (x: int) = f x;
fun f x = x - x;
fun g (x: int) = f x;
fun f x = x * x;
fun g (x: int) = f x;
fun f x = x div x;
fun g (x: int) = f x;
fun f x = x mod x;
fun g (x: int) = f x;
fun f x = x < x;
fun g (x: int) = f x;
fun f x = x <= x;
fun g (x: int) = f x;
fun f x = x > x;
fun g (x: int) = f x;
fun f x = x >= x;
fun g (x: int) = f x;
fun f x = x / x;
fun g (x: real) = f x;
(* CHECK-EXIT: 0 *)
