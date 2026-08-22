signature EMPTY = sig end
signature COPY = sig
  structure A : sig datatype color = Red | Green of int end
  datatype shade = datatype A.color
end
signature ORDERED = sig
  type t
  eqtype key
  type pair = t * key
  val compare : t * t -> order
  val ++ : t * t -> t
  datatype shape = Square | Circle of int
  exception Bad of string and Worse
  structure Inner : sig type u end
  include EMPTY
  sharing type t = key
end and OTHER = sig val x : int end
(* CHECK-STDOUT: (signature (EMPTY (sig))) *)
(* CHECK-STDOUT-NEXT: (signature (COPY (sig (structure (A (sig (datatype (color () (Red) (Green int)))))) (datatype-copy shade A.color)))) *)
(* CHECK-STDOUT-NEXT: (signature (ORDERED (sig (type (t ())) (type (eq key ())) (type (pair () {{[(]}}* t key))) (val (compare (-> {{[(]}}* t t) order))) (val (++ (-> {{[(]}}* t t) t))) (datatype (shape () (Square) (Circle int))) (exception (Bad string) (Worse)) (structure (Inner (sig (type (u ()))))) (include EMPTY) (sharing t key))) (OTHER (sig (val (x int))))) *)
(* CHECK-RUN-EXIT: 0 *)
