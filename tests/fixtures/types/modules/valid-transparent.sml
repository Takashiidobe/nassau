signature S = sig type t val make : int -> t val get : t -> int end
structure A : S = struct
  type t = int
  fun make n = n
  fun get n = n
end
val a = A.make 1 + 2
val b = A.get 3 * 2
val c = A.make 1 = A.make 2
structure B : S where type t = int = struct
  type t = int
  fun make n = n
  fun get n = n
end
val d = B.make 1 + 1
structure C : sig type t = int val x : t end = struct type t = int val x = 3 end
val e = C.x + 1
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : bool *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-STDOUT-NEXT: val e : int *)
