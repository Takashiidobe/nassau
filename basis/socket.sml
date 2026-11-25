structure NetHostDB =
struct
  type in_addr = string

  fun toString (addr: in_addr) = addr
  val any : in_addr = "0.0.0.0"

  fun fromString s =
    let
      val parts = String.fields (fn c => c = #".") s
      fun octet p =
        case Int.fromString p of
          SOME n => n >= 0 andalso n < 256 andalso p <> "" andalso Char.isDigit (String.sub (p, 0))
        | NONE => false
    in
      if length parts = 4 andalso List.all octet parts then SOME s else NONE
    end
end

structure Socket =
struct
  datatype ('af, 'sock_type) sock = SOCK of int
  datatype 'af sock_addr = ADDR of string
  datatype dgram = DGRAM
  datatype 'a stream = STREAM
  datatype passive = PASSIVE
  datatype active = ACTIVE
  datatype shutdown_mode = NO_RECVS | NO_SENDS | NO_RECVS_OR_SENDS

  fun check result =
    if result < 0 then raise OS.SysErr (OS.errorMsg (~result), SOME (~result)) else result

  fun sameDesc (SOCK a, SOCK b) = a = b
  fun ioDesc (SOCK h) = OS.IO.ofHandle h
  fun sameAddr (ADDR a, ADDR b) = a = b

  fun bind (SOCK h, ADDR a) = (check (Prim.sockBind (h, a)); ())
  fun listen (SOCK h, _: int) = (check (Prim.sockListen h); ())
  fun connect (SOCK h, ADDR a) = (check (Prim.sockConnect (h, a)); ())

  fun checkedText reply =
    if size reply > 0 andalso String.sub (reply, 0) = #"=" then String.extract (reply, 1, NONE)
    else raise OS.SysErr (OS.errorMsg (valOf (Int.fromString reply)), Int.fromString reply)

  fun addressOf (h, peer) = checkedText (Prim.sockName (h, peer))

  fun accept (SOCK h : ('af, 'kind) sock) : ('af, active stream) sock * 'af sock_addr =
    let val fresh = check (Prim.sockAccept h)
    in (SOCK fresh, ADDR (addressOf (fresh, 1))) end

  fun close (SOCK h) = (Prim.osClose h; ())

  fun shutdown (SOCK h, mode) =
    let
      val how =
        case mode of
          NO_RECVS => 0
        | NO_SENDS => 1
        | NO_RECVS_OR_SENDS => 2
    in
      (check (Prim.sockShutdown (h, how)); ())
    end

  fun sendVec (SOCK h, slice) =
    let val data = Byte.unpackStringVec slice
    in (check (Prim.osWrite (h, data)); size data) end

  fun recvVec (SOCK h, n) = Byte.stringToBytes (Prim.osRead (h, n))

  type out_flags = {don't_route: bool, oob: bool}
  type in_flags = {peek: bool, oob: bool}

  local
    val defaultOut = {don't_route = false, oob = false}
    val defaultIn = {peek = false, oob = false}

    fun flagBits (oob, secondary, nonblocking) =
      (if oob then 1 else 0) + (if secondary then 2 else 0) + (if nonblocking then 4 else 0)

    fun sendTo (SOCK h : ('af, dgram) sock, ADDR a : 'af sock_addr,
                data, {don't_route, oob}, nonblocking) =
      check (Prim.udpSendTo (h, a, data, flagBits (oob, don't_route, nonblocking))) <> 0

    fun recvFrom (SOCK h : ('af, dgram) sock, n, {peek, oob}, nonblocking)
        : (Substring.substring * 'af sock_addr) option =
      if n < 0 orelse n > Word8Vector.maxLen then raise Size
      else
        let val reply = Prim.udpRecvFrom (h, n, flagBits (oob, peek, nonblocking))
        in
          if reply = "" then NONE
          else
            let
              val (peer, rest) = Substring.splitl (fn c => c <> #"\n") (Substring.full (checkedText reply))
            in
              SOME (Substring.triml 1 rest, ADDR (Substring.string peer))
            end
        end

    fun toVector (data, peer) = (Byte.stringToBytes (Substring.string data), peer)

    fun toArray slice (data, peer) =
      let val (array, start, _) = Word8ArraySlice.base slice
      in (Byte.packString (array, start, data); (Substring.size data, peer)) end
  in
    fun sendVecTo' (sock, addr, slice, flags) =
      (sendTo (sock, addr, Byte.unpackStringVec slice, flags, false); ())
    fun sendVecTo (sock, addr, slice) = sendVecTo' (sock, addr, slice, defaultOut)
    fun sendArrTo' (sock, addr, slice, flags) =
      (sendTo (sock, addr, Byte.unpackString slice, flags, false); ())
    fun sendArrTo (sock, addr, slice) = sendArrTo' (sock, addr, slice, defaultOut)

    fun sendVecToNB' (sock, addr, slice, flags) =
      sendTo (sock, addr, Byte.unpackStringVec slice, flags, true)
    fun sendVecToNB (sock, addr, slice) = sendVecToNB' (sock, addr, slice, defaultOut)
    fun sendArrToNB' (sock, addr, slice, flags) =
      sendTo (sock, addr, Byte.unpackString slice, flags, true)
    fun sendArrToNB (sock, addr, slice) = sendArrToNB' (sock, addr, slice, defaultOut)

    fun recvVecFrom' (sock, n, flags) = toVector (valOf (recvFrom (sock, n, flags, false)))
    fun recvVecFrom (sock, n) = recvVecFrom' (sock, n, defaultIn)
    fun recvArrFrom' (sock, slice, flags) =
      toArray slice (valOf (recvFrom (sock, Word8ArraySlice.length slice, flags, false)))
    fun recvArrFrom (sock, slice) = recvArrFrom' (sock, slice, defaultIn)

    fun recvVecFromNB' (sock, n, flags) = Option.map toVector (recvFrom (sock, n, flags, true))
    fun recvVecFromNB (sock, n) = recvVecFromNB' (sock, n, defaultIn)
    fun recvArrFromNB' (sock, slice, flags) =
      Option.map (toArray slice) (recvFrom (sock, Word8ArraySlice.length slice, flags, true))
    fun recvArrFromNB (sock, slice) = recvArrFromNB' (sock, slice, defaultIn)
  end

  structure Ctl =
  struct
    fun getSockName (SOCK h : ('af, 'kind) sock) : 'af sock_addr = ADDR (addressOf (h, 0))
    fun getPeerName (SOCK h : ('af, 'kind) sock) : 'af sock_addr = ADDR (addressOf (h, 1))
  end
end

structure INetSock =
struct
  type inet = unit
  type 'a sock = (inet, 'a) Socket.sock
  type 'a stream_sock = 'a Socket.stream sock
  type dgram_sock = Socket.dgram sock
  type sock_addr = inet Socket.sock_addr

  fun toAddr (addr, port) : sock_addr =
    Socket.ADDR ("inet:" ^ NetHostDB.toString addr ^ ":" ^ Int.toString port)

  fun fromAddr (Socket.ADDR text : sock_addr) =
    let
      val body = String.extract (text, 5, NONE)
      val cut = valOf (List.find (fn i => String.sub (body, i) = #":")
                         (List.tabulate (size body, fn i => size body - 1 - i)))
    in
      (String.substring (body, 0, cut), valOf (Int.fromString (String.extract (body, cut + 1, NONE))))
    end

  fun any port = toAddr (NetHostDB.any, port)

  structure UDP =
  struct
    fun socket' protocol : dgram_sock = Socket.SOCK (Socket.check (Prim.udpCreate protocol))
    fun socket () = socket' 0
  end

  structure TCP =
  struct
    fun socket () : 'mode stream_sock = Socket.SOCK (Socket.check (Prim.sockCreate ()))
  end
end

structure UnixSock =
struct
  type unix = unit
  type 'a sock = (unix, 'a) Socket.sock
  type 'a stream_sock = unix Socket.stream sock
  type sock_addr = unix Socket.sock_addr

  fun toAddr path = Socket.ADDR ("unix:" ^ path)
  fun fromAddr (Socket.ADDR text) = String.extract (text, 5, NONE)

  structure Strm =
  struct
    fun socket () = Socket.SOCK (Socket.check (Prim.sockCreate ()))

    fun socketPair () =
      let val packed = Socket.check (Prim.sockPair ())
      in (Socket.SOCK (packed div 16384), Socket.SOCK (packed mod 16384)) end
  end
end
