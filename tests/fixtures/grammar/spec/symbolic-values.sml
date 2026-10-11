signature SYMBOLIC =
sig
  val + : int * int -> int and - : int * int -> int
  val * : int * int -> int
  val / : real * real -> real
  val div : int * int -> int
  val mod : int * int -> int
  val ~ : int -> int
  val <> : int * int -> bool
  val < : int * int -> bool
  val <= : int * int -> bool
  val > : int * int -> bool
  val >= : int * int -> bool
  val ^ : string * string -> string
  val @ : int list * int list -> int list
end
val _ = print "ok\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
