structure IntOrd = struct type t = int fun le (a : int, b) = a <= b end
functor Sort (O : sig type t val le : t * t -> bool end) = struct val le = O.le end
structure S = Sort (IntOrd)
structure T = Sort (struct type t = int fun le (a : int, b) = a <= b end)
structure U = Sort (type t = string fun le (a : string, b) = a <= b)
structure V = Sort (IntOrd) : sig val le : int * int -> bool end
functor Empty () = struct val z = 1 end
structure W = Empty ()
structure X = Empty (val unused = 2; val more = 3)
(* CHECK-STDOUT: (structure (IntOrd (struct (type (t () int)) (fun (le (((tuple (: a int) b)) (<= a b))))))) *)
(* CHECK-STDOUT-NEXT: (functor (Sort (O (sig (type (t ())) (val (le (-> {{[(]}}* t t) bool))))) (struct (val le O.le)))) *)
(* CHECK-STDOUT-NEXT: (structure (S (app Sort IntOrd))) *)
(* CHECK-STDOUT-NEXT: (structure (T (app Sort (struct (type (t () int)) (fun (le (((tuple (: a int) b)) (<= a b)))))))) *)
(* CHECK-STDOUT-NEXT: (structure (U (app Sort (struct (type (t () string)) (fun (le (((tuple (: a string) b)) (<= a b)))))))) *)
(* CHECK-STDOUT-NEXT: (structure (V (: (app Sort IntOrd) (sig (val (le (-> {{[(]}}* int int) bool))))))) *)
(* CHECK-STDOUT-NEXT: (functor (Empty (sig) (struct (val z 1)))) *)
(* CHECK-STDOUT-NEXT: (structure (W (app Empty (struct)))) *)
(* CHECK-STDOUT-NEXT: (structure (X (app Empty (struct (val unused 2) (val more 3))))) *)
(* CHECK-RUN-EXIT: 0 *)
