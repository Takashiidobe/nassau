structure General =
struct
  fun ignore _ = ()

  fun op o (f, g) =
    fn x => f (g x)

  fun op before (x, ()) = x
end

val ignore = General.ignore
val op o = General.o
val op before = General.before
