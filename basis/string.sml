structure String =
struct
  val size = size

  val op ^ = op ^

  val str = Prim.charToString

  fun sub (s, i) =
    if i < 0 orelse i >= size s then raise Subscript else Prim.stringSub (s, i)

  fun explode s =
    let
      fun go (i, acc) =
        if i < 0 then acc else go (i - 1, Prim.stringSub (s, i) :: acc)
    in
      go (size s - 1, [])
    end

  fun concat [] = ""
    | concat [s] = s
    | concat ss =
        let
          fun pairs (a :: b :: rest) = (a ^ b) :: pairs rest
            | pairs rest = rest
        in
          concat (pairs ss)
        end

  fun implode cs =
    let
      fun strings [] = []
        | strings (c :: rest) = str c :: strings rest
    in
      concat (strings cs)
    end

  fun concatWith sep ss =
    let
      fun join [] = []
        | join [s] = [s]
        | join (s :: rest) = s :: sep :: join rest
    in
      concat (join ss)
    end
end

val str = String.str
val explode = String.explode
val implode = String.implode
val concat = String.concat
