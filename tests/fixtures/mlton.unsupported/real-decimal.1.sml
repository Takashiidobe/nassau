(* mlton regression/real-decimal.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(*
 * The Basis Library description for fromDecimal in signature REAL states:
 *
 *   ... If class is NAN, a signed NaN is generated.
 *
 * However, it appears that the generated NaN always has the sign bit cleared.
 *)

open Real;

val minusIEEENan = {class = IEEEReal.NAN, sign = true, digits = [], exp = 0};

app print [
  "minusIEEENan = ", IEEEReal.toString minusIEEENan, "\n",

  "signBit (valOf (fromDecimal minusIEEENan)) = ",
  Bool.toString
  (signBit (valOf (fromDecimal minusIEEENan))),
  "\n"

];
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: minusIEEENan = ~nan *)
(* CHECK-STDOUT-NEXT: signBit (valOf (fromDecimal minusIEEENan)) = true *)
