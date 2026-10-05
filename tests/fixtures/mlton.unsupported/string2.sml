(* mlton regression/string2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ = print(concat[Char.toCString #"\000",
                     String.toCString "\000",
                     "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: \000\000 *)
