structure Process : OS_PROCESS = OS.Process
val _ = print (Bool.toString (Process.isSuccess Process.success) ^ "\n")
val _ = print (Bool.toString (Process.isSuccess Process.failure) ^ "\n")
val _ = Process.sleep Time.zeroTime
val exit : Process.status -> string = Process.exit
val terminate : Process.status -> int = Process.terminate
val _ = Process.atExit (fn () => print "signature hook\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: true *)
(* CHECK-STDOUT-NEXT: false *)
(* CHECK-STDOUT-NEXT: signature hook *)
