val state = ref 0
fun tick n = (state := !state * 10 + n; n)
structure A = struct val x = tick 1 end
structure B = struct val x = tick 2 end and C = struct val x = A.x end
structure Alias = A
structure A = struct val y = tick 3 end
val () = print (Int.toString (!state) ^ " " ^ Int.toString (Alias.x + B.x + C.x) ^ "\n")
structure Outer = struct
  val x = 10
  structure Inner = struct val x = 20 end
end
val () = print (Int.toString (let open Outer Outer.Inner in x end) ^ "\n")
structure Made = let
  val x = 5
  structure Local = struct fun add k = x + k end
in struct val add = Local.add end end
val add = Made.add
val () = print (Int.toString (add 4) ^ "\n")
structure Local = struct val x = 100 end
structure Scoped = let structure Local = struct val x = 1 end in struct val f = Local.x end end
val f = Scoped.f
val () = print (Int.toString (f + Local.x) ^ "\n")
val A = 99
structure Constructors = struct datatype t = A | B of int end
val () = print (let open Constructors in case A of A => "constructor\n" | _ => "wrong\n" end)
datatype t = A of int | B | C
val () = print (case A 8 of A n => Int.toString n ^ "\n" | _ => "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 123 4 *)
(* CHECK-STDOUT-NEXT: 20 *)
(* CHECK-STDOUT-NEXT: 9 *)
(* CHECK-STDOUT-NEXT: 101 *)
(* CHECK-STDOUT-NEXT: constructor *)
(* CHECK-STDOUT-NEXT: 8 *)
