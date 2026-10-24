fun show label value = print (label ^ ": " ^ value ^ "\n")
fun bool true = "true" | bool false = "false"

val _ = show "exit 0" (bool (OS.Process.isSuccess (OS.Process.system "exit 0")))
val _ = show "exit 3" (bool (OS.Process.isSuccess (OS.Process.system "exit 3")))
val _ = print "before\n"
val _ = OS.Process.system "echo from child"
val _ = print "after\n"

val half = Time.fromMilliseconds 1500
val quarter = Time.fromMicroseconds 250000
val _ = show "seconds" (LargeInt.toString (Time.toSeconds half))
val _ = show "milliseconds" (LargeInt.toString (Time.toMilliseconds half))
val _ = show "sum" (LargeInt.toString (Time.toMilliseconds (Time.+ (half, quarter))))
val _ = show "difference" (LargeInt.toString (Time.toMilliseconds (Time.- (half, quarter))))
val _ = show "negative" (LargeInt.toString (Time.toMilliseconds (Time.- (quarter, half))))
val _ = show "compare" (case Time.compare (quarter, half) of LESS => "less" | EQUAL => "equal" | GREATER => "greater")
val _ = show "ge" (bool (Time.>= (half, half)))
val _ = show "zero" (bool (Time.< (Time.zeroTime, quarter)))
val _ = show "real" (bool (Time.toReal (Time.fromReal 2.5) >= 2.5 andalso Time.toReal (Time.fromReal 2.5) <= 2.5))

val started = Time.now ()
val _ = OS.Process.sleep (Time.fromMilliseconds 120)
val elapsed = Time.toMilliseconds (Time.- (Time.now (), started))
val _ = show "slept enough" (bool (elapsed >= 100))
val _ = show "slept briefly" (bool (elapsed < 2000))
val _ = OS.Process.sleep Time.zeroTime
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: exit 0: true *)
(* CHECK-STDOUT-NEXT: exit 3: false *)
(* CHECK-STDOUT-NEXT: before *)
(* CHECK-STDOUT-NEXT: from child *)
(* CHECK-STDOUT-NEXT: after *)
(* CHECK-STDOUT-NEXT: seconds: 1 *)
(* CHECK-STDOUT-NEXT: milliseconds: 1500 *)
(* CHECK-STDOUT-NEXT: sum: 1750 *)
(* CHECK-STDOUT-NEXT: difference: 1250 *)
(* CHECK-STDOUT-NEXT: negative: ~1250 *)
(* CHECK-STDOUT-NEXT: compare: less *)
(* CHECK-STDOUT-NEXT: ge: true *)
(* CHECK-STDOUT-NEXT: zero: true *)
(* CHECK-STDOUT-NEXT: real: true *)
(* CHECK-STDOUT-NEXT: slept enough: true *)
(* CHECK-STDOUT-NEXT: slept briefly: true *)
(* WASM-SKIP: OS.Process.system raises SysErr in the browser *)
(* WASM-SKIP: OS.Process.system raises SysErr in the browser *)
