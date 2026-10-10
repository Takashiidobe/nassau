signature MATH =
sig
  type real
  val pi : real
  val e : real
  val sqrt : real -> real
  val sin : real -> real
  val cos : real -> real
  val tan : real -> real
  val asin : real -> real
  val acos : real -> real
  val atan : real -> real
  val atan2 : real * real -> real
  val exp : real -> real
  val pow : real * real -> real
  val ln : real -> real
  val log10 : real -> real
  val sinh : real -> real
  val cosh : real -> real
  val tanh : real -> real
end

structure Math =
struct
  type real = Real.real
  val pi : real = 3.14159265358979323846
  val e : real = 2.7182818284590452354
end
