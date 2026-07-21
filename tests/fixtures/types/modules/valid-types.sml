structure T = struct
  type pair = int * string
  type 'a box = 'a list
  datatype color = Red | Green
  datatype 'a opt = None | Some of 'a
  val p = (1, "a")
end
val a = T.p
val b = length ([1, 2] : int T.box)
val c : T.color = T.Red
val d : (int * string) T.opt = T.Some T.p
val e = (fn (x, _) => x + 1) (T.p : T.pair)
type mine = T.color T.opt
val f = case (T.None : mine) of T.None => 0 | T.Some _ => 1
val g = length ([[T.Red]] : T.color list T.box)
(* CHECK-STDOUT: val a : int * string *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : T.color *)
(* CHECK-STDOUT-NEXT: val d : (int * string) T.opt *)
(* CHECK-STDOUT-NEXT: val e : int *)
(* CHECK-STDOUT-NEXT: val f : int *)
(* CHECK-STDOUT-NEXT: val g : int *)
