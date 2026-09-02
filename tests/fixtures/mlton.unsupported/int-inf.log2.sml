(* mlton regression/int-inf.log2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ =
   List.app
   (fn i => print (concat [Int.toString (IntInf.log2 i), "\n"]))
   [1,
    2,
    3,
    0x10000000,
    0x20000000,
    0x40000000,
    0x80000000,
    0x100000000,
    0x1FFFFFFFF,
    0x200000000,
    0x200000001]

val _ =
   List.app
   (fn i =>
    if i = IntInf.log2 (IntInf.pow (2, i))
       andalso i = IntInf.log2 (IntInf.pow (2, i) + 1)
       andalso i - 1 = IntInf.log2 (IntInf.pow (2, i) - 1)
       then ()
    else raise Fail "bug")
   (List.tabulate (100, fn i => i + 1))

val _ = print "OK\n"
      
    
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 0 *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: 28 *)
(* CHECK-STDOUT-NEXT: 29 *)
(* CHECK-STDOUT-NEXT: 30 *)
(* CHECK-STDOUT-NEXT: 31 *)
(* CHECK-STDOUT-NEXT: 32 *)
(* CHECK-STDOUT-NEXT: 32 *)
(* CHECK-STDOUT-NEXT: 33 *)
(* CHECK-STDOUT-NEXT: 33 *)
(* CHECK-STDOUT-NEXT: OK *)
