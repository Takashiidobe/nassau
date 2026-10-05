(* mlton regression/print-self.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val K = "\nval _ = print (concat [\"val K = \\\"\", String.translate (fn #\"\\n\" => \"\\\\n\" | #\"\\\\\" => \"\\\\\\\\\" | #\"\\\"\" => \"\\\\\\\"\" | c => str c) K, \"\\\"\", K, \"\\n\"])"
val _ = print (concat ["val K = \"", String.translate (fn #"\n" => "\\n" | #"\\" => "\\\\" | #"\"" => "\\\"" | c => str c) K, "\"", K, "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: val K = "\nval _ = print (concat [\"val K = \\\"\", String.translate (fn #\"\\n\" => \"\\\\n\" | #\"\\\\\" => \"\\\\\\\\\" | #\"\\\"\" => \"\\\\\\\"\" | c => str c) K, \"\\\"\", K, \"\\n\"])" *)
(* CHECK-STDOUT-NEXT: val _ = print (concat ["val K = \"", String.translate (fn #"\n" => "\\n" | #"\\" => "\\\\" | #"\"" => "\\\"" | c => str c) K, "\"", K, "\n"]) *)
