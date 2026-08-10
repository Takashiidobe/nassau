signature ERR = sig
  exception Bad of string
  exception Worse
  val fail : string -> 'a
end
structure E : ERR = struct
  exception Bad of string
  exception Worse
  fun fail s = raise Bad s
end
val a = E.fail "x" handle E.Bad s => s
val b = (E.fail "y" handle E.Worse => "worse") handle E.Bad _ => "bad"
structure F :> ERR = E
val c = (F.fail "z" : int) handle F.Bad s => size s
exception Local = E.Bad
val d = (raise Local "l") handle Local m => m
val () = print (if a = "x" andalso b = "bad" andalso c = 1 andalso d = "l" then "valid-exception-specs\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-exception-specs *)
