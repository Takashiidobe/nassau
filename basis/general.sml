structure General =
struct
  type unit = unit
  type exn = exn
  datatype order = datatype order

  exception Bind = Bind
  exception Match = Match
  exception Chr
  exception Div = Div
  exception Domain
  exception Fail = Fail
  exception Overflow = Overflow
  exception Size = Size
  exception Span
  exception Subscript = Subscript

  val op ! = op !
  val op := = op :=

  fun ignore _ = ()

  fun exnName (e: exn) : string = Prim.exnName e

  local
  fun escapeString (s: string) : string =
    let
      fun digit n = Prim.charToString (Prim.chr (n + 48))
      fun escape c =
        let val n = Prim.ord c
        in
          if n = 34 then "\\\""
          else if n = 92 then "\\\\"
          else if n = 10 then "\\n"
          else if n = 9 then "\\t"
          else if n = 13 then "\\r"
          else if n = 8 then "\\b"
          else if n = 12 then "\\f"
          else if n < 32 then "\\^" ^ Prim.charToString (Prim.chr (n + 64))
          else if n > 126 then "\\" ^ digit (n div 100) ^ digit ((n div 10) mod 10) ^ digit (n mod 10)
          else Prim.charToString c
        end
      fun go (i, acc) = if i = size s then acc else go (i + 1, acc ^ escape (Prim.stringSub (s, i)))
    in
      go (0, "")
    end

  in
  fun exnMessage (e: exn) : string =
    case e of
      Fail s => "Fail \"" ^ escapeString s ^ "\""
    | _ => exnName e
  end

  fun op o (f, g) =
    fn x => f (g x)

  fun op before (x, ()) = x
end

exception Chr = General.Chr
exception Domain = General.Domain
exception Span = General.Span

val ignore = General.ignore
val exnName = General.exnName
val exnMessage = General.exnMessage
val op o = General.o
val op before = General.before
