exception Chr

structure Char =
struct
  val minChar = Prim.chr 0
  val maxChar = Prim.chr 255
  val maxOrd = 255

  val ord = Prim.ord

  fun chr i =
    if i < 0 orelse i > maxOrd then raise Chr else Prim.chr i

  fun succ c = chr (ord c + 1)
  fun pred c = chr (ord c - 1)

  fun compare (a, b) =
    if ord a < ord b then LESS else if ord a > ord b then GREATER else EQUAL

  fun op < (a, b) = ord a < ord b
  fun op <= (a, b) = ord a <= ord b
  fun op > (a, b) = ord a > ord b
  fun op >= (a, b) = ord a >= ord b

  fun contains s c =
    let
      fun loop i =
        i < size s andalso (Prim.stringSub (s, i) = c orelse loop (i + 1))
    in
      loop 0
    end

  fun notContains s c = not (contains s c)
  fun isAscii c = ord c < 128
  fun isLower c = ord c >= 97 andalso ord c <= 122
  fun isUpper c = ord c >= 65 andalso ord c <= 90
  fun isAlpha c = isLower c orelse isUpper c
  fun isDigit c = ord c >= 48 andalso ord c <= 57
  fun isAlphaNum c = isAlpha c orelse isDigit c
  fun isCntrl c = ord c < 32 orelse ord c = 127
  fun isSpace c =
    let val i = ord c
    in i = 32 orelse (i >= 9 andalso i <= 13)
    end
  fun isPrint c = not (isCntrl c)
  fun isGraph c = isPrint c andalso not (isSpace c)
  fun isPunct c = isGraph c andalso not (isAlphaNum c)
  fun toLower c =
    if isUpper c then chr (ord c + 32) else c

  fun toUpper c =
    if isLower c then chr (ord c - 32) else c

  fun isHexDigit c = isDigit c orelse (ord (toLower c) >= 97 andalso ord (toLower c) <= 102)

  fun toString c =
    let
      val i = ord c
      val raw = Prim.charToString c
      fun pad n = if size n < 3 then pad ("0" ^ n) else n
      fun decimal n =
        if n < 10 then Prim.charToString (chr (n + 48))
        else decimal (n div 10) ^ Prim.charToString (chr (n mod 10 + 48))
    in
      case i of
        7 => "\\a"
      | 8 => "\\b"
      | 9 => "\\t"
      | 10 => "\\n"
      | 11 => "\\v"
      | 12 => "\\f"
      | 13 => "\\r"
      | 92 => "\\\\"
      | 34 => "\\\""
      | _ =>
          if i < 32 then "\\^" ^ Prim.charToString (chr (i + 64))
          else if i <= 126 then raw
          else "\\" ^ pad (decimal i)
    end

  fun scan getc stream =
    let
      fun value n = if n < 0 orelse n > maxOrd then NONE else SOME (chr n)
      fun hex c =
        let val n = ord c
        in
          if n >= 48 andalso n <= 57 then n - 48
          else if n >= 65 andalso n <= 70 then n - 55
          else if n >= 97 andalso n <= 102 then n - 87
          else ~1
        end
      fun fixed base convert count state =
        let
          fun loop (0, current, acc) = SOME (acc, current)
            | loop (left, current, acc) =
                (case getc current of
                   SOME (c, rest) =>
                     let val d = convert c
                     in
                       if d < 0 then NONE
                       else loop (left - 1, rest, acc * base + d)
                     end
                 | NONE => NONE)
        in loop (count, state, 0) end
      fun escaped state =
        case getc state of
          NONE => NONE
        | SOME (c, rest) =>
            let
              fun withValue n after =
                case value n of SOME result => SOME (result, after) | NONE => NONE
              fun formatted current =
                (case getc current of
                   SOME (next, after) =>
                     if isSpace next then formatted after
                     else if next = #"\\" then scan getc after
                     else NONE
                 | NONE => NONE)
              fun control current =
                (case getc current of
                   SOME (next, after) =>
                     let val n = ord next
                     in if n >= 64 andalso n <= 95 then withValue (n - 64) after
                        else NONE
                     end
                 | NONE => NONE)
              fun decimal current =
                (case fixed 10 (fn d => if isDigit d then ord d - 48 else ~1) 3 current of
                   SOME (n, after) => withValue n after
                 | NONE => NONE)
              fun unicode current =
                (case fixed 16 hex 4 current of
                   SOME (n, after) => withValue n after
                 | NONE => NONE)
            in
              case c of
                #"a" => withValue 7 rest
              | #"b" => withValue 8 rest
              | #"t" => withValue 9 rest
              | #"n" => withValue 10 rest
              | #"v" => withValue 11 rest
              | #"f" => withValue 12 rest
              | #"r" => withValue 13 rest
              | #"\\" => SOME (#"\\", rest)
              | #"\"" => SOME (#"\"", rest)
              | #"^" => control rest
              | #"u" => unicode rest
              | _ =>
                  if isSpace c then formatted rest
                  else if isDigit c then
                    (case fixed 10 (fn d => if isDigit d then ord d - 48 else ~1) 2 rest of
                       SOME (tail, after) =>
                         let val n = (ord c - 48) * 100 + tail
                         in withValue n after end
                     | NONE => NONE)
                  else NONE
            end
    in
      case getc stream of
        NONE => NONE
      | SOME (#"\\", rest) => escaped rest
      | SOME (c, rest) =>
          let val n = ord c
          in if n >= 32 andalso n <= 126 then SOME (c, rest) else NONE end
    end

  fun fromString s =
    let
      fun getc i =
        if i < size s then SOME (Prim.stringSub (s, i), i + 1) else NONE
    in
      case scan getc 0 of NONE => NONE | SOME (c, _) => SOME c
    end

  fun toCString c =
    let
      val n = ord c
      fun octal x =
        Prim.charToString (chr (x div 64 + 48)) ^
        Prim.charToString (chr ((x div 8) mod 8 + 48)) ^
        Prim.charToString (chr (x mod 8 + 48))
      val raw = Prim.charToString c
    in
      case n of
        7 => "\\a"
      | 8 => "\\b"
      | 9 => "\\t"
      | 10 => "\\n"
      | 11 => "\\v"
      | 12 => "\\f"
      | 13 => "\\r"
      | 92 => "\\\\"
      | 34 => "\\\""
      | 63 => "\\?"
      | 39 => "\\'"
      | _ => if n >= 32 andalso n <= 126 then raw else "\\" ^ octal n
    end

  fun fromCString s =
    let
      fun getc i =
        if i < size s then SOME (Prim.stringSub (s, i), i + 1) else NONE
      fun value n = if n < 0 orelse n > maxOrd then NONE else SOME (chr n)
      fun hex c =
        let val n = ord c
        in
          if n >= 48 andalso n <= 57 then n - 48
          else if n >= 65 andalso n <= 70 then n - 55
          else if n >= 97 andalso n <= 102 then n - 87
          else ~1
        end
      fun escaped state =
        case getc state of
          NONE => NONE
        | SOME (c, rest) =>
            let
              fun octal (current, acc, count) =
                if count = 3 then value acc
                else
                  (case getc current of
                     SOME (d, after) =>
                       let val n = ord d - 48
                       in
                         if n >= 0 andalso n <= 7 then octal (after, acc * 8 + n, count + 1)
                         else if count = 0 then NONE else value acc
                       end
                   | NONE => if count = 0 then NONE else value acc)
              fun hexadecimal (current, acc, count) =
                (case getc current of
                   SOME (d, after) =>
                     let val n = hex d
                     in
                       if n < 0 then
                         if count = 0 then NONE else value acc
                       else hexadecimal
                         (after, if acc * 16 + n > maxOrd then maxOrd + 1 else acc * 16 + n,
                          count + 1)
                     end
                 | NONE => if count = 0 then NONE else value acc)
            in
              case c of
                #"a" => SOME (chr 7)
              | #"b" => SOME (chr 8)
              | #"t" => SOME (chr 9)
              | #"n" => SOME (chr 10)
              | #"v" => SOME (chr 11)
              | #"f" => SOME (chr 12)
              | #"r" => SOME (chr 13)
              | #"?" => SOME #"?"
              | #"\\" => SOME #"\\"
              | #"\"" => SOME #"\""
              | #"'" => SOME #"'"
              | #"^" =>
                  (case getc rest of
                     SOME (d, _) =>
                       let val n = ord d
                       in if n >= 64 andalso n <= 95 then value (n - 64) else NONE end
                   | NONE => NONE)
              | #"x" => hexadecimal (rest, 0, 0)
              | _ =>
                  let val n = ord c - 48
                  in if n >= 0 andalso n <= 7 then octal (rest, n, 1) else NONE end
            end
      fun parse state =
        case getc state of
          NONE => NONE
        | SOME (#"\\", rest) => escaped rest
        | SOME (#"\"", _) => NONE
        | SOME (c, _) =>
            let val n = ord c
            in if n >= 32 andalso n <= 126 then SOME c else NONE end
    in
      parse 0
    end
end

val ord = Char.ord
val chr = Char.chr
