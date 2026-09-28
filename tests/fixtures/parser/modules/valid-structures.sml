structure A = struct val x = 1 end
structure B = struct
  val y = A.x
  structure C = struct fun f n = n + 1 end
end
structure D = B.C
structure E = struct val a = 1 end and F = struct val b = 2 end
structure G = let val n = 3 in struct val m = n end end
(* CHECK-STDOUT: (structure (A (struct (val x 1)))) *)
(* CHECK-STDOUT-NEXT: (structure (B (struct (val y A.x) (structure (C (struct (fun (f ((n) (+ n 1)))))))))) *)
(* CHECK-STDOUT-NEXT: (structure (D B.C)) *)
(* CHECK-STDOUT-NEXT: (structure (E (struct (val a 1))) (F (struct (val b 2)))) *)
(* CHECK-STDOUT-NEXT: (structure (G (let ((val n 3)) (struct (val m n))))) *)
