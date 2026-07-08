val _ = print "\a\b\t\n\v\f\r\\\"\^A\065"
val _ = Posix.Process.exit (Word8.fromInt 0)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: {{[[:cntrl:] ]}}{{[[:cntrl:] ]}}{{[[:cntrl:] ]}} *)
(* CHECK-STDOUT-NEXT: {{[[:cntrl:] ]}}{{[[:cntrl:] ]}}{{[[:cntrl:] ]}}\"{{[[:cntrl:] ]}}A *)
