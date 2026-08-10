signature S = sig eqtype t val a : t val b : t end
structure A :> S = struct type t = int val a = 1 val b = 2 end
val x = A.a = A.b
structure B : S = struct type t = string val a = "a" val b = "b" end
val y = B.a <> B.b
structure C :> S = struct datatype t = X | Y val a = X val b = Y end
val z = C.a = C.b
signature P = sig eqtype 'a t val mk : 'a -> 'a t end
structure D :> P = struct type 'a t = 'a list val mk = fn x => [x] end
val w = D.mk 1 = D.mk 2
val () = print (if not x andalso y andalso not z andalso not w then "valid-eqtype\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-eqtype *)
