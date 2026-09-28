exception Outer
val a = (raise Outer) handle Outer => 1
val b = let exception Inner in (raise Inner) handle Inner => 2 end
local
  exception Hidden
in
  exception Shown
  val c = (raise Hidden) handle Hidden => 3
  val d = (raise Shown) handle Shown => 4
end
exception Outer of int
val e = (raise Outer 5) handle Outer n => n
(* CHECK-STDOUT: (exception (Outer)) *)
(* CHECK-STDOUT-NEXT: (val a (handle (raise Outer) (Outer 1))) *)
(* CHECK-STDOUT-NEXT: (val b (let ((exception (Inner))) (handle (raise Inner) (Inner 2)))) *)
(* CHECK-STDOUT-NEXT: (local ((exception (Hidden))) ((exception (Shown)) (val c (handle (raise Hidden) (Hidden 3))) (val d (handle (raise Shown) (Shown 4))))) *)
(* CHECK-STDOUT-NEXT: (exception (Outer int)) *)
(* CHECK-STDOUT-NEXT: (val e (handle (raise (app Outer 5)) ((con Outer n) n))) *)
