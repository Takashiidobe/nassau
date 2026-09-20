structure Real =
struct
  val fromInt = Prim.intToReal

  fun floor r =
    if r >= ~1073741824.0 andalso r < 1073741824.0 then Prim.realFloor r
    else raise Overflow
end

val real = Real.fromInt
val floor = Real.floor
