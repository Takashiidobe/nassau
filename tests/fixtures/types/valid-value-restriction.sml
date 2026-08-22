fun id x = x
val cell = ref []
val ident = id id
val plain = fn x => x
val pair = (fn x => x, [])
val wrapped = SOME []
val applied = id []
val two_cells = (ref [], ref [])
val counter = ref 0
val stored = (counter := !counter + 1; !counter)
val (kept, n) = (ref [], 1)
(* CHECK-STDOUT: val id : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val cell : ?.X1 list ref *)
(* CHECK-STDOUT-NEXT: val ident : ?.X1 -> ?.X1 *)
(* CHECK-STDOUT-NEXT: val plain : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val pair : ('a -> 'a) * 'b list *)
(* CHECK-STDOUT-NEXT: val wrapped : 'a list option *)
(* CHECK-STDOUT-NEXT: val applied : ?.X1 list *)
(* CHECK-STDOUT-NEXT: val two_cells : ?.X1 list ref * ?.X2 list ref *)
(* CHECK-STDOUT-NEXT: val counter : int ref *)
(* CHECK-STDOUT-NEXT: val stored : int *)
(* CHECK-STDOUT-NEXT: val kept : ?.X1 list ref *)
(* CHECK-STDOUT-NEXT: val n : int *)
(* CHECK-RUN-EXIT: 0 *)
