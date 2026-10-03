fun id x = x
fun const x y = x
fun swap (a, b) = (b, a)
fun compose f g x = f (g x)
fun twice f x = f (f x)
val both = (id 1, id "s", id [true])
val shared = const 1 "ignored"
val local_poly = let val same = fn x => x in (same 1, same "s") end
val curried = fn x => fn y => (y, x)
val head_of = hd
val nothing = []
val none = NONE
val pairs = map (fn x => (x, x)) [1, 2]
val folded = foldl (fn (x, acc) => x + acc) 0 [1, 2, 3]
fun apply_all fs x = map (fn f => f x) fs
val opt = SOME (SOME 1)
(* RUNTIME-SKIP: built-in functions used as values, such as hd, are not supported by code generation yet *)
(* CHECK-STDOUT: val id : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val const : 'a -> 'b -> 'a *)
(* CHECK-STDOUT-NEXT: val swap : 'a * 'b -> 'b * 'a *)
(* CHECK-STDOUT-NEXT: val compose : ('a -> 'b) -> ('c -> 'a) -> 'c -> 'b *)
(* CHECK-STDOUT-NEXT: val twice : ('a -> 'a) -> 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val both : int * string * bool list *)
(* CHECK-STDOUT-NEXT: val shared : int *)
(* CHECK-STDOUT-NEXT: val local_poly : int * string *)
(* CHECK-STDOUT-NEXT: val curried : 'a -> 'b -> 'b * 'a *)
(* CHECK-STDOUT-NEXT: val head_of : 'a list -> 'a *)
(* CHECK-STDOUT-NEXT: val nothing : 'a list *)
(* CHECK-STDOUT-NEXT: val none : 'a option *)
(* CHECK-STDOUT-NEXT: val pairs : (int * int) list *)
(* CHECK-STDOUT-NEXT: val folded : int *)
(* CHECK-STDOUT-NEXT: val apply_all : ('a -> 'b) list -> 'a -> 'b list *)
(* CHECK-STDOUT-NEXT: val opt : int option option *)
(* CHECK-RUN-ERR: × built-in functions used as values, such as hd, are not supported by code *)
(* CHECK-RUN-ERR: :10:15] *)
