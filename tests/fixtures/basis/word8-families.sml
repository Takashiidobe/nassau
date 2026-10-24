fun w n = Word8.fromInt n
fun sum v = Word8Vector.foldl (fn (x, a) => a + Word8.toInt x) 0 v

val v = Word8Vector.fromList [w 65, w 66, w 67, w 68]
val a = Word8Array.tabulate (6, fn i => w (i * 10))
val () = Word8Array.update (a, 1, w 99)
val () = Word8Array.copyVec {src = v, dst = a, di = 2}
val frozen = Word8Array.vector a
val slice = Word8VectorSlice.slice (v, 1, SOME 2)
val aslice = Word8ArraySlice.slice (a, 1, NONE)
val bytes = Byte.stringToBytes "AB"
val packed = Word8Array.array (4, w 45)
val () = Byte.packString (packed, 1, Substring.substring ("xyz", 1, 2))
val caught = (Word8Vector.sub (v, 4); "none") handle Subscript => "subscript"

val _ = print (Int.toString (sum v) ^ "\n")
val _ = print (Int.toString (Word8Array.length a) ^ "\n")
val _ = print (Int.toString (sum frozen) ^ "\n")
val _ = print (Int.toString (Word8Vector.length (Word8VectorSlice.vector slice)) ^ "\n")
val _ = print (Int.toString (Word8ArraySlice.length aslice) ^ "\n")
val _ = print (Int.toString (Word8.toInt (Word8Vector.sub (bytes, 1))) ^ "\n")
val _ = print (Byte.bytesToString bytes ^ "\n")
val _ = print (Byte.bytesToString (Word8Array.vector packed) ^ "\n")
val _ = print (Byte.unpackStringVec slice ^ "|" ^ Byte.unpackString aslice ^ "\n")
val _ = print (caught ^ "\n")
(* CHECK-STDOUT: 266 *)
(* CHECK-STDOUT-NEXT: 6 *)
(* CHECK-STDOUT-NEXT: 365 *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: 5 *)
(* CHECK-STDOUT-NEXT: 66 *)
(* CHECK-STDOUT-NEXT: AB *)
(* CHECK-STDOUT-NEXT: -yz- *)
(* CHECK-STDOUT-NEXT: BC|cABCD *)
(* CHECK-STDOUT-NEXT: subscript *)
(* CHECK-EXIT: 0 *)
