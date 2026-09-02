(* mlton regression/command-line.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* print out the command name and all of the command line arguments on separate
 lines *)

val _ =
   (print(CommandLine.name()) ;
    print "\n" ;
    app (fn s => (print s ; print "\n")) (CommandLine.arguments()))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: poly *)
(* CHECK-STDOUT-NEXT: -q *)
(* CHECK-STDOUT-NEXT: --script *)
(* CHECK-STDOUT-NEXT: /home/takashi/Projects/nassau/tools/polyml_oracle.sml *)
