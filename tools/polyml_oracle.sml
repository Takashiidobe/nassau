val path = valOf (OS.Process.getEnv "NASSAU_ORACLE_FILE")
val echo = OS.Process.getEnv "NASSAU_ORACLE_ECHO" = SOME "1"
val input = TextIO.openIn path
val line = ref 1
fun next () =
  case TextIO.input1 input of
    SOME c => (if c = #"\n" then line := !line + 1 else (); SOME c)
  | NONE => NONE
fun diagnostic {message, hard, location, context} =
  (TextIO.output (TextIO.stdErr, if hard then "error: " else "warning: ");
   PolyML.prettyPrint (fn s => TextIO.output (TextIO.stdErr, s), 1000) message;
   TextIO.output (TextIO.stdErr, "\n"))
fun loop () =
  if TextIO.endOfStream input then ()
  else
    (PolyML.compiler (next,
       [PolyML.Compiler.CPOutStream (if echo then print else fn _ => ()),
        PolyML.Compiler.CPPrintDepth (fn () => if echo then 1000 else 0),
        PolyML.Compiler.CPPrintInAlphabeticalOrder false,
        PolyML.Compiler.CPLineLength 1000,
        PolyML.Compiler.CPLineNo (fn () => !line),
        PolyML.Compiler.CPFileName path,
        PolyML.Compiler.CPErrorMessageProc diagnostic]) ();
     loop ())
val _ = loop () handle e =>
  (TextIO.output (TextIO.stdErr, "Exception- " ^ General.exnMessage e ^ " raised\n");
   OS.Process.exit OS.Process.failure)
