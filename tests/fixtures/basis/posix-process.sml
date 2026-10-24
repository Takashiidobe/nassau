structure P = Posix.Process
fun show label value = print (label ^ ": " ^ value ^ "\n")
fun bool true = "true" | bool false = "false"
fun status (P.W_EXITED) = "exited"
  | status (P.W_EXITSTATUS w) = "exit " ^ Word8.toString w
  | status (P.W_SIGNALED s) = "signaled " ^ (if s = Posix.Signal.term then "term" else if s = Posix.Signal.kill then "kill" else "other")
  | status (P.W_STOPPED _) = "stopped"

val _ = show "fromStatus 0" (status (P.fromStatus OS.Process.success))

val _ = case P.fork () of
    NONE => P.exit (Word8.fromInt 7)
  | SOME pid =>
      let val (got, st) = P.wait ()
      in show "wait pid" (bool (got = pid)); show "wait status" (status st) end

val _ = case P.fork () of
    NONE => P.exit (Word8.fromInt 0)
  | SOME pid =>
      let val (got, st) = P.waitpid (P.W_CHILD pid, [])
      in show "waitpid pid" (bool (got = pid)); show "waitpid status" (status st) end

val _ = case P.fork () of
    NONE => (P.sleep (Time.fromSeconds 30); P.exit (Word8.fromInt 1))
  | SOME pid =>
      (show "nh running" (bool (P.waitpid_nh (P.W_CHILD pid, []) = NONE));
       P.kill (P.K_PROC pid, Posix.Signal.term);
       let val (got, st) = P.waitpid (P.W_CHILD pid, [])
       in show "killed pid" (bool (got = pid)); show "killed status" (status st) end)

val _ = case P.fork () of
    NONE => P.exec ("/bin/sh", ["sh", "-c", "exit 5"])
  | SOME _ => show "exec status" (status (#2 (P.wait ())))

val _ = case P.fork () of
    NONE => P.execp ("sh", ["sh", "-c", "exit 6"])
  | SOME _ => show "execp status" (status (#2 (P.wait ())))

val _ = case P.fork () of
    NONE => P.exece ("/bin/sh", ["sh", "-c", "test \"$NASSAU_X\" = hello && exit 8; exit 9"], ["NASSAU_X=hello"])
  | SOME _ => show "exece status" (status (#2 (P.wait ())))

val _ = show "exec missing" ((P.exec ("/nonexistent/prog", ["prog"]); "ran") handle OS.SysErr _ => "SysErr")
val _ = show "wait without children" ((P.wait (); "ok") handle OS.SysErr _ => "SysErr")
val _ = show "alarm none" (Time.toString (P.alarm (Time.fromSeconds 100)))
val _ = show "alarm prev" (Time.toString (P.alarm Time.zeroTime))
val _ = show "sleep" (Time.toString (P.sleep Time.zeroTime))
val _ = show "pid words" (bool (P.wordToPid (P.pidToWord (Posix.ProcEnv.getpid ())) = Posix.ProcEnv.getpid ()))
val _ = show "signal words" (bool (Posix.Signal.fromWord (Posix.Signal.toWord Posix.Signal.int) = Posix.Signal.int))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: fromStatus 0: exited *)
(* CHECK-STDOUT-NEXT: wait pid: true *)
(* CHECK-STDOUT-NEXT: wait status: exit 7 *)
(* CHECK-STDOUT-NEXT: waitpid pid: true *)
(* CHECK-STDOUT-NEXT: waitpid status: exited *)
(* CHECK-STDOUT-NEXT: nh running: true *)
(* CHECK-STDOUT-NEXT: killed pid: true *)
(* CHECK-STDOUT-NEXT: killed status: signaled term *)
(* CHECK-STDOUT-NEXT: exec status: exit 5 *)
(* CHECK-STDOUT-NEXT: execp status: exit 6 *)
(* CHECK-STDOUT-NEXT: exece status: exit 8 *)
(* CHECK-STDOUT-NEXT: exec missing: SysErr *)
(* CHECK-STDOUT-NEXT: wait without children: SysErr *)
(* CHECK-STDOUT-NEXT: alarm none: 0.000 *)
(* CHECK-STDOUT-NEXT: alarm prev: 100.000 *)
(* CHECK-STDOUT-NEXT: sleep: 0.000 *)
(* CHECK-STDOUT-NEXT: pid words: true *)
(* CHECK-STDOUT-NEXT: signal words: true *)
(* WASM-SKIP: Posix process calls are unsupported in the browser *)
