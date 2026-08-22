val r = ref 0
val cell = ref []
val id_ref = ref (fn x => x)
val unit_ref = ref ()
val nested = ref (ref "s")
val deref = !r
val inner = !(!nested)
val assigned = r := 5
val stored = (r := !r + 1; !r)
val fresh = fn x => ref x
val get = fn c => !c
val set = fn (c, v) => c := v
val swap = fn (a, b) => let val t = !a in (a := !b; b := t) end
val mapped = map ref [1, 2, 3]
val as_function = ref
val deref_fn = op !
val assign_fn = op :=
val pair = (ref 1, ref "a")
val same = r = r
val differ = ref 1 <> ref 2
val eq_poly = fn (a, b) => a = b
val list_ref = ref [1, 2]
val pushed = (list_ref := 0 :: !list_ref; !list_ref)
val _ = (set (r,9); swap (r, #1 pair); print (if get r = 1 andalso !(#1 pair) = 9 andalso inner = "s" andalso stored = 6 andalso same andalso differ andalso pushed = [0,1,2] then "references verified\n" else raise Fail "references"))
(* CHECK-STDOUT: val r : int ref *)
(* CHECK-STDOUT-NEXT: val cell : ?.X1 list ref *)
(* CHECK-STDOUT-NEXT: val id_ref : (?.X1 -> ?.X1) ref *)
(* CHECK-STDOUT-NEXT: val unit_ref : unit ref *)
(* CHECK-STDOUT-NEXT: val nested : string ref ref *)
(* CHECK-STDOUT-NEXT: val deref : int *)
(* CHECK-STDOUT-NEXT: val inner : string *)
(* CHECK-STDOUT-NEXT: val assigned : unit *)
(* CHECK-STDOUT-NEXT: val stored : int *)
(* CHECK-STDOUT-NEXT: val fresh : 'a -> 'a ref *)
(* CHECK-STDOUT-NEXT: val get : 'a ref -> 'a *)
(* CHECK-STDOUT-NEXT: val set : 'a ref * 'a -> unit *)
(* CHECK-STDOUT-NEXT: val swap : 'a ref * 'a ref -> unit *)
(* CHECK-STDOUT-NEXT: val mapped : int ref list *)
(* CHECK-STDOUT-NEXT: val as_function : 'a -> 'a ref *)
(* CHECK-STDOUT-NEXT: val deref_fn : 'a ref -> 'a *)
(* CHECK-STDOUT-NEXT: val assign_fn : 'a ref * 'a -> unit *)
(* CHECK-STDOUT-NEXT: val pair : int ref * string ref *)
(* CHECK-STDOUT-NEXT: val same : bool *)
(* CHECK-STDOUT-NEXT: val differ : bool *)
(* CHECK-STDOUT-NEXT: val eq_poly : ''a * ''a -> bool *)
(* CHECK-STDOUT-NEXT: val list_ref : int list ref *)
(* CHECK-STDOUT-NEXT: val pushed : int list *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: references verified *)
