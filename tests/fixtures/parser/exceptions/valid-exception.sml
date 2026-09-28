exception Empty
exception Failed of string
exception Pair of int * string and Unit_like
exception Alias = Failed
exception Both = Empty and Also = Unit_like
exception op Wrapped of {code : int}
exception Fun of int -> int
(* CHECK-STDOUT: (exception (Empty)) *)
(* CHECK-STDOUT-NEXT: (exception (Failed string)) *)
(* CHECK-STDOUT-NEXT: (exception (Pair {{[(]}}* int string)) (Unit_like)) *)
(* CHECK-STDOUT-NEXT: (exception (Alias = Failed)) *)
(* CHECK-STDOUT-NEXT: (exception (Both = Empty) (Also = Unit_like)) *)
(* CHECK-STDOUT-NEXT: (exception (Wrapped (record (code int)))) *)
(* CHECK-STDOUT-NEXT: (exception (Fun (-> int int))) *)
