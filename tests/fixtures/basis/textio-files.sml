val path = "nassau-textio-files.tmp"

fun show label value = print (label ^ ": " ^ value ^ "\n")
fun showLine NONE = "NONE"
  | showLine (SOME line) = "SOME " ^ String.toString line

val out = TextIO.openOut path
val _ = TextIO.output (out, "one\ntwo\n")
val _ = TextIO.output1 (out, #"t")
val _ = TextIO.output (out, "hree")
val _ = TextIO.closeOut out

val appended = TextIO.openAppend path
val _ = TextIO.output (appended, "\nfour\n")
val _ = TextIO.closeOut appended

val input = TextIO.openIn path
val _ = show "line 1" (showLine (TextIO.inputLine input))
val _ = show "char" (case TextIO.input1 input of SOME c => Char.toString c | NONE => "NONE")
val _ = show "lookahead" (case TextIO.lookahead input of SOME c => Char.toString c | NONE => "NONE")
val _ = show "inputN" (TextIO.inputN (input, 4))
val _ = show "line" (showLine (TextIO.inputLine input))
val _ = show "line" (showLine (TextIO.inputLine input))
val _ = show "line" (showLine (TextIO.inputLine input))
val _ = show "eof" (Bool.toString (TextIO.endOfStream input))
val _ = TextIO.closeIn input

val whole = TextIO.openIn path
val _ = show "all" (String.toString (TextIO.inputAll whole))
val _ = TextIO.closeIn whole

val _ = show "closed" ((TextIO.input1 whole; "no error") handle IO.Io {function, ...} => function)

val _ = show "missing"
  ((TextIO.openIn "nassau-no-such-directory/file"; "no error")
   handle IO.Io {name, function, cause = OS.SysErr (message, _)} =>
     function ^ " " ^ name ^ ": " ^ message)

val _ = OS.FileSys.remove path
val _ = show "removed" ((TextIO.openIn path; "still there") handle IO.Io _ => "gone")
val _ = show "remove again" ((OS.FileSys.remove path; "no error") handle OS.SysErr (message, _) => message)

val _ = show "home set" (Bool.toString (isSome (OS.Process.getEnv "PATH")))
val _ = show "unset" (case OS.Process.getEnv "NASSAU_SURELY_UNSET_VARIABLE" of NONE => "NONE" | SOME _ => "SOME")

val _ = TextIO.output (TextIO.stdOut, "through stdOut\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: line 1: SOME one\n *)
(* CHECK-STDOUT-NEXT: char: t *)
(* CHECK-STDOUT-NEXT: lookahead: w *)
(* CHECK-STDOUT-NEXT: inputN: wo *)
(* CHECK-STDOUT-NEXT: t *)
(* CHECK-STDOUT-NEXT: line: SOME hree\n *)
(* CHECK-STDOUT-NEXT: line: SOME four\n *)
(* CHECK-STDOUT-NEXT: line: NONE *)
(* CHECK-STDOUT-NEXT: eof: true *)
(* CHECK-STDOUT-NEXT: all: one\ntwo\nthree\nfour\n *)
(* CHECK-STDOUT-NEXT: closed: no error *)
(* CHECK-STDOUT-NEXT: missing: TextIO.openIn nassau-no-such-directory/file: No such file or directory *)
(* CHECK-STDOUT-NEXT: removed: gone *)
(* CHECK-STDOUT-NEXT: remove again: No such file or directory *)
(* CHECK-STDOUT-NEXT: home set: true *)
(* CHECK-STDOUT-NEXT: unset: NONE *)
(* CHECK-STDOUT-NEXT: through stdOut *)
