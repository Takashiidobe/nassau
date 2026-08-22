val counter = ref 0
val loop = while !counter < 10 do counter := !counter + 1
val sequence = (counter := 1; counter := 2; !counter)
val before_value = (!counter) before (counter := 0)
val chained = 1 before () before ()
val ignored = (ignore (counter := 5); !counter)
val in_let = let val i = ref 0 val acc = ref 0 in (while !i < 3 do (acc := !acc + !i; i := !i + 1); !acc) end
val guarded = if !counter > 0 then counter := 0 else ()
val while_value = while false do ()
val body_any = while false do 1
val unit_seq = ((); ())
fun sum_to n = let val i = ref 0 val acc = ref 0 in (while !i <= n do (acc := !acc + !i; i := !i + 1); !acc) end
fun tick c = (c := !c + 1; !c)
val total = sum_to 4 + tick counter
val mixed = let val r = ref 1.5 in (r := !r * 2.0; !r) end
(* CHECK-STDOUT: val counter : int ref *)
(* CHECK-STDOUT-NEXT: val loop : unit *)
(* CHECK-STDOUT-NEXT: val sequence : int *)
(* CHECK-STDOUT-NEXT: val before_value : int *)
(* CHECK-STDOUT-NEXT: val chained : int *)
(* CHECK-STDOUT-NEXT: val ignored : int *)
(* CHECK-STDOUT-NEXT: val in_let : int *)
(* CHECK-STDOUT-NEXT: val guarded : unit *)
(* CHECK-STDOUT-NEXT: val while_value : unit *)
(* CHECK-STDOUT-NEXT: val body_any : unit *)
(* CHECK-STDOUT-NEXT: val unit_seq : unit *)
(* CHECK-STDOUT-NEXT: val sum_to : int -> int *)
(* CHECK-STDOUT-NEXT: val tick : int ref -> int *)
(* CHECK-STDOUT-NEXT: val total : int *)
(* CHECK-STDOUT-NEXT: val mixed : real *)
(* CHECK-RUN-EXIT: 0 *)
