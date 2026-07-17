exception Outer
val a = (raise Outer) handle Outer => 1
val b = let exception Inner of int in (raise Inner 2) handle Inner n => n end
val c = let exception Inner in [Inner, Inner] end
local
  exception Hidden
in
  exception Shown of string
  val d = (raise Hidden) handle Hidden => 3
  fun show () = raise Shown "s"
end
val e = show () handle Shown s => size s
exception Outer of int
val f = (raise Outer 5) handle Outer n => n
val g = Outer
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : exn list *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-STDOUT-NEXT: val show : unit -> 'a *)
(* CHECK-STDOUT-NEXT: val e : int *)
(* CHECK-STDOUT-NEXT: val f : int *)
(* CHECK-STDOUT-NEXT: val g : int -> exn *)
