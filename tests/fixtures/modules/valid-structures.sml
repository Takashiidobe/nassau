structure A = struct
  val x = 1
  fun twice f y = f (f y)
  val id = fn z => z
end
val a = A.x
val b = A.twice (fn n => n + 1) 5
val c = A.id "s"
val d = A.id 3
structure B = struct
  structure C = struct val name = "c" end
  val n = size C.name
end
val e = B.C.name
val f = B.n
structure D = B.C
val g = D.name
structure E = struct val a = 1 end and F = struct val b = "two" end
val h = (E.a, F.b)
val () = print (if a = 1 andalso b = 7 andalso c = "s" andalso d = 3 andalso e = "c" andalso f = 1 andalso g = "c" andalso h = (1, "two") then "valid-structures\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-structures *)
