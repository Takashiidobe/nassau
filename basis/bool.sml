structure Bool =
struct
  val not = not

  fun toString true = "true"
    | toString false = "false"
end
