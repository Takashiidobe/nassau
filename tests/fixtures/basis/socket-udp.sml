fun bytes s = Word8VectorSlice.full (Byte.stringToBytes s)
fun check true = ()
  | check false = raise Fail "UDP loopback"

val loopback = valOf (NetHostDB.fromString "127.0.0.1")
val receiver = INetSock.UDP.socket ()
val sender = INetSock.UDP.socket' 17
val () = Socket.bind (receiver, INetSock.toAddr (loopback, 0))
val () = Socket.bind (sender, INetSock.toAddr (loopback, 0))
val target = Socket.Ctl.getSockName receiver
val source = Socket.Ctl.getSockName sender
val () = check (#2 (INetSock.fromAddr target) > 0)
val () = check (not (Option.isSome (Socket.recvVecFromNB (receiver, 8))))
val () = check (Socket.sendVecToNB (sender, target, bytes "A\000\255\nZ"))
val (peeked, peer) = Socket.recvVecFrom' (receiver, 3, {peek = true, oob = false})
val () = check (Byte.bytesToString peeked = "A\000\255" andalso Socket.sameAddr (peer, source))
val (whole, peer) = valOf (Socket.recvVecFromNB (receiver, 8))
val () = check (Byte.bytesToString whole = "A\000\255\nZ" andalso Socket.sameAddr (peer, source))

val data = Word8Array.fromList (List.map Word8.fromInt [0, 1, 2, 3, 4, 5])
val buffer = Word8Array.array (4, Word8.fromInt 9)
val () = Socket.sendArrTo (sender, target, Word8ArraySlice.slice (data, 1, SOME 4))
val (count, peer) = Socket.recvArrFrom (receiver, Word8ArraySlice.slice (buffer, 1, SOME 2))
val () = check (count = 2 andalso Socket.sameAddr (peer, source))
val () = check (Word8Array.foldr (fn (w, ws) => Word8.toInt w :: ws) [] buffer = [9, 1, 2, 9])
val () = check (not (Option.isSome (Socket.recvArrFromNB (receiver, Word8ArraySlice.full buffer))))

val () = Socket.connect (sender, target)
val () = Socket.connect (receiver, source)
val () = check (Socket.sameAddr (Socket.Ctl.getPeerName sender, target))
val () = Socket.sendVecTo (sender, target, bytes "")
val (empty, peer) = Socket.recvVecFrom (receiver, 1)
val () = check (Word8Vector.length empty = 0 andalso Socket.sameAddr (peer, source))
val () = check ((Socket.recvVecFrom (receiver, ~1); false) handle Size => true)
val () = Socket.close receiver
val () = check ((Socket.recvVecFromNB (receiver, 1); false) handle OS.SysErr _ => true)
val () = Socket.close sender
val () = print "udp loopback\n"

(* CHECK-STDOUT: udp loopback *)
(* CHECK-EXIT: 0 *)
(* WASM-SKIP: WebAssembly has no sockets *)
