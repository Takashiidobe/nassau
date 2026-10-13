(* POLYML-SKIP: exercises Nassau's internal normal shutdown entry point *)
val path = "nassau-process-cleanup.tmp"
val output = TextIO.openOut path
val closed = TextIO.openAppend path
val _ = TextIO.closeOut closed
val _ = TextIO.closeOut closed
val input = TextIO.openIn path
val hookOutput : TextIO.outstream option ref = ref NONE
val _ = OS.Process.atExit (fn () =>
  (TextIO.output (output, "from hook\n");
   hookOutput := SOME (TextIO.openAppend path)))
val _ = OS.Process.runAtExit ()
fun isClosed stream =
  (TextIO.output (stream, "unexpected"); false)
  handle IO.Io {cause = IO.ClosedStream, ...} => true
val _ = print (Bool.toString (isClosed output) ^ "\n")
val _ = print (Bool.toString (isClosed (valOf (!hookOutput))) ^ "\n")
val _ = print (Bool.toString (TextIO.endOfStream input) ^ "\n")
val _ = TextIO.output (TextIO.stdOut, "stdout remains open\n")
val _ = TextIO.output (TextIO.stdErr, "stderr remains open\n")
val read = TextIO.openIn path
val _ = print (TextIO.inputAll read)
val _ = TextIO.closeIn read
val _ = OS.FileSys.remove path
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: true *)
(* CHECK-STDOUT-NEXT: true *)
(* CHECK-STDOUT-NEXT: true *)
(* CHECK-STDOUT-NEXT: stdout remains open *)
(* CHECK-STDOUT-NEXT: from hook *)
(* CHECK-STDERR: stderr remains open *)
