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
  val sqrt : real -> real = Prim.mathSqrt
  val sin : real -> real = Prim.mathSin
  val cos : real -> real = Prim.mathCos
  val tan : real -> real = Prim.mathTan
  val asin : real -> real = Prim.mathAsin
  val acos : real -> real = Prim.mathAcos
  val atan : real -> real = Prim.mathAtan
  val atan2 : real * real -> real = Prim.mathAtan2
  val exp : real -> real = Prim.mathExp
  val pow : real * real -> real = Prim.mathPow
  val ln : real -> real = Prim.mathLn
  val log10 : real -> real = Prim.mathLog10
  val sinh : real -> real = Prim.mathSinh
  val cosh : real -> real = Prim.mathCosh
  val tanh : real -> real = Prim.mathTanh
end
