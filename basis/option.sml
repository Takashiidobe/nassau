exception Option

structure Option =
struct
  exception Option = Option

  fun isSome NONE = false
    | isSome (SOME _) = true

  fun valOf NONE = raise Option
    | valOf (SOME x) = x

  fun getOpt (NONE, default) = default
    | getOpt (SOME x, _) = x

  fun filter p x = if p x then SOME x else NONE

  fun join NONE = NONE
    | join (SOME opt) = opt

  fun app f NONE = ()
    | app f (SOME x) = f x

  fun map f NONE = NONE
    | map f (SOME x) = SOME (f x)

  fun mapPartial f NONE = NONE
    | mapPartial f (SOME x) = f x

  fun compose (f, g) x =
    (case g x of
       NONE => NONE
     | SOME y => SOME (f y))

  fun composePartial (f, g) x =
    (case g x of
       NONE => NONE
     | SOME y => f y)
end

val isSome = Option.isSome
val valOf = Option.valOf
val getOpt = Option.getOpt
