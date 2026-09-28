local
  datatype hidden = H
in
  val h = H
end
val x = (h = H)
(* CHECK-ERR: × unbound variable 'H' *)
(* CHECK-ERR: :6:14] *)
