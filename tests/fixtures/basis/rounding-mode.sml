fun name IEEEReal.TO_NEAREST = "nearest"
  | name IEEEReal.TO_NEGINF = "neginf"
  | name IEEEReal.TO_POSINF = "posinf"
  | name IEEEReal.TO_ZERO = "zero"
val nearest = 0.3333333333333333
fun third () =
  let val x = Real.fromInt 1 / Real.fromInt 3
  in if x > nearest then "above" else if x < nearest then "below" else "nearest" end
val _ = print (name (IEEEReal.getRoundingMode ()) ^ " " ^ third () ^ "\n")
val _ = IEEEReal.setRoundingMode IEEEReal.TO_POSINF
val _ = print (name (IEEEReal.getRoundingMode ()) ^ " " ^ third () ^ "\n")
val _ = IEEEReal.setRoundingMode IEEEReal.TO_NEGINF
val _ = print (name (IEEEReal.getRoundingMode ()) ^ " " ^ third () ^ "\n")
val _ = IEEEReal.setRoundingMode IEEEReal.TO_ZERO
val _ = print (name (IEEEReal.getRoundingMode ()) ^ " " ^ third () ^ "\n")
val _ = IEEEReal.setRoundingMode IEEEReal.TO_NEAREST
val _ = print (name (IEEEReal.getRoundingMode ()) ^ " " ^ third () ^ "\n")
(* CHECK-STDOUT: nearest nearest *)
(* CHECK-STDOUT-NEXT: posinf above *)
(* CHECK-STDOUT-NEXT: neginf nearest *)
(* CHECK-STDOUT-NEXT: zero nearest *)
(* CHECK-STDOUT-NEXT: nearest nearest *)
(* CHECK-EXIT: 0 *)
(* WASM-SKIP: WebAssembly always rounds to nearest, so other modes raise Fail *)
