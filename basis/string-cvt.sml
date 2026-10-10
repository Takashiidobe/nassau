signature STRING_CVT =
sig
  datatype radix = BIN | OCT | DEC | HEX
  datatype realfmt = SCI of int option | FIX of int option | GEN of int option | EXACT
  type ('a, 'b) reader = 'b -> ('a * 'b) option
  val padLeft : char -> int -> string -> string
  val padRight : char -> int -> string -> string
  val splitl : (char -> bool) -> (char, 'a) reader -> 'a -> string * 'a
  val takel : (char -> bool) -> (char, 'a) reader -> 'a -> string
  val dropl : (char -> bool) -> (char, 'a) reader -> 'a -> 'a
  val skipWS : (char, 'a) reader -> 'a -> 'a
  type cs
  val scanString : ((char, cs) reader -> ('a, cs) reader) -> string -> 'a option
end

structure StringCvt :> STRING_CVT =
struct
  datatype radix = BIN | OCT | DEC | HEX
  datatype realfmt = SCI of int option | FIX of int option | GEN of int option | EXACT
  type ('a, 'b) reader = 'b -> ('a * 'b) option
  type cs = string * int

  fun reverse xs =
    let fun loop ([], acc) = acc
          | loop (x :: rest, acc) = loop (rest, x :: acc)
    in loop (xs, []) end

  fun charsToString cs =
    let fun loop ([], acc) = acc
          | loop (c :: rest, acc) = loop (rest, acc ^ Prim.charToString c)
    in loop (cs, "") end

  fun repeat c n =
    if n <= 0 then "" else Prim.charToString c ^ repeat c (n - 1)

  fun padLeft c width s =
    if size s >= width then s
    else if width > 1073741823 then raise Size
    else repeat c (width - size s) ^ s

  fun padRight c width s =
    if size s >= width then s
    else if width > 1073741823 then raise Size
    else s ^ repeat c (width - size s)

  fun splitl pred getc source =
    let
      fun loop (state, revChars) =
        case getc state of
          SOME (c, rest) =>
            if pred c then loop (rest, c :: revChars)
            else (charsToString (reverse revChars), state)
        | NONE => (charsToString (reverse revChars), state)
    in loop (source, []) end

  fun takel pred getc source = #1 (splitl pred getc source)
  fun dropl pred getc source = #2 (splitl pred getc source)

  fun skipWS getc source =
    #2 (splitl (fn c =>
      let val n = Prim.ord c
      in n = 32 orelse (n >= 9 andalso n <= 13) end) getc source)

  fun scanString scan s =
    let
      fun getc (source, i) =
        if i >= size source then NONE
        else SOME (Prim.stringSub (source, i), (source, i + 1))
    in
      case scan getc (s, 0) of NONE => NONE | SOME (value, _) => SOME value
    end
end
