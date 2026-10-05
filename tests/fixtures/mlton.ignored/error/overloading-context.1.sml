(* mlton regression/fail/overloading-context.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
(* This must fail, because the overloading context can be no larger than the
 * smallest enclosing strdec.  So, the declaration of double must be resolved
 * (with type int -> int) before continuing.
 *)
structure S =
   struct
      fun double x = x + x
   end
val _ = S.double 2.0
