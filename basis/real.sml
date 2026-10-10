structure Real =
struct
  type real = real

  val radix = 2
  val precision = 53
  val maxFinite = 1.7976931348623157E308
  val minPos = 4.9406564584124654E~324
  val minNormalPos = 2.2250738585072014E~308
  val posInf = 1.0 / 0.0
  val negInf = ~posInf

  val op + = op +
  val op - = op -
  val op * = op *
  val op / = op /
  val op ~ = op ~
  val op < = op <
  val op <= = op <=
  val op > = op >
  val op >= = op >=

  fun abs r =
    if r < 0.0 then ~r else if r = 0.0 then 0.0 else r

  fun isNan r = not (r <= r)

  fun isFinite r = r <= maxFinite andalso r >= ~maxFinite

  fun isNormal r = isFinite r andalso abs r >= minNormalPos

  fun class r =
    if isNan r then IEEEReal.NAN
    else if not (isFinite r) then IEEEReal.INF
    else if r = 0.0 then IEEEReal.ZERO
    else if abs r < minNormalPos then IEEEReal.SUBNORMAL
    else IEEEReal.NORMAL

  fun unordered (x, y) = isNan x orelse isNan y

  fun compareReal (x, y) =
    if unordered (x, y) then IEEEReal.UNORDERED
    else if x < y then IEEEReal.LESS
    else if x > y then IEEEReal.GREATER
    else IEEEReal.EQUAL

  fun compare (x, y) =
    case compareReal (x, y) of
      IEEEReal.LESS => LESS
    | IEEEReal.EQUAL => EQUAL
    | IEEEReal.GREATER => GREATER
    | IEEEReal.UNORDERED => raise IEEEReal.Unordered

  fun op == (x, y) = not (unordered (x, y)) andalso x = y

  fun op != (x, y) = not (op == (x, y))

  fun op ?= (x, y) = unordered (x, y) orelse x = y

  fun min (x, y) =
    if isNan x then if isNan y then x else y
    else if isNan y then x
    else if x <= y then x else y

  fun max (x, y) =
    if isNan x then if isNan y then x else y
    else if isNan y then x
    else if x >= y then x else y

  fun sign r =
    if isNan r then raise Domain
    else if r < 0.0 then ~1
    else if r > 0.0 then 1
    else 0

  val fromInt = Prim.intToReal

  fun floor r =
    if r >= ~1073741824.0 andalso r < 1073741824.0 then Prim.realFloor r
    else raise Overflow
end

structure LargeReal = Real

val real = Real.fromInt
val floor = Real.floor
