val root = "nassau-filesys-meta.tmp"

fun show label value = print (label ^ ": " ^ value ^ "\n")
fun bool true = "true" | bool false = "false"

fun write (path, text) =
  let val out = TextIO.openOut path
  in TextIO.output (out, text); TextIO.closeOut out end

fun attempt label f =
  (f (); show label "ok")
  handle OS.SysErr (message, _) => show label message

val _ = OS.FileSys.mkDir root
val first = root ^ "/a.txt"
val second = root ^ "/b.txt"
val _ = write (first, "data")

val _ = show "read" (bool (OS.FileSys.access (first, [OS.FileSys.A_READ])))
val _ = show "read write" (bool (OS.FileSys.access (first, [OS.FileSys.A_READ, OS.FileSys.A_WRITE])))
val _ = show "exec file" (bool (OS.FileSys.access (first, [OS.FileSys.A_EXEC])))
val _ = show "exists" (bool (OS.FileSys.access (first, [])))
val _ = show "exec dir" (bool (OS.FileSys.access (root, [OS.FileSys.A_EXEC])))
val _ = show "missing" (bool (OS.FileSys.access (root ^ "/missing", [])))

val _ = OS.FileSys.rename {old = first, new = second}
val _ = show "old gone" (bool (OS.FileSys.access (first, [])))
val _ = show "new there" (bool (OS.FileSys.access (second, [])))
val _ = attempt "rename missing" (fn () => OS.FileSys.rename {old = first, new = second})

val _ = OS.FileSys.setTime (second, SOME (Time.fromSeconds 1000000))
val _ = show "set time" (LargeInt.toString (Time.toSeconds (OS.FileSys.modTime second)))
val _ = OS.FileSys.setTime (second, NONE)
val age = Time.- (Time.now (), OS.FileSys.modTime second)
val _ = show "touched" (bool (Time.< (age, Time.fromSeconds 60)))
val _ = attempt "mod time missing" (fn () => ignore (OS.FileSys.modTime first))

val _ = show "full path" (OS.Path.file (OS.FileSys.fullPath second))
val _ = show "full path dots"
  (bool (OS.FileSys.fullPath (root ^ "/../" ^ root ^ "/./b.txt") = OS.FileSys.fullPath second))
val _ = show "real path" (OS.FileSys.realPath second)
val _ = attempt "full path missing" (fn () => ignore (OS.FileSys.fullPath first))

val _ = show "is link" (bool (OS.FileSys.isLink second))
val _ = attempt "read link file" (fn () => ignore (OS.FileSys.readLink second))

val idA = OS.FileSys.fileId second
val _ = show "id same" (bool (OS.FileSys.compare (idA, OS.FileSys.fileId (root ^ "/../" ^ root ^ "/b.txt")) = EQUAL))
val _ = show "id dir differs" (bool (OS.FileSys.compare (idA, OS.FileSys.fileId root) <> EQUAL))
val _ = show "id antisym"
  (bool (OS.FileSys.compare (idA, OS.FileSys.fileId root) = (case OS.FileSys.compare (OS.FileSys.fileId root, idA) of LESS => GREATER | GREATER => LESS | EQUAL => EQUAL)))
val _ = attempt "id missing" (fn () => ignore (OS.FileSys.fileId first))

val name = OS.FileSys.tmpName ()
val _ = show "tmp exists" (bool (OS.FileSys.access (name, [])))
val _ = OS.FileSys.remove name

val _ = OS.FileSys.remove second
val _ = OS.FileSys.rmDir root
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: read: true *)
(* CHECK-STDOUT-NEXT: read write: true *)
(* CHECK-STDOUT-NEXT: exec file: false *)
(* CHECK-STDOUT-NEXT: exists: true *)
(* CHECK-STDOUT-NEXT: exec dir: true *)
(* CHECK-STDOUT-NEXT: missing: false *)
(* CHECK-STDOUT-NEXT: old gone: false *)
(* CHECK-STDOUT-NEXT: new there: true *)
(* CHECK-STDOUT-NEXT: rename missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: set time: 1000000 *)
(* CHECK-STDOUT-NEXT: touched: true *)
(* CHECK-STDOUT-NEXT: mod time missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: full path: b.txt *)
(* CHECK-STDOUT-NEXT: full path dots: true *)
(* CHECK-STDOUT-NEXT: real path: nassau-filesys-meta.tmp/b.txt *)
(* CHECK-STDOUT-NEXT: full path missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: is link: false *)
(* CHECK-STDOUT-NEXT: read link file: Invalid argument *)
(* CHECK-STDOUT-NEXT: id same: true *)
(* CHECK-STDOUT-NEXT: id dir differs: true *)
(* CHECK-STDOUT-NEXT: id antisym: true *)
(* CHECK-STDOUT-NEXT: id missing: No such file or directory *)
(* CHECK-STDOUT-NEXT: tmp exists: true *)
