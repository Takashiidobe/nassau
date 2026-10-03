val a = "ab" ^ "c"
val b = "ab" ^ "d"
val same = "a" ^ "bc"
val prefix = "a" ^ "b"
val () = print (if a < b andalso b > a andalso a <= same andalso same >= a
  andalso not (a < same) andalso prefix < a andalso "" < prefix
  andalso "a\000a" < "a\000b" andalso "\255" > "\127" then "ordered\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ordered *)
