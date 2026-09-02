(* mlton regression/fail/pat.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val NONE _ = ()

val _ :: 13 = []

val 13 : bool = 14

(* Precedence parsing *)
val x _ _ = 13

val + _ = 13

val _ + = 13

val _ + _ = 13

local
   infixr 5 foo
   infix 5 bar
in
   val x foo y bar z = 13
end
   
val x _ = 13

val x : int as NONE = NONE

val [13, "foo"] = []

val {foo, ...} = raise Fail ""
(* CHECK-ERR: × expected = after val pattern *)
(* CHECK-ERR: :9:9] *)
