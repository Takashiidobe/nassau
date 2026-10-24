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

  fun addressOf (h, peer) =
    let
      val reply = Prim.sockName (h, peer)
    in
      if size reply > 0 andalso String.sub (reply, 0) = #"=" then String.extract (reply, 1, NONE)
      else raise OS.SysErr (OS.errorMsg (valOf (Int.fromString reply)), Int.fromString reply)
    end

  fun accept (SOCK h) =
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

  structure Ctl =
  struct
    fun getSockName (SOCK h) = ADDR (addressOf (h, 0))
    fun getPeerName (SOCK h) = ADDR (addressOf (h, 1))
  end
end

structure INetSock =
struct
  type inet = unit
  type 'a sock = (inet, 'a) Socket.sock
  type 'a stream_sock = inet Socket.stream sock
  type dgram_sock = Socket.dgram sock
  type sock_addr = inet Socket.sock_addr

  fun toAddr (addr, port) =
    Socket.ADDR ("inet:" ^ NetHostDB.toString addr ^ ":" ^ Int.toString port)

  fun fromAddr (Socket.ADDR text) =
    let
      val body = String.extract (text, 5, NONE)
      val cut = valOf (List.find (fn i => String.sub (body, i) = #":")
                         (List.tabulate (size body, fn i => size body - 1 - i)))
    in
      (String.substring (body, 0, cut), valOf (Int.fromString (String.extract (body, cut + 1, NONE))))
    end

  fun any port = toAddr (NetHostDB.any, port)

  structure TCP =
  struct
    fun socket () = Socket.SOCK (Socket.check (Prim.sockCreate ()))
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
