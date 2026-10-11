val empty : int Array.array = Array.fromList []
val zero : string Array.array = Array.tabulate (0, fn _ => raise Fail "evaluated")
val visits : int list ref = ref []
val matrix = Array2.tabulate Array2.ColMajor (2, 3, fn (i, j) =>
  (visits := (i * 3 + j) :: !visits; i * 3 + j))
val _ = print (if Array.length empty = 0 andalso Array.length zero = 0
  andalso Array2.sub (matrix, 1, 2) = 5
  andalso List.rev (!visits) = [0, 3, 1, 4, 2, 5]
  andalso OS.Path.validVolume {isAbs = false, vol = ""}
  andalso OS.Path.getParent "/" = "/"
  andalso Substring.string (#2 (valOf (Substring.getc (Substring.full "ab")))) = "b"
  andalso Word8.toInt (Word8.~ (Word8.fromInt 1)) = 255
  andalso Real.== (Real.abs (~0.0), 0.0)
  andalso Real.sign (~1.0) = ~1
  andalso (Char.chr 256; false) handle General.Chr => true
  then "loaded\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: loaded *)
