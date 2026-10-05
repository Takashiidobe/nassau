(* mlton regression/fail/type-use-before-def.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val r = ref NONE
datatype t = T
val _ = r := SOME T
(* CHECK-ERR: × expected ?.X1 option ref * ?.X1 option, found ?.X1 option ref * t option *)
(* CHECK-ERR: :4:9] *)
