(* mlton regression/char.scan.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val dquote = "\""
   
val _ = print (concat [Bool.toString (isSome (Char.fromString dquote)), "\n"])

val scan: string -> unit =
   fn s =>
   let
      val n = String.size s
      fun reader i =
         if i = n
            then NONE
         else SOME (String.sub (s, i), i + 1)
   in
      case Char.scan reader 0 of
         NONE => print "NONE\n"
       | SOME (c, i) => print (concat ["#\"", Char.toString c, "\" at ", Int.toString i,
                                       " of ", Int.toString n, "\n"])
   end

val _ =
   List.app scan ["\\ \\",
                  "a\\ \\", "\\ \\a", "\\ \\a\\ \\", "\\ \\\\ \\a",
                  "\\n\\ \\", "\\ \\\\n", "\\ \\\\n\\ \\", "\\ \\\\ \\\\n"]
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: true *)
(* CHECK-STDOUT-NEXT: NONE *)
(* CHECK-STDOUT-NEXT: #"a" at 1 of 4 *)
(* CHECK-STDOUT-NEXT: #"a" at 4 of 4 *)
(* CHECK-STDOUT-NEXT: #"a" at 4 of 7 *)
(* CHECK-STDOUT-NEXT: #"a" at 7 of 7 *)
(* CHECK-STDOUT-NEXT: #"\n" at 2 of 5 *)
(* CHECK-STDOUT-NEXT: #"\n" at 5 of 5 *)
(* CHECK-STDOUT-NEXT: #"\n" at 5 of 8 *)
(* CHECK-STDOUT-NEXT: #"\n" at 8 of 8 *)
