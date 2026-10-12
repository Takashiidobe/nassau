val root = "nassau-filesys.tmp"

fun show label value = print (label ^ ": " ^ value ^ "\n")
fun bool true = "true" | bool false = "false"

fun insert (x : string, []) = [x]
  | insert (x, y :: rest) = if x <= y then x :: y :: rest else y :: insert (x, rest)
fun sort xs = List.foldl insert [] xs

fun entries stream =
  let fun loop acc =
        case OS.FileSys.readDir stream of
          NONE => sort acc
        | SOME name => loop (name :: acc)
  in loop [] end

fun write (path, text) =
  let val out = TextIO.openOut path
  in TextIO.output (out, text); TextIO.closeOut out end

fun attempt label f =
  (f (); show label "ok")
  handle OS.SysErr (message, _) => show label message

val _ = OS.FileSys.mkDir root
val _ = write (root ^ "/b.txt", "bee")
val _ = write (root ^ "/a.txt", "a")
val _ = OS.FileSys.mkDir (root ^ "/sub")

val _ = show "root is dir" (bool (OS.FileSys.isDir root))
val _ = show "file is dir" (bool (OS.FileSys.isDir (root ^ "/a.txt")))
val _ = show "size" (Int.toString (Position.toInt (OS.FileSys.fileSize (root ^ "/b.txt"))))

val stream = OS.FileSys.openDir root
val _ = show "entries" (String.concatWith "," (entries stream))
val _ = show "at end" (case OS.FileSys.readDir stream of NONE => "NONE" | SOME _ => "SOME")
val _ = OS.FileSys.rewindDir stream
val _ = show "after rewind" (Int.toString (length (entries stream)))
val _ = OS.FileSys.closeDir stream

val input = TextIO.openIn (root ^ "/b.txt")
val _ = show "read back" (TextIO.inputAll input)
val _ = TextIO.closeIn input

val _ = attempt "open missing" (fn () => ignore (OS.FileSys.openDir (root ^ "/missing")))
val _ = attempt "open file" (fn () => ignore (OS.FileSys.openDir (root ^ "/a.txt")))
val _ = attempt "size missing" (fn () => ignore (OS.FileSys.fileSize (root ^ "/missing")))
val _ = attempt "rmdir full" (fn () => OS.FileSys.rmDir root)
val _ = attempt "mkdir twice" (fn () => OS.FileSys.mkDir root)

val _ = OS.FileSys.remove (root ^ "/a.txt")
val _ = OS.FileSys.remove (root ^ "/b.txt")
val _ = OS.FileSys.rmDir (root ^ "/sub")
val _ = OS.FileSys.rmDir root
val _ = show "removed" (bool (not (OS.FileSys.isDir root) handle OS.SysErr _ => true))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: root is dir: true *)
(* CHECK-STDOUT-NEXT: file is dir: false *)
(* CHECK-STDOUT-NEXT: size: 3 *)
(* CHECK-STDOUT-NEXT: entries: a.txt,b.txt,sub *)
(* CHECK-STDOUT-NEXT: at end: NONE *)
(* CHECK-STDOUT-NEXT: after rewind: 3 *)
(* CHECK-STDOUT-NEXT: read back: bee *)
(* CHECK-STDOUT-NEXT: open missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: open file: Not a directory *)
(* CHECK-STDOUT-NEXT: size missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: rmdir full: Directory not empty *)
(* CHECK-STDOUT-NEXT: mkdir twice: File exists *)
(* CHECK-STDOUT-NEXT: removed: true *)
