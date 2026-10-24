val path = "nassau-binio.tmp"
val out = BinIO.openOut path
val _ = BinIO.output (out, Word8Vector.fromList (map Word8.fromInt [0, 255, 10, 13]))
val _ = BinIO.output1 (out, Word8.fromInt 65)
val _ = BinIO.closeOut out
val app = BinIO.openAppend path
val _ = BinIO.output1 (app, Word8.fromInt 66)
val _ = BinIO.closeOut app
val inp = BinIO.openIn path
fun show v = print (String.concatWith "," (map Word8.toString (Word8Vector.foldr (op ::) [] v)) ^ "\n")
val _ = print (case BinIO.lookahead inp of SOME w => Word8.toString w ^ "\n" | NONE => "none\n")
val _ = show (BinIO.inputN (inp, 3))
val _ = print (case BinIO.input1 inp of SOME w => Word8.toString w ^ "\n" | NONE => "none\n")
val _ = print (case BinIO.canInput (inp, 10) of SOME n => Bool.toString (n > 0) ^ "\n" | NONE => "none\n")
val _ = show (BinIO.inputAll inp)
val _ = print (Bool.toString (BinIO.endOfStream inp) ^ "\n")
val _ = BinIO.closeIn inp
val _ = print ((BinIO.openIn "nassau-no-such-dir/x"; "opened\n") handle IO.Io {function, ...} => function ^ "\n")
val s = TextIO.openString "ab\ncd"
val _ = print (case TextIO.inputLine s of SOME l => String.toString l ^ "\n" | NONE => "none\n")
val _ = print (TextIO.inputAll s ^ "\n")
val _ = OS.FileSys.remove path
(* CHECK-STDOUT: 0 *)
(* CHECK-STDOUT-NEXT: 0,FF,A *)
(* CHECK-STDOUT-NEXT: D *)
(* CHECK-STDOUT-NEXT: true *)
(* CHECK-STDOUT-NEXT: 41,42 *)
(* CHECK-STDOUT-NEXT: true *)
(* CHECK-STDOUT-NEXT: BinIO.openIn *)
(* CHECK-STDOUT-NEXT: ab\n *)
(* CHECK-STDOUT-NEXT: cd *)
(* CHECK-EXIT: 0 *)
