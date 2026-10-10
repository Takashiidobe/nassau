structure General =
struct
  type unit = unit
  type exn = exn
  datatype order = datatype order

  exception Bind = Bind
  exception Match = Match
  exception Chr = Chr
  exception Div = Div
  exception Domain = Domain
  exception Fail = Fail
  exception Overflow = Overflow
  exception Size = Size
  exception Span = Span
  exception Subscript = Subscript

  val op ! = op !
  val op := = op :=

  fun ignore _ = ()

  fun op o (f, g) =
    fn x => f (g x)

  fun op before (x, ()) = x
end

val ignore = General.ignore
val op o = General.o
val op before = General.before
