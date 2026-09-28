infix 5 ++
infixr 6 ==>
datatype expr = Lit of int | ++ of expr * expr | ==> of expr * expr
fun size (Lit _) = 1
  | size (a ++ b) = size a + size b
  | size (a ==> b) = size a + size b
fun shallow (Lit n ++ Lit m) = n + m
  | shallow (a ==> b ==> c) = 0
  | shallow _ = 1
val built = Lit 1 ++ Lit 2 ==> Lit 3
(* CHECK-STDOUT: (infix 5 ++) *)
(* CHECK-STDOUT-NEXT: (infixr 6 ==>) *)
(* CHECK-STDOUT-NEXT: (datatype (expr () (Lit int) (++ {{[(]}}* expr expr)) (==> {{[(]}}* expr expr)))) *)
(* CHECK-STDOUT-NEXT: (fun (size (((con Lit _)) 1) (((con ++ (tuple a b))) (+ (app size a) (app size b))) (((con ==> (tuple a b))) (+ (app size a) (app size b))))) *)
(* CHECK-STDOUT-NEXT: (fun (shallow (((con ++ (tuple (con Lit n) (con Lit m)))) (+ n m)) (((con ==> (tuple a (con ==> (tuple b c))))) 0) ((_) 1))) *)
(* CHECK-STDOUT-NEXT: (val built (++ (app Lit 1) (==> (app Lit 2) (app Lit 3)))) *)
