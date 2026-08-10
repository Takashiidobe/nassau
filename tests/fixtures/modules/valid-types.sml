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
val () = print (if a = (1, "a") andalso b = 2 andalso (case c of T.Red => true | _ => false) andalso (case d of T.Some p => p = (1, "a") | _ => false) andalso e = 2 andalso f = 0 andalso g = 1 then "valid-types\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-types *)
