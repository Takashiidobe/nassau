fun safe_div (a, b) = a div b handle Div => 0
val guarded = (raise Fail "boom") handle Fail msg => size msg
val failing = fn () => raise Empty
val counter = ref 0
val ticked = (while !counter < 3 do counter := !counter + 1; !counter)
val choose = fn flag => if flag then "on" else "off"
val letrec = let fun go n = if n = 0 then 0 else go (n - 1) in go 3 end
val chain = fn x => (ignore x; x)
val ordered = fn (a, b) => a < b andalso not (b < a)
(* CHECK-STDOUT: val safe_div : int * int -> int *)
(* CHECK-STDOUT-NEXT: val guarded : int *)
(* CHECK-STDOUT-NEXT: val failing : unit -> 'a *)
(* CHECK-STDOUT-NEXT: val counter : int ref *)
(* CHECK-STDOUT-NEXT: val ticked : int *)
(* CHECK-STDOUT-NEXT: val choose : bool -> string *)
(* CHECK-STDOUT-NEXT: val letrec : int *)
(* CHECK-STDOUT-NEXT: val chain : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val ordered : int * int -> bool *)
