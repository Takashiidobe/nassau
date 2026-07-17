local
  exception Hidden
in
  val x = 1
end
val y = raise Hidden
(* CHECK-ERR: × unbound variable 'Hidden' *)
(* CHECK-ERR: :6:15] *)
