structure Bool =
struct
  val not = not

  fun toString true = "true"
    | toString false = "false"

  fun scan getc stream =
    let
      fun whitespace c =
        let val i = Prim.ord c
        in i = 32 orelse (i >= 9 andalso i <= 13)
        end
      fun lower c =
        let val i = Prim.ord c
        in if i >= 65 andalso i <= 90 then Prim.chr (i + 32) else c
        end
      fun skip state =
        (case getc state of
           SOME (c, rest) => if whitespace c then skip rest else state
         | NONE => state)
      fun match [] state = SOME state
        | match (c :: cs) state =
            (case getc state of
               SOME (actual, rest) =>
                 if lower actual = c then match cs rest else NONE
             | NONE => NONE)
      val start = skip stream
    in
      case match [#"t", #"r", #"u", #"e"] start of
        SOME rest => SOME (true, rest)
      | NONE =>
          (case match [#"f", #"a", #"l", #"s", #"e"] start of
             SOME rest => SOME (false, rest)
           | NONE => NONE)
    end

  fun fromString s =
    let
      fun getc i =
        if i < size s then SOME (Prim.stringSub (s, i), i + 1) else NONE
    in
      case scan getc 0 of
        SOME (value, _) => SOME value
      | NONE => NONE
    end
end
