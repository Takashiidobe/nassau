signature STRING =
sig
  eqtype string
  eqtype char
  val maxSize : int
  val size : string -> int
  val sub : string * int -> char
  val extract : string * int * int option -> string
  val substring : string * int * int -> string
  val op ^ : string * string -> string
  val concat : string list -> string
  val concatWith : string -> string list -> string
  val str : char -> string
  val implode : char list -> string
  val explode : string -> char list
  val map : (char -> char) -> string -> string
  val translate : (char -> string) -> string -> string
  val tokens : (char -> bool) -> string -> string list
  val fields : (char -> bool) -> string -> string list
  val isPrefix : string -> string -> bool
  val isSubstring : string -> string -> bool
  val isSuffix : string -> string -> bool
  val compare : string * string -> order
  val collate : (char * char -> order) -> string * string -> order
  val op < : string * string -> bool
  val op <= : string * string -> bool
  val op > : string * string -> bool
  val op >= : string * string -> bool
  val toString : string -> string
  val scan : (char, 'a) StringCvt.reader -> (string, 'a) StringCvt.reader
  val fromString : string -> string option
  val toCString : string -> string
  val fromCString : string -> string option
end

structure String :> STRING where type string = string where type char = char =
struct
  type string = string
  type char = char

  val maxSize = 1073741823
  val size = size

  fun reverse xs =
    let fun loop ([], acc) = acc
          | loop (x :: rest, acc) = loop (rest, x :: acc)
    in loop (xs, []) end

  fun joinStrings [] = ""
    | joinStrings [s] = s
    | joinStrings (s :: rest) = s ^ joinStrings rest

  val op^ = op^

  val str = Prim.charToString

  fun sub (s, i) =
    if i < 0 orelse i >= size s then raise Subscript else Prim.stringSub (s, i)

  fun extract (s, i, len) =
    if i < 0 orelse i > size s then raise Subscript
    else
      let
        val n = case len of NONE => size s - i | SOME n => n
        fun loop (j, acc) =
          if j >= n then joinStrings (reverse acc)
          else loop (j + 1, Prim.charToString (sub (s, i + j)) :: acc)
      in
        if n < 0 orelse n > size s - i then raise Subscript
        else loop (0, [])
      end

  fun substring (s, i, n) = extract (s, i, SOME n)

  fun explode s =
    let
      fun go (i, acc) =
        if i < 0 then acc else go (i - 1, Prim.stringSub (s, i) :: acc)
    in
      go (size s - 1, [])
    end

  fun concat ss =
    let
      fun checkSize ([], total) = total
        | checkSize (s :: rest, total) =
            if size s > maxSize - total then raise Size
            else checkSize (rest, total + size s)
      val _ = checkSize (ss, 0)
      fun combine [] = ""
        | combine [s] = s
        | combine strings =
        let
          fun pairs (a :: b :: rest) =
                (a ^ b) :: pairs rest
            | pairs rest = rest
        in
          combine (pairs strings)
        end
    in combine ss end

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
        | join (s :: rest) =
            s :: sep :: join rest
    in
      concat (join ss)
    end

  fun map f s =
    let fun loop (i, acc) =
          if i = size s then concat (reverse acc)
          else loop (i + 1, str (f (sub (s, i))) :: acc)
    in loop (0, []) end

  fun translate f s =
    let fun loop (i, acc) =
          if i = size s then concat (reverse acc)
          else loop (i + 1, f (sub (s, i)) :: acc)
    in loop (0, []) end

  fun split pred s keepEmpty =
    let
      fun finish (chars, acc) =
        let val part = implode (reverse chars)
        in if keepEmpty orelse size part > 0 then part :: acc else acc end
      fun loop (i, chars, acc) =
        if i = size s then reverse (finish (chars, acc))
        else
          let val c = sub (s, i)
          in if pred c then loop (i + 1, [], finish (chars, acc))
             else loop (i + 1, c :: chars, acc)
          end
    in loop (0, [], []) end

  fun tokens pred s = split pred s false
  fun fields pred s = split pred s true

  fun isPrefix prefix s =
    let val n = size prefix
        fun loop i = i = n orelse (sub (prefix, i) = sub (s, i) andalso loop (i + 1))
    in n <= size s andalso loop 0 end

  fun isSubstring pattern s =
    let
      val n = size pattern
      val limit = size s - n
      fun sameAt (i, j) =
        j = n orelse (sub (pattern, j) = sub (s, i + j) andalso sameAt (i, j + 1))
      fun loop i = i <= limit andalso (sameAt (i, 0) orelse loop (i + 1))
    in n = 0 orelse loop 0 end

  fun isSuffix suffix s =
    let val n = size suffix
        val offset = size s - n
        fun loop i = i = n orelse
          (sub (suffix, i) = sub (s, offset + i) andalso loop (i + 1))
    in offset >= 0 andalso loop 0 end

  fun collate cmp (s, t) =
    let
      val limit = if size s < size t then size s else size t
      fun loop i =
        if i = limit then
          if size s < size t then LESS else if size s > size t then GREATER else EQUAL
        else
          case cmp (sub (s, i), sub (t, i)) of
            EQUAL => loop (i + 1)
          | order => order
    in loop 0 end

  fun compare pair = collate Char.compare pair
  fun op < (s, t) = compare (s, t) = LESS
  fun op <= (s, t) = compare (s, t) <> GREATER
  fun op > (s, t) = compare (s, t) = GREATER
  fun op >= (s, t) = compare (s, t) <> LESS

  fun toString s = translate Char.toString s

  fun hexValue c =
    let val n = Char.ord (Char.toLower c)
    in if n >= 48 andalso n <= 57 then SOME (n - 48)
       else if n >= 97 andalso n <= 102 then SOME (n - 87)
       else NONE
    end

  fun decimalValue c =
    let val n = Char.ord c
    in if n >= 48 andalso n <= 57 then SOME (n - 48) else NONE end

  fun whitespace c =
    let val n = Char.ord c
    in n = 32 orelse (n >= 9 andalso n <= 13) end

  fun decodeEscape cMode getc source =
    let
      fun fixedDigits value left state =
        if left = 0 then SOME (value, state)
        else
          case getc state of
            SOME (c, rest) =>
              (case (if cMode then hexValue c else decimalValue c) of
                 NONE => NONE
               | SOME d => fixedDigits (value * (if cMode then 16 else 10) + d) (left - 1) rest)
          | NONE => NONE
      fun anyHex value state seen =
        case getc state of
          SOME (c, rest) =>
            (case hexValue c of
               NONE => if seen then SOME (value, state) else NONE
             | SOME d => anyHex (if value > 255 then 256 else value * 16 + d) rest true)
        | NONE => if seen then SOME (value, state) else NONE
      fun anyOct value state count =
        case getc state of
          SOME (c, rest) =>
            let val n = Char.ord c - 48
            in if n >= 0 andalso n <= 7 andalso count < 3
               then anyOct (value * 8 + n) rest (count + 1)
               else SOME (value, state)
            end
        | NONE => SOME (value, state)
      fun charCode n rest =
        if n <= Char.maxOrd then SOME (SOME (Char.chr n), rest) else NONE
      fun gap state seen =
        case getc state of
          SOME (c, rest) =>
            if whitespace c then gap rest true
            else if seen andalso c = #"\\" then SOME (NONE, rest)
            else NONE
        | NONE => NONE
      fun simple c rest =
        case c of
          #"a" => charCode 7 rest
        | #"b" => charCode 8 rest
        | #"t" => charCode 9 rest
        | #"n" => charCode 10 rest
        | #"v" => charCode 11 rest
        | #"f" => charCode 12 rest
        | #"r" => charCode 13 rest
        | #"\\" => SOME (SOME #"\\", rest)
        | #"\"" => SOME (SOME #"\"", rest)
        | #"'" => if cMode then SOME (SOME #"'", rest) else NONE
        | #"?" => if cMode then SOME (SOME #"?", rest) else NONE
        | _ => NONE
    in
      case getc source of
        NONE => NONE
      | SOME (c, rest) =>
          if c = #"^" then
            (case getc rest of
               SOME (d, after) =>
                 let val n = Char.ord d
                 in if n >= 64 andalso n <= 95 then charCode (n - 64) after else NONE end
             | NONE => NONE)
          else if not cMode andalso whitespace c then gap rest true
          else if cMode andalso c = #"x" then
            (case anyHex 0 rest false of
               SOME (n, after) => charCode n after
             | NONE => NONE)
          else if cMode andalso Char.ord c >= 48 andalso Char.ord c <= 55 then
            (case anyOct (Char.ord c - 48) rest 1 of
               SOME (n, after) => charCode n after
             | NONE => NONE)
          else if not cMode andalso c = #"u" then
            (case fixedDigits 0 4 rest of
               SOME (n, after) => charCode n after
             | NONE => NONE)
          else if not cMode andalso Option.isSome (decimalValue c) then
            (case decimalValue c of
               SOME d =>
                 (case fixedDigits d 2 rest of
                    SOME (n, after) => charCode n after
                  | NONE => NONE)
             | NONE => NONE)
          else simple c rest
    end

  fun scanMode cMode getc source =
    let
      fun done (chars, state) = SOME (implode (reverse chars), state)
      fun loop (state, chars, consumed) =
        case getc state of
          NONE => done (chars, state)
        | SOME (c, rest) =>
            if not (Char.isPrint c) orelse (cMode andalso c = #"\"") then
              if consumed then done (chars, state) else NONE
            else if c <> #"\\" then loop (rest, c :: chars, true)
            else
              (case decodeEscape cMode getc rest of
                 NONE => if consumed then done (chars, state) else NONE
               | SOME (NONE, next) => loop (next, chars, true)
               | SOME (SOME decoded, next) => loop (next, decoded :: chars, true))
    in loop (source, [], false) end

  fun scan getc source = scanMode false getc source
  fun fromString s = StringCvt.scanString scan s

  fun cCharString c =
    let val n = Char.ord c
        fun octal n =
          String.str (Char.chr (48 + n div 64)) ^
          String.str (Char.chr (48 + (n div 8) mod 8)) ^
          String.str (Char.chr (48 + n mod 8))
    in
      case n of
        7 => "\\a"
      | 8 => "\\b"
      | 9 => "\\t"
      | 10 => "\\n"
      | 11 => "\\v"
      | 12 => "\\f"
      | 13 => "\\r"
      | 34 => "\\\""
      | 39 => "\\'"
      | 63 => "\\?"
      | 92 => "\\\\"
      | _ => if n >= 32 andalso n <= 126 then String.str c else "\\" ^ octal n
    end

  fun toCString s = translate cCharString s
  fun fromCString s = StringCvt.scanString (scanMode true) s
end

val str = String.str
val explode = String.explode
val implode = String.implode
val concat = String.concat
