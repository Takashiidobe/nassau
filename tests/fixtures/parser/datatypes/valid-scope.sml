datatype t = A | B
fun first A = 1 | first B = 2
val inside = let datatype u = C | D in fn C => 1 | D => 2 end
fun again A = 1 | again B = 2
local
  datatype hidden = H | I
in
  datatype shown = S | T
  fun pick S = 1 | pick T = 2
  fun hid H = 1 | hid I = 2
end
datatype u = B | C
fun shadow B = 1 | shadow C = 2
fun old A = 1
(* CHECK-STDOUT: (datatype (t () (A) (B))) *)
(* CHECK-STDOUT-NEXT: (fun (first ((A) 1) ((B) 2))) *)
(* CHECK-STDOUT-NEXT: (val inside (let ((datatype (u () (C) (D)))) (fn (C 1) (D 2)))) *)
(* CHECK-STDOUT-NEXT: (fun (again ((A) 1) ((B) 2))) *)
(* CHECK-STDOUT-NEXT: (local ((datatype (hidden () (H) (I)))) ((datatype (shown () (S) (T))) (fun (pick ((S) 1) ((T) 2))) (fun (hid ((H) 1) ((I) 2))))) *)
(* CHECK-STDOUT-NEXT: (datatype (u () (B) (C))) *)
(* CHECK-STDOUT-NEXT: (fun (shadow ((B) 1) ((C) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (old ((A) 1))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 14:9 *)
