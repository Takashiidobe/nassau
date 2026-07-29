val point = {y = 2, x = 1}
val {x, y} = point
val () = print (Int.toString x ^ "," ^ Int.toString y ^ "\n")
fun norm {x, y} = x * x + y * y
val () = print (Int.toString (norm {y = 4, x = 3}) ^ "\n")
(* A record with labels 1 and 2 is the same value as a pair. *)
val pair = {2 = "b", 1 = "a"}
val (first, second) = pair
val () = print (first ^ second ^ "\n")
(* Fields are evaluated in source order. *)
val r = {b = print "b ", a = print "a "}
val () = print "\n"
type person = {name : string, age : int}
fun older ({name, age} : person) = {name = name, age = age + 1}
fun describe ({name, age = a as 31} : person) = name ^ " is 31"
  | describe {name, ...} = name ^ " is not 31"
val () = print (describe (older {age = 30, name = "Ada"}) ^ "\n")
val () = print (describe {age = 40, name = "Bob"} ^ "\n")
val {name = who, ...} : person = {name = "Cy", age = 5}
val () = print (who ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1,2 *)
(* CHECK-STDOUT-NEXT: 25 *)
(* CHECK-STDOUT-NEXT: ab *)
(* CHECK-STDOUT-NEXT: b a *)
(* CHECK-STDOUT-NEXT: Ada is 31 *)
(* CHECK-STDOUT-NEXT: Bob is not 31 *)
(* CHECK-STDOUT-NEXT: Cy *)
