signature S = sig type t type u val x : t val y : u end
structure A :> S where type t = int = struct type t = int type u = bool val x = 1 val y = true end
val a = A.x + 1
structure B :> S where type t = int where type u = string =
  struct type t = int type u = string val x = 1 val y = "y" end
val b = (B.x + 1, B.y ^ "!")
signature INTS = S where type t = int and type u = int
structure C :> INTS = struct type t = int type u = int val x = 1 val y = 2 end
val c = C.x + C.y
signature LIST = sig type 'a t val none : 'a t end
structure L :> LIST where type 'a t = 'a list = struct type 'a t = 'a list val none = [] end
val d = 1 :: L.none
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int * string *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : int list *)
(* CHECK-RUN-EXIT: 0 *)
