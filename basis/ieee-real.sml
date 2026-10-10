structure IEEEReal =
struct
  exception Unordered

  datatype real_order = LESS | EQUAL | GREATER | UNORDERED
  datatype float_class = NAN | INF | ZERO | NORMAL | SUBNORMAL
  datatype rounding_mode = TO_NEAREST | TO_NEGINF | TO_POSINF | TO_ZERO

  type decimal_approx = {
    class : float_class,
    sign : bool,
    digits : int list,
    exp : int
  }

  fun toString {class, sign, digits, exp} =
    let
      fun digitString [] = ""
        | digitString (d :: ds) = Int.toString d ^ digitString ds
      val body =
        case class of
          ZERO => "0.0"
        | NORMAL => "0." ^ digitString digits
        | SUBNORMAL => "0." ^ digitString digits
        | INF => "inf"
        | NAN => "nan"
      val signed = if sign then "~" ^ body else body
    in
      case class of
        NORMAL => if exp = 0 then signed else signed ^ "E" ^ Int.toString exp
      | SUBNORMAL => if exp = 0 then signed else signed ^ "E" ^ Int.toString exp
      | _ => signed
    end

  fun scan getc stream =
    let
      fun rev xs =
        let
          fun loop ([], acc) = acc
            | loop (x :: rest, acc) = loop (rest, x :: acc)
        in
          loop (xs, [])
        end
      fun whitespace c =
        let val i = Char.ord c
        in i = 32 orelse (i >= 9 andalso i <= 13)
        end
      fun digit c = Char.ord c >= 48 andalso Char.ord c <= 57
      fun digitValue c = Char.ord c - 48
      fun lower c = Char.toLower c
      fun skip state =
        (case getc state of
           SOME (c, rest) => if whitespace c then skip rest else state
         | NONE => state)
      fun takeWhile pred state =
        let
          fun loop (current, acc) =
            (case getc current of
               SOME (c, rest) =>
                 if pred c then loop (rest, c :: acc)
                 else (rev acc, current)
             | NONE => (rev acc, current))
        in
          loop (state, [])
        end
      fun matchWord word state =
        let
          fun loop ([], current) = SOME current
            | loop (expected :: rest, current) =
                (case getc current of
                   SOME (actual, next) =>
                     if lower actual = expected then loop (rest, next) else NONE
                 | NONE => NONE)
        in
          loop (String.explode word, state)
        end
      fun stripLeading [] = ([], 0)
        | stripLeading (0 :: ds) =
            let val (rest, count) = stripLeading ds
            in (rest, count + 1)
            end
        | stripLeading ds = (ds, 0)
      fun stripTrailing ds =
        let
          fun drop (0 :: rest) = drop rest
            | drop rest = rest
        in
          rev (drop (rev ds))
        end
      fun parseExponent original =
        (case getc original of
           SOME (marker, afterMarker) =>
             if marker = #"e" orelse marker = #"E" then
               let
                 val (negative, startDigits) =
                   (case getc afterMarker of
                      SOME (#"~", rest) => (true, rest)
                    | SOME (#"-", rest) => (true, rest)
                    | SOME (#"+", rest) => (false, rest)
                    | _ => (false, afterMarker))
                 val (chars, finalState) = takeWhile digit startDigits
               in
                 if chars = [] then (0, original)
                 else
                   let
                     val magnitude =
                       List.foldl
                         (fn (c, n) => n * 10 + digitValue c)
                         0 chars
                   in
                     (if negative then ~magnitude else magnitude, finalState)
                   end
               end
             else (0, original)
         | NONE => (0, original))
      fun makeNumber negative state =
        let
          val (wholeChars, afterWhole) = takeWhile digit state
          val afterPoint =
            (case getc afterWhole of
               SOME (#".", rest) => SOME rest
             | _ => NONE)
        in
          case afterPoint of
            NONE => NONE
          | SOME fractionalStart =>
              let
                val (fractionChars, afterFraction) = takeWhile digit fractionalStart
              in
                if wholeChars = [] andalso fractionChars = [] then NONE
                else
                  let
                    val (exponent, finalState) = parseExponent afterFraction
                    val wholeDigits = List.map digitValue wholeChars
                    val fractionDigits = List.map digitValue fractionChars
                    val (significantWhole, _) = stripLeading wholeDigits
                    val significantFraction = stripTrailing fractionDigits
                    val result =
                      if significantWhole <> [] then
                        {class = NORMAL, sign = negative,
                         digits = significantWhole @ significantFraction,
                         exp = List.length significantWhole + exponent}
                      else
                        let
                          val (significant, leadingZeros) = stripLeading significantFraction
                        in
                          if significant = [] then
                            {class = ZERO, sign = negative, digits = [], exp = 0}
                          else
                            {class = NORMAL, sign = negative, digits = significant,
                             exp = ~leadingZeros + exponent}
                        end
                  in
                    SOME (result, finalState)
                  end
              end
        end
      val start = skip stream
      val (negative, valueStart) =
        (case getc start of
           SOME (#"~", rest) => (true, rest)
         | SOME (#"-", rest) => (true, rest)
         | SOME (#"+", rest) => (false, rest)
         | _ => (false, start))
      fun nonfinite word class =
        case matchWord word valueStart of
          SOME rest => SOME ({class = class, sign = negative, digits = [], exp = 0}, rest)
        | NONE => NONE
    in
      case nonfinite "infinity" INF of
        SOME result => SOME result
      | NONE =>
          (case nonfinite "inf" INF of
             SOME result => SOME result
           | NONE =>
               (case nonfinite "nan" NAN of
                  SOME result => SOME result
                | NONE => makeNumber negative valueStart))
    end

  fun fromString s =
    let
      fun getc i =
        if i < size s then SOME (String.sub (s, i), i + 1) else NONE
    in
      case scan getc 0 of
        SOME (value, _) => SOME value
      | NONE => NONE
    end
end
