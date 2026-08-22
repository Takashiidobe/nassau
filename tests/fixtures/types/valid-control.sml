fun safe_div (a, b) = a div b handle Div => 0
val guarded = (raise Fail "boom") handle Fail msg => size msg
val failing = fn () => raise Empty
val counter = ref 0
val ticked = (while !counter < 3 do counter := !counter + 1; !counter)
val choose = fn flag => if flag then "on" else "off"
val letrec = let fun go n = if n = 0 then 0 else go (n - 1) in go 3 end
val chain = fn x => (ignore x; x)
val ordered = fn (a, b) => a < b andalso not (b < a)
val _ = print (if safe_div (9,3) = 3 andalso safe_div (9,0) = 0 andalso guarded = 4 andalso ticked = 3 andalso choose true = "on" andalso letrec = 0 andalso chain 9 = 9 andalso ordered (1,2) then "control verified\n" else raise Fail "control")
(* CHECK-STDOUT: val safe_div : int * int -> int *)
(* CHECK-STDOUT-NEXT: val guarded : int *)
(* CHECK-STDOUT-NEXT: val failing : unit -> 'a *)
(* CHECK-STDOUT-NEXT: val counter : int ref *)
(* CHECK-STDOUT-NEXT: val ticked : int *)
(* CHECK-STDOUT-NEXT: val choose : bool -> string *)
(* CHECK-STDOUT-NEXT: val letrec : int *)
(* CHECK-STDOUT-NEXT: val chain : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val ordered : int * int -> bool *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: control verified *)
