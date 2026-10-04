# <span id="section:0"></span>The `INetSock` structure

---

#### Synopsis

<span id="INET_SOCK:SIG:SPEC"></span>
<span id="INetSock:STR:SPEC"></span>

```sml
signature INET_SOCK (* OPTIONAL *)
structure INetSock :> INET_SOCK (* OPTIONAL *)
```

This structure provides operations for creating and manipulating Internet-domain addresses and sockets.

---

#### Interface

<span id="SIG:INET_SOCK.inet:TY:SPEC"></span>
<span id="SIG:INET_SOCK.sock:TY:SPEC"></span>
<span id="SIG:INET_SOCK.stream_sock:TY:SPEC"></span>
<span id="SIG:INET_SOCK.dgram_sock:TY:SPEC"></span>
<span id="SIG:INET_SOCK.sock_addr:TY:SPEC"></span>
<span id="SIG:INET_SOCK.inetAF:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.toAddr:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.fromAddr:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.any:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.socket:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.socket':VAL:SPEC"></span>
<span id="SIG:INET_SOCK.getNODELAY:VAL:SPEC"></span>
<span id="SIG:INET_SOCK.setNODELAY:VAL:SPEC"></span>

```sml
type inet
type 'sock_type sock = (inet, 'sock_type) Socket.sock
type 'mode stream_sock = 'mode Socket.stream sock
type dgram_sock = Socket.dgram sock
type sock_addr = inet Socket.sock_addr
val inetAF : Socket.AF.addr_family
val toAddr : NetHostDB.in_addr * int -> sock_addr
val fromAddr : sock_addr -> NetHostDB.in_addr * int
val any : int -> sock_addr
structure UDP : sig
val socket : unit -> dgram_sock
val socket' : int -> dgram_sock
end
structure TCP : sig
val socket : unit -> 'mode stream_sock
val socket' : int -> 'mode stream_sock
val getNODELAY : 'mode stream_sock -> bool
val setNODELAY : 'mode stream_sock * bool -> unit
end
```

#### Description

<span id="SIG:INET_SOCK.inet:TY"></span>**`type`**` inet`  
The witness type of the INet address family.

<span id="SIG:INET_SOCK.stream_sock:TY"></span>**`type`**` `_`'mode`_` stream_sock = `_`'mode`_` Socket.stream sock`  
The type-scheme of Internet-domain stream sockets; The type parameter `'mode` can be instantiated to either [`Socket.active`](socket.md#SIG:SOCKET.active:TY:SPEC) or [`Socket.passive`](socket.md#SIG:SOCKET.passive:TY:SPEC).

<span id="SIG:INET_SOCK.dgram_sock:TY"></span>**`type`**` dgram_sock = Socket.dgram sock`  
The type of Internet-domain datagram sockets.

<span id="SIG:INET_SOCK.sock_addr:TY"></span>**`type`**` sock_addr = inet Socket.sock_addr`  
The type of Internet-domain socket addresses.

<span id="SIG:INET_SOCK.inetAF:VAL"></span>**`val`**` inetAF `**`:`**` Socket.AF.addr_family`  
The address family value that represents the Internet domain.

<span id="SIG:INET_SOCK.toAddr:VAL"></span>
`toAddr (``ia``, ``i``) `  
converts an Internet address `ia` and a port number `i` into a socket address (in the INet address family).

<span id="SIG:INET_SOCK.fromAddr:VAL"></span>**`val`**` fromAddr `**`:`**` sock_addr `**`->`**` NetHostDB.in_addr `**`*`**` int`  
This function converts a socket address (in the INet address family) into a pair `(ia,i)` of an Internet address `ia` and a port number `i`.

<span id="SIG:INET_SOCK.any:VAL"></span>
`any ``port`` `  
creates a socket address that fixes the port to `port`, but leaves the Internet address unspecified. This function corresponds to the `INADDR_ANY` constant in the C Sockets API. The values created by this function are used to [`bind`](socket.md#SIG:SOCKET.bind:VAL:SPEC) a socket to a specific port.

<span id="SIG:INET_SOCK.UDP:STR"></span>
**`structure`**` UDP`  

<span id="SIG:INET_SOCK.UDP.socket:VAL"></span>**`val`**` socket `**`:`**` unit `**`->`**` dgram_sock`  
This creates a datagram socket in the INet address family with the default protocol. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:INET_SOCK.UDP.socket':VAL"></span>
`socket' ``prot`` `  
creates a datagram socket in the INet address family with the protocol number `prot`. The interpretation of `prot` is system dependent, but a value of `0` is equivalent to [`socket`](inet-sock.md#SIG:INET_SOCK.UDP.socket:VAL:SPEC)`()`. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:INET_SOCK.TCP:STR"></span>
**`structure`**` TCP`  

<span id="SIG:INET_SOCK.TCP.socket:VAL"></span>**`val`**` socket `**`:`**` unit `**`->`**` `_`'mode`_` stream_sock`  
This creates a stream socket in the INet address family with the default protocol. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:INET_SOCK.TCP.socket':VAL"></span>
`socket' ``prot`` `  
creates a stream socket in the INet address family with the protocol number `prot`. The interpretation of `prot` is system dependent, but a value of `0` is equivalent to [`socket`](inet-sock.md#SIG:INET_SOCK.TCP.socket:VAL:SPEC)`()`.

<span id="SIG:INET_SOCK.TCP.getNODELAY:VAL"></span>**`val`**` getNODELAY `**`:`**` `_`'mode`_` stream_sock `**`->`**` bool`
**`val`**` setNODELAY `**`:`**` `_`'mode`_` stream_sock `**`*`**` bool `**`->`**` unit`  
These functions query and set the `TCP_NODELAY` flag on the socket. When set to `false` (the default), there is only a single small packet allowed to be outstanding on a given TCP connection at any time, thereby reducing small packet traffic on slower WANs. When set to `true`, packets are sent as fast as possible. \[\[Refer to Stevens, p.316.\]\]

#### Examples

```repl
INetSock.TCP.socket ();;
```

#### See Also

> [`GenericSock`](generic-sock.md#GenericSock:STR:SPEC), [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC), [`Socket`](socket.md#Socket:STR:SPEC), [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)
