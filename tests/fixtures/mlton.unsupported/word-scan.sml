(* mlton regression/word-scan.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun testScan cvt s =
   (print o concat)
   ["Word.scan ",
    case cvt of
       StringCvt.BIN => "StringCvt.BIN"
     | StringCvt.OCT => "StringCvt.OCT"
     | StringCvt.DEC => "StringCvt.DEC"
     | StringCvt.HEX => "StringCvt.HEX",
    " \"",
    s,
    "\" = ",
    case Word.scan cvt Substring.getc (Substring.full s) of
       NONE => "NONE"
     | SOME (result, rest) =>
          concat ["SOME (0w", Word.fmt StringCvt.DEC result,
                  ", \"", Substring.string rest, "\")"],
    "\n"]

val () = List.app (testScan StringCvt.BIN)
                  ["0", "0w", "0wx", "0x",
                   "01", "0w1", "0wx1", "0x1",
                   "0z", "0wz", "0wxz", "0xz",
                   "01z", "0w1z", "0wx1z", "0x1z"];
val () = List.app (testScan StringCvt.OCT)
                  ["0", "0w", "0wx", "0x",
                   "01", "0w1", "0wx1", "0x1",
                   "0z", "0wz", "0wxz", "0xz",
                   "01z", "0w1z", "0wx1z", "0x1z"];
val () = List.app (testScan StringCvt.DEC)
                  ["0", "0w", "0wx", "0x",
                   "01", "0w1", "0wx1", "0x1",
                   "0z", "0wz", "0wxz", "0xz",
                   "01z", "0w1z", "0wx1z", "0x1z"];
val () = List.app (testScan StringCvt.HEX)
                  ["0", "0w", "0wx", "0x",
                   "01", "0w1", "0wx1", "0x1",
                   "0z", "0wz", "0wxz", "0xz",
                   "01z", "0w1z", "0wx1z", "0x1z"];
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: Word.scan StringCvt.BIN "0" = SOME (0w0, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0w" = SOME (0w0, "w") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0wx" = SOME (0w0, "wx") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0x" = SOME (0w0, "x") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "01" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0w1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0wx1" = SOME (0w0, "wx1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0x1" = SOME (0w0, "x1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0z" = SOME (0w0, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0wz" = SOME (0w0, "wz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0wxz" = SOME (0w0, "wxz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0xz" = SOME (0w0, "xz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "01z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0w1z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0wx1z" = SOME (0w0, "wx1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.BIN "0x1z" = SOME (0w0, "x1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0" = SOME (0w0, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0w" = SOME (0w0, "w") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0wx" = SOME (0w0, "wx") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0x" = SOME (0w0, "x") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "01" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0w1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0wx1" = SOME (0w0, "wx1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0x1" = SOME (0w0, "x1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0z" = SOME (0w0, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0wz" = SOME (0w0, "wz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0wxz" = SOME (0w0, "wxz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0xz" = SOME (0w0, "xz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "01z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0w1z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0wx1z" = SOME (0w0, "wx1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.OCT "0x1z" = SOME (0w0, "x1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0" = SOME (0w0, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0w" = SOME (0w0, "w") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0wx" = SOME (0w0, "wx") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0x" = SOME (0w0, "x") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "01" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0w1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0wx1" = SOME (0w0, "wx1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0x1" = SOME (0w0, "x1") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0z" = SOME (0w0, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0wz" = SOME (0w0, "wz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0wxz" = SOME (0w0, "wxz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0xz" = SOME (0w0, "xz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "01z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0w1z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0wx1z" = SOME (0w0, "wx1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.DEC "0x1z" = SOME (0w0, "x1z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0" = SOME (0w0, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0w" = SOME (0w0, "w") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0wx" = SOME (0w0, "wx") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0x" = SOME (0w0, "x") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "01" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0w1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0wx1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0x1" = SOME (0w1, "") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0z" = SOME (0w0, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0wz" = SOME (0w0, "wz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0wxz" = SOME (0w0, "wxz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0xz" = SOME (0w0, "xz") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "01z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0w1z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0wx1z" = SOME (0w1, "z") *)
(* CHECK-STDOUT-NEXT: Word.scan StringCvt.HEX "0x1z" = SOME (0w1, "z") *)
