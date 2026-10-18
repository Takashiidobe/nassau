datatype t = C | D
val C = C
val (C : t) = C
val op nil = []
val _ = print (case C of C => "constructor\n" | D => "wrong\n")
val _ = print ((let val C = D in "wrong\n" end) handle Bind => "Bind\n")
exception E
val E = E
val _ = print ((let val E = Fail "x" in "wrong\n" end) handle Bind => "exn Bind\n")
val ref n = ref 4
val _ = print (Int.toString n ^ "\n")
fun C x = x + 1
fun E x = x + 2
val _ = print (Int.toString (C (E 3)) ^ "\n")
structure S = struct datatype t = A | B end
val S.A = S.A
val _ = print ((let val S.A = S.B in "wrong\n" end) handle Bind => "qualified Bind\n")
signature IT = sig val it:int end
structure M:IT = struct val it=7 end
val _ = print (Int.toString M.it ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: constructor *)
(* CHECK-STDOUT-NEXT: Bind *)
(* CHECK-STDOUT-NEXT: exn Bind *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: 6 *)
(* CHECK-STDOUT-NEXT: qualified Bind *)
(* CHECK-STDOUT-NEXT: 7 *)
