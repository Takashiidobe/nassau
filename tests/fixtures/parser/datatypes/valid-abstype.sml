abstype counter = Counter of int
with
  val zero = Counter 0
  fun bump (Counter n) = Counter (n + 1)
  fun read (Counter n) = n
end
abstype 'a box = Box of 'a
withtype 'a alias = 'a box
with
  fun box x = Box x
  fun unbox (Box x) = x
end
(* CHECK-STDOUT: (abstype (counter () (Counter int)) (with (val zero (app Counter 0)) (fun (bump (((con Counter n)) (app Counter (+ n 1))))) (fun (read (((con Counter n)) n))))) *)
(* CHECK-STDOUT-NEXT: (abstype (box ('a) (Box 'a)) (withtype (alias ('a) (tycon box 'a))) (with (fun (box ((x) (app Box x)))) (fun (unbox (((con Box x)) x))))) *)
(* CHECK-RUN-EXIT: 0 *)
