(* SML'97 grammar, constant: string escapes. *)
val _ = print "a\tb\\c\"d\065\^A e\    \f\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: a{{[[:cntrl:] ]}}b\c"dA{{[[:cntrl:] ]}} ef *)
(* WASM-SKIP: FileCheck regex patterns are not supported by the wasm runner *)
(* WASM-SKIP: FileCheck regex patterns are not supported by the wasm runner *)
