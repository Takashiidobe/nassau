val r = ref 0
val bumped = (r := !r + 1; !r)
val swapped = let val a = ref 1 val b = ref 2 in (a := !b; b := 0; (!a, !b)) end
val chained = let val x = ref 0 in x := 1; x := 2; !x end
val nested = ref (ref 3)
val inner = !(!nested)
val assigned = !r = 1 andalso !r < 2
val f = op !
val g = op :=
val mapped = map ref [1, 2, 3]
val cell = ref []
(* CHECK-STDOUT: (val r (app ref 0)) *)
(* CHECK-STDOUT-NEXT: (val bumped (seq (:= r (+ (app ! r) 1)) (app ! r))) *)
(* CHECK-STDOUT-NEXT: (val swapped (let ((val a (app ref 1)) (val b (app ref 2))) (seq (:= a (app ! b)) (:= b 0) (tuple (app ! a) (app ! b))))) *)
(* CHECK-STDOUT-NEXT: (val chained (let ((val x (app ref 0))) (seq (:= x 1) (:= x 2) (app ! x)))) *)
(* CHECK-STDOUT-NEXT: (val nested (app ref (app ref 3))) *)
(* CHECK-STDOUT-NEXT: (val inner (app ! (app ! nested))) *)
(* CHECK-STDOUT-NEXT: (val assigned (andalso (= (app ! r) 1) (< (app ! r) 2))) *)
(* CHECK-STDOUT-NEXT: (val f !) *)
(* CHECK-STDOUT-NEXT: (val g :=) *)
(* CHECK-STDOUT-NEXT: (val mapped (app (app map ref) (list 1 2 3))) *)
(* CHECK-STDOUT-NEXT: (val cell (app ref (list))) *)
(* CHECK-RUN-EXIT: 0 *)
