(* mlton regression/string.fromString.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun toString NONE = "NONE"
  | toString (SOME s) = concat ["SOME [", s, "]"]
fun check (s, s') =
    let val s'' = String.fromString s
    in if s'' = s'
          then print (concat ["OK ", toString s'', "\n"])
       else print (concat ["WRONG ", toString s'', " ", toString s', "\n"])
    end

val _ =
   List.app check
   [("abc\"def", SOME "abc"),
    ("\n", NONE),
    (* from SML Basis manual example *)
    ("\\q", NONE),
    ("a\^D", SOME "a"),
    ("a\\ \\\\q", SOME "a"),
    ("\\ \\", SOME ""),
    ("", SOME ""),
    ("\\ \\\^D", SOME ""),
    ("\\ a", NONE)]
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: WRONG SOME [abc"def] SOME [abc] *)
(* CHECK-STDOUT-NEXT: OK NONE *)
(* CHECK-STDOUT-NEXT: OK NONE *)
(* CHECK-STDOUT-NEXT: OK SOME [a] *)
(* CHECK-STDOUT-NEXT: OK SOME [a] *)
(* CHECK-STDOUT-NEXT: OK SOME [] *)
(* CHECK-STDOUT-NEXT: OK SOME [] *)
(* CHECK-STDOUT-NEXT: OK SOME [] *)
(* CHECK-STDOUT-NEXT: OK NONE *)
