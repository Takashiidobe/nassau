# <span id="section:0"></span>The `Socket` structure

---

#### Synopsis

<span id="SOCKET:SIG:SPEC"></span>
<span id="Socket:STR:SPEC"></span>

```sml
signature SOCKET (* OPTIONAL *)
structure Socket :> SOCKET (* OPTIONAL *)
```

This structure provides the standard socket types, socket management, and I/O operations. The creation of sockets is relegated to domain-specific structures (such as [`INetSock`](inet-sock.md#INetSock:STR:SPEC) and [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)).

---

#### Interface

<span id="SIG:SOCKET.sock:TY:SPEC"></span>
<span id="SIG:SOCKET.sock_addr:TY:SPEC"></span>
<span id="SIG:SOCKET.dgram:TY:SPEC"></span>
<span id="SIG:SOCKET.stream:TY:SPEC"></span>
<span id="SIG:SOCKET.passive:TY:SPEC"></span>
<span id="SIG:SOCKET.active:TY:SPEC"></span>
<span id="SIG:SOCKET.addr_family:TY:SPEC"></span>
<span id="SIG:SOCKET.list:VAL:SPEC"></span>
<span id="SIG:SOCKET.toString:VAL:SPEC"></span>
<span id="SIG:SOCKET.fromString:VAL:SPEC"></span>
<span id="SIG:SOCKET.sock_type:TY:SPEC"></span>
<span id="SIG:SOCKET.stream:VAL:SPEC"></span>
<span id="SIG:SOCKET.dgram:VAL:SPEC"></span>
<span id="SIG:SOCKET.getDEBUG:VAL:SPEC"></span>
<span id="SIG:SOCKET.setDEBUG:VAL:SPEC"></span>
<span id="SIG:SOCKET.getREUSEADDR:VAL:SPEC"></span>
<span id="SIG:SOCKET.setREUSEADDR:VAL:SPEC"></span>
<span id="SIG:SOCKET.getKEEPALIVE:VAL:SPEC"></span>
<span id="SIG:SOCKET.setKEEPALIVE:VAL:SPEC"></span>
<span id="SIG:SOCKET.getDONTROUTE:VAL:SPEC"></span>
<span id="SIG:SOCKET.setDONTROUTE:VAL:SPEC"></span>
<span id="SIG:SOCKET.getLINGER:VAL:SPEC"></span>
<span id="SIG:SOCKET.setLINGER:VAL:SPEC"></span>
<span id="SIG:SOCKET.getBROADCAST:VAL:SPEC"></span>
<span id="SIG:SOCKET.setBROADCAST:VAL:SPEC"></span>
<span id="SIG:SOCKET.getOOBINLINE:VAL:SPEC"></span>
<span id="SIG:SOCKET.setOOBINLINE:VAL:SPEC"></span>
<span id="SIG:SOCKET.getSNDBUF:VAL:SPEC"></span>
<span id="SIG:SOCKET.setSNDBUF:VAL:SPEC"></span>
<span id="SIG:SOCKET.getRCVBUF:VAL:SPEC"></span>
<span id="SIG:SOCKET.setRCVBUF:VAL:SPEC"></span>
<span id="SIG:SOCKET.getTYPE:VAL:SPEC"></span>
<span id="SIG:SOCKET.getERROR:VAL:SPEC"></span>
<span id="SIG:SOCKET.getPeerName:VAL:SPEC"></span>
<span id="SIG:SOCKET.getSockName:VAL:SPEC"></span>
<span id="SIG:SOCKET.getNREAD:VAL:SPEC"></span>
<span id="SIG:SOCKET.getATMARK:VAL:SPEC"></span>
<span id="SIG:SOCKET.sameAddr:VAL:SPEC"></span>
<span id="SIG:SOCKET.familyOfAddr:VAL:SPEC"></span>
<span id="SIG:SOCKET.bind:VAL:SPEC"></span>
<span id="SIG:SOCKET.listen:VAL:SPEC"></span>
<span id="SIG:SOCKET.accept:VAL:SPEC"></span>
<span id="SIG:SOCKET.acceptNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.connect:VAL:SPEC"></span>
<span id="SIG:SOCKET.connectNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.close:VAL:SPEC"></span>
<span id="SIG:SOCKET.shutdown_mode:TY:SPEC"></span>
<span id="SIG:SOCKET.NO_RECVS:TY:SPEC"></span>
<span id="SIG:SOCKET.NO_SENDS:TY:SPEC"></span>
<span id="SIG:SOCKET.NO_RECVS_OR_SENDS:TY:SPEC"></span>
<span id="SIG:SOCKET.shutdown:VAL:SPEC"></span>
<span id="SIG:SOCKET.sock_desc:TY:SPEC"></span>
<span id="SIG:SOCKET.sockDesc:VAL:SPEC"></span>
<span id="SIG:SOCKET.sameDesc:VAL:SPEC"></span>
<span id="SIG:SOCKET.select:VAL:SPEC"></span>
<span id="SIG:SOCKET.ioDesc:VAL:SPEC"></span>
<span id="SIG:SOCKET.out_flags:TY:SPEC"></span>
<span id="SIG:SOCKET.in_flags:TY:SPEC"></span>
<span id="SIG:SOCKET.sendVec:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArr:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVec':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArr':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVec:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVec':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArr:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArr':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecTo:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrTo:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecTo':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrTo':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecToNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendVecToNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrToNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.sendArrToNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecFrom:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecFrom':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrFrom:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrFrom':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecFromNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvVecFromNB':VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrFromNB:VAL:SPEC"></span>
<span id="SIG:SOCKET.recvArrFromNB':VAL:SPEC"></span>

```sml
type ('af,'sock_type) sock
type 'af sock_addr
type dgram
type 'mode stream
type passive
type active
structure AF : sig
type addr_family = NetHostDB.addr_family
val list : unit -> (string * addr_family) list
val toString : addr_family -> string
val fromString : string -> addr_family option
end
structure SOCK : sig
eqtype sock_type
val stream : sock_type
val dgram : sock_type
val list : unit -> (string * sock_type) list
val toString : sock_type -> string
val fromString : string -> sock_type option
end
structure Ctl : sig
val getDEBUG : ('af, 'sock_type) sock -> bool
val setDEBUG : ('af, 'sock_type) sock * bool -> unit
val getREUSEADDR : ('af, 'sock_type) sock -> bool
val setREUSEADDR : ('af, 'sock_type) sock * bool -> unit
val getKEEPALIVE : ('af, 'sock_type) sock -> bool
val setKEEPALIVE : ('af, 'sock_type) sock * bool -> unit
val getDONTROUTE : ('af, 'sock_type) sock -> bool
val setDONTROUTE : ('af, 'sock_type) sock * bool -> unit
val getLINGER : ('af, 'sock_type) sock -> Time.time option
val setLINGER : ('af, 'sock_type) sock
* Time.time option -> unit
val getBROADCAST : ('af, 'sock_type) sock -> bool
val setBROADCAST : ('af, 'sock_type) sock * bool -> unit
val getOOBINLINE : ('af, 'sock_type) sock -> bool
val setOOBINLINE : ('af, 'sock_type) sock * bool -> unit
val getSNDBUF : ('af, 'sock_type) sock -> int
val setSNDBUF : ('af, 'sock_type) sock * int -> unit
val getRCVBUF : ('af, 'sock_type) sock -> int
val setRCVBUF : ('af, 'sock_type) sock * int -> unit
val getTYPE : ('af, 'sock_type) sock -> SOCK.sock_type
val getERROR : ('af, 'sock_type) sock -> bool
val getPeerName : ('af, 'sock_type) sock -> 'af sock_addr
val getSockName : ('af, 'sock_type) sock -> 'af sock_addr
val getNREAD : ('af, 'sock_type) sock -> int
val getATMARK : ('af, active stream) sock -> bool
end
val sameAddr : 'af sock_addr * 'af sock_addr -> bool
val familyOfAddr : 'af sock_addr -> AF.addr_family
val bind : ('af, 'sock_type) sock * 'af sock_addr -> unit
val listen : ('af, passive stream) sock * int -> unit
val accept : ('af, passive stream) sock -> ('af, active stream) sock * 'af sock_addr
val acceptNB : ('af, passive stream) sock -> (('af, active stream) sock
* 'af sock_addr) option
val connect : ('af, 'sock_type) sock * 'af sock_addr -> unit
val connectNB : ('af, 'sock_type) sock * 'af sock_addr -> bool
val close : ('af, 'sock_type) sock -> unit
datatype shutdown_mode
= NO_RECVS
| NO_SENDS
| NO_RECVS_OR_SENDS
val shutdown : ('af, 'mode stream) sock * shutdown_mode -> unit
type sock_desc
val sockDesc : ('af, 'sock_type) sock -> sock_desc
val sameDesc : sock_desc * sock_desc -> bool
val select : {
rds : sock_desc list,
wrs : sock_desc list,
exs : sock_desc list,
timeout : Time.time option
} -> {
rds : sock_desc list,
wrs : sock_desc list,
exs : sock_desc list
}
val ioDesc : ('af, 'sock_type) sock -> OS.IO.iodesc
type out_flags = {don't_route : bool, oob : bool}
type in_flags = {peek : bool, oob : bool}
val sendVec : ('af, active stream) sock
* Word8VectorSlice.slice -> int
val sendArr : ('af, active stream) sock
* Word8ArraySlice.slice -> int
val sendVec' : ('af, active stream) sock
* Word8VectorSlice.slice
* out_flags -> int
val sendArr' : ('af, active stream) sock
* Word8ArraySlice.slice
* out_flags -> int
val sendVecNB : ('af, active stream) sock
* Word8VectorSlice.slice -> int option
val sendVecNB' : ('af, active stream) sock
* Word8VectorSlice.slice
* out_flags -> int option
val sendArrNB : ('af, active stream) sock
* Word8ArraySlice.slice -> int option
val sendArrNB' : ('af, active stream) sock
* Word8ArraySlice.slice
* out_flags -> int option
val recvVec : ('af, active stream) sock * int -> Word8Vector.vector
val recvVec' : ('af, active stream) sock * int * in_flags -> Word8Vector.vector
val recvArr : ('af, active stream) sock
* Word8ArraySlice.slice -> int
val recvArr' : ('af, active stream) sock
* Word8ArraySlice.slice
* in_flags -> int
val recvVecNB : ('af, active stream) sock * int -> Word8Vector.vector option
val recvVecNB' : ('af, active stream) sock * int * in_flags -> Word8Vector.vector option
val recvArrNB : ('af, active stream) sock
* Word8ArraySlice.slice -> int option
val recvArrNB' : ('af, active stream) sock
* Word8ArraySlice.slice
* in_flags -> int option
val sendVecTo : ('af, dgram) sock
* 'af sock_addr
* Word8VectorSlice.slice -> unit
val sendArrTo : ('af, dgram) sock
* 'af sock_addr
* Word8ArraySlice.slice -> unit
val sendVecTo' : ('af, dgram) sock
* 'af sock_addr
* Word8VectorSlice.slice
* out_flags -> unit
val sendArrTo' : ('af, dgram) sock
* 'af sock_addr
* Word8ArraySlice.slice
* out_flags -> unit
val sendVecToNB : ('af, dgram) sock
* 'af sock_addr
* Word8VectorSlice.slice -> bool
val sendVecToNB' : ('af, dgram) sock
* 'af sock_addr
* Word8VectorSlice.slice
* out_flags -> bool
val sendArrToNB : ('af, dgram) sock
* 'af sock_addr
* Word8ArraySlice.slice -> bool
val sendArrToNB' : ('af, dgram) sock
* 'af sock_addr
* Word8ArraySlice.slice
* out_flags -> bool
val recvVecFrom : ('af, dgram) sock * int -> Word8Vector.vector
* 'sock_type sock_addr
val recvVecFrom' : ('af, dgram) sock * int * in_flags -> Word8Vector.vector
* 'sock_type sock_addr
val recvArrFrom : ('af, dgram) sock
* Word8ArraySlice.slice -> int * 'af sock_addr
val recvArrFrom' : ('af, dgram) sock
* Word8ArraySlice.slice
* in_flags -> int * 'af sock_addr
val recvVecFromNB : ('af, dgram) sock * int -> (Word8Vector.vector
* 'sock_type sock_addr) option
val recvVecFromNB' : ('af, dgram) sock * int * in_flags -> (Word8Vector.vector
* 'sock_type sock_addr) option
val recvArrFromNB : ('af, dgram) sock
* Word8ArraySlice.slice -> (int * 'af sock_addr) option
val recvArrFromNB' : ('af, dgram) sock
* Word8ArraySlice.slice
* in_flags -> (int * 'af sock_addr) option
```

#### Description

<span id="SIG:SOCKET.sock:TY"></span>**`type`**` (`_`'af`_`,`_`'sock_type`_`) sock`  
The type of a socket. Sockets are polymorphic over both the address family and the socket type. The type parameter `'af` is instantiated with the appropriate address family type ([`INetSock.inet`](inet-sock.md#SIG:INET_SOCK.inet:TY:SPEC) or [`UnixSock.unix`](unix-sock.md#SIG:UNIX_SOCK.unix:TY:SPEC)). The type parameter `'sock_type` is instantiated with the appropriate socket type ([`dgram`](socket.md#SIG:SOCKET.dgram:TY:SPEC) or [`stream`](socket.md#SIG:SOCKET.stream:TY:SPEC)).

<span id="SIG:SOCKET.sock_addr:TY"></span>**`type`**` `_`'af`_` sock_addr`  
The type of a socket address. The type parameter `'af` describes the address family of the address ([`INetSock.inet`](inet-sock.md#SIG:INET_SOCK.inet:TY:SPEC) or [`UnixSock.unix`](unix-sock.md#SIG:UNIX_SOCK.unix:TY:SPEC)).

<span id="SIG:SOCKET.dgram:TY"></span>**`type`**` dgram`  
The witness type for datagram sockets.

<span id="SIG:SOCKET.stream:TY"></span>**`type`**` `_`'mode`_` stream`  
The witness type for stream sockets. The type parameter `'mode` describes the mode of the stream socket: active or passive.

<span id="SIG:SOCKET.AF:STR"></span>
**`structure`**` AF`  
The [`AF`](socket.md#SIG:SOCKET.AF:STR:SPEC) substructure defines an abstract type that represents the different network-address families.

<span id="SIG:SOCKET.AF.list:VAL"></span>**`val`**` list `**`:`**` unit `**`->`**` (string `**`*`**` addr_family) list`  
This returns a list of all the available address families. Every element of the list is a pair `(``name``,``af``)` where `name` is the name of the address family, and `af` is the actual address family value.

The names of the address families are taken from the symbolic constants used in the C Socket API and stripping the leading \`\``AF_`.'' For example, the Unix-domain address family is named `"UNIX"`, the Internet-domain address family is named `"INET"`, and the _Apple Talk_ address family is named `"APPLETALK"`.

<span id="SIG:SOCKET.AF.toString:VAL"></span>**`val`**` toString `**`:`**` addr_family `**`->`**` string`
**`val`**` fromString `**`:`**` string `**`->`**` addr_family option`  
These convert between address family values and their names. For example, the expression `toString (INetSock.inetAF)` returns the string `"INET"`. [`fromString`](socket.md#SIG:SOCKET.AF.fromString:VAL:SPEC) returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if no family value corresponds to the given name.

If a pair `(``name``,``af``)` is in the list returned by [`list`](socket.md#SIG:SOCKET.AF.list:VAL:SPEC), then it is the case that `name` is equal to `toString(``af``)`.

<span id="SIG:SOCKET.SOCK:STR"></span>
**`structure`**` SOCK`  
The [`SOCK`](socket.md#SIG:SOCKET.SOCK:STR:SPEC) substructure provides an abstract type and operations for the different types of sockets. This type is used by the [`getTYPE`](socket.md#SIG:SOCKET.Ctl.getTYPE:VAL:SPEC) function.

<span id="SIG:SOCKET.SOCK.sock_type:TY"></span>**`eqtype`**` sock_type`  
The type of socket types.

<span id="SIG:SOCKET.SOCK.stream:VAL"></span>**`val`**` stream `**`:`**` sock_type`  
The stream socket type value.

<span id="SIG:SOCKET.SOCK.dgram:VAL"></span>**`val`**` dgram `**`:`**` sock_type`  
The datagram socket type value.

<span id="SIG:SOCKET.SOCK.list:VAL"></span>**`val`**` list `**`:`**` unit `**`->`**` (string `**`*`**` sock_type) list`  
A list of the available socket types. Every element of the list is of the form `(``name``,``sty``)` where `name` is the name of the socket type, and `sty` is the actual socket type value.

The list of possible socket type names includes `"STREAM"` for stream sockets, `"DGRAM"` for datagram sockets, and `"RAW"` for raw sockets. These names are formed by taking the symbolic constants from the C API and removing the leading \`\``SOCK_`.''

<span id="SIG:SOCKET.SOCK.toString:VAL"></span>**`val`**` toString `**`:`**` sock_type `**`->`**` string`
**`val`**` fromString `**`:`**` string `**`->`**` sock_type option`  
These convert between a socket type value and its name (_e.g._, "STREAM"). [`fromString`](socket.md#SIG:SOCKET.SOCK.fromString:VAL:SPEC) returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if no socket type value corresponds to the name.

If a pair `(``name``,``sty``)` is in the list returned by [`list`](socket.md#SIG:SOCKET.SOCK.list:VAL:SPEC), then it is the case that `name` is equal to `toString(``sty``)`.

<span id="SIG:SOCKET.Ctl:STR"></span>
**`structure`**` Ctl`  
The [`Ctl`](socket.md#SIG:SOCKET.Ctl:STR:SPEC) substructure provides support for manipulating the options associated with a socket. These functions raise the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception when the argument socket has been closed.

<span id="SIG:SOCKET.Ctl.getDEBUG:VAL"></span>**`val`**` getDEBUG `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setDEBUG `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These functions query and set the `SO_DEBUG` flag for the socket. This flag enables or disables low-level debugging within the kernel. Enabled, it allows the kernel to maintain a history of the recent packets that have been received or sent.

<span id="SIG:SOCKET.Ctl.getREUSEADDR:VAL"></span>**`val`**` getREUSEADDR `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setREUSEADDR `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These query and set the `SO_REUSEADDR` flag for the socket. When `true`, this flag instructs the system to allow reuse of local socket addresses in [`bind`](socket.md#SIG:SOCKET.bind:VAL:SPEC) calls.

<span id="SIG:SOCKET.Ctl.getKEEPALIVE:VAL"></span>**`val`**` getKEEPALIVE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setKEEPALIVE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These query and set the `SO_KEEPALIVE` flag for the socket. When `true`, the system will generate periodic transmissions on a connected socket, when no other data is being exchanged.

<span id="SIG:SOCKET.Ctl.getDONTROUTE:VAL"></span>**`val`**` getDONTROUTE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setDONTROUTE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These query and set the `SO_DONTROUTE` flag for the socket. When this flag is `true`, outgoing messages bypass the normal routing mechanisms of the underlying protocol, and are instead directed to the appropriate network interface as specified by the network portion of the destination address. Note that this option can be specified on a per message basis by using one of the [`sendVec'`](socket.md#SIG:SOCKET.sendVec':VAL:SPEC), [`sendArr'`](socket.md#SIG:SOCKET.sendArr':VAL:SPEC), [`sendVecTo'`](socket.md#SIG:SOCKET.sendVecTo':VAL:SPEC), or [`sendArrTo'`](socket.md#SIG:SOCKET.sendArrTo':VAL:SPEC) functions.

<span id="SIG:SOCKET.Ctl.getLINGER:VAL"></span>**`val`**` getLINGER `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` Time.time option`
**`val`**` setLINGER `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` Time.time option`
`                  `**`->`**` unit`  
These functions query and set the `SO_LINGER` flag for the socket `sock`. This flag controls the action taken when unsent messages are queued on socket and a [`close`](socket.md#SIG:SOCKET.close:VAL:SPEC) is performed. If the flag is set to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), then the system will close the socket as quickly as possible, discarding data if necessary. If the flag is set to [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``t``)` and the socket promises reliable delivery, then the system will block the [`close`](socket.md#SIG:SOCKET.close:VAL:SPEC) operation until the data is delivered or the timeout `t` expires. If `t` is negative or too large, then the [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) is raised.

<span id="SIG:SOCKET.Ctl.getBROADCAST:VAL"></span>**`val`**` getBROADCAST `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setBROADCAST `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These query and set the `SO_BROADCAST` flag for the socket `sock`, which enables or disables the ability of the process to send broadcast messages over the socket.

<span id="SIG:SOCKET.Ctl.getOOBINLINE:VAL"></span>**`val`**` getOOBINLINE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`
**`val`**` setOOBINLINE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` bool `**`->`**` unit`  
These query and set the `SO_OOBINLINE` flag for the socket. When set, this indicates that out-of-band data should be placed in the normal input queue of the socket. Note that this option can be specified on a per message basis by using one of the [`sendVec'`](socket.md#SIG:SOCKET.sendVec':VAL:SPEC), [`sendArr'`](socket.md#SIG:SOCKET.sendArr':VAL:SPEC), [`sendVecTo'`](socket.md#SIG:SOCKET.sendVecTo':VAL:SPEC), or [`sendArrTo'`](socket.md#SIG:SOCKET.sendArrTo':VAL:SPEC) functions.

<span id="SIG:SOCKET.Ctl.getSNDBUF:VAL"></span>**`val`**` getSNDBUF `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` int`
**`val`**` setSNDBUF `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` int `**`->`**` unit`  
These query and set the size of the send queue buffer for the socket.

<span id="SIG:SOCKET.Ctl.getRCVBUF:VAL"></span>**`val`**` getRCVBUF `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` int`
**`val`**` setRCVBUF `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` int `**`->`**` unit`  
These query and set the size of receive queue buffer for the socket.

<span id="SIG:SOCKET.Ctl.getTYPE:VAL"></span>**`val`**` getTYPE `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` SOCK.sock_type`  
This returns the socket type of the socket.

<span id="SIG:SOCKET.Ctl.getERROR:VAL"></span>**`val`**` getERROR `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` bool`  
This indicates whether or not an error has occurred.

<span id="SIG:SOCKET.Ctl.getPeerName:VAL"></span>**`val`**` getPeerName `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` `_`'af`_` sock_addr`  
This returns the socket address to which the socket is connected.

<span id="SIG:SOCKET.Ctl.getSockName:VAL"></span>**`val`**` getSockName `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` `_`'af`_` sock_addr`  
This returns the socket address to which the socket is bound.

<span id="SIG:SOCKET.Ctl.getNREAD:VAL"></span>**`val`**` getNREAD `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`->`**` int`  
This returns the number of bytes available for reading on the socket.

<span id="SIG:SOCKET.Ctl.getATMARK:VAL"></span>**`val`**` getATMARK `**`:`**` (`_`'af`_`, active stream) sock `**`->`**` bool`  
This indicates whether or not the read pointer on the socket is currently at the out-of-band mark.

<span id="SIG:SOCKET.sameAddr:VAL"></span>**`val`**` sameAddr `**`:`**` `_`'af`_` sock_addr `**`*`**` `_`'af`_` sock_addr `**`->`**` bool`  
This tests whether two socket addresses are the same address.

<span id="SIG:SOCKET.familyOfAddr:VAL"></span>
`familyOfAddr ``addr`` `  
returns the address family of the socket address `addr`.

<span id="SIG:SOCKET.bind:VAL"></span>
`bind (``sock``, ``sa``) `  
binds the address `sa` to the passive socket `sock`. This function raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) when the address `sa` is already in use, when `sock` is already bound to an address, or when `sock` has been closed.

<span id="SIG:SOCKET.listen:VAL"></span>
`listen (``sock``, ``n``) `  
creates a queue (of size `n`) for pending questions associated to the socket `sock`. The size of queue is limited by the underlying system, but requesting a queue size larger than the limit does not cause an error (a typical limit is 128, but older systems use a limit of 5).

This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if `sock` has been closed.

<span id="SIG:SOCKET.accept:VAL"></span>
`accept ``sock`` `  
extracts the first connection request from the queue of pending connections for the socket `sock`. The socket must have been bound to an address via [`bind`](socket.md#SIG:SOCKET.bind:VAL:SPEC) and enabled for listening via [`listen`](socket.md#SIG:SOCKET.listen:VAL:SPEC). If a connection is present, `accept` returns a pair `(``s``,``sa``)` consisting of a new active socket `s` with the same properties as `sock` and the address `sa` of the connecting entity. If no pending connections are present on the queue then `accept` blocks until a connection is requested. One can test for pending connection requests by using the [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) function to test the socket for reading.

This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if `sock` has not been properly bound and enabled, or it `sock` has been closed.

<span id="SIG:SOCKET.acceptNB:VAL"></span>**`val`**` acceptNB `**`:`**` (`_`'af`_`, passive stream) sock`
`                 `**`->`**` ((`_`'af`_`, active stream) sock`
`                 `**`*`**` `_`'af`_` sock_addr) option`  
This function is the nonblocking form of the [`accept`](socket.md#SIG:SOCKET.accept:VAL:SPEC) operation. If the operation can complete without blocking (_i.e._, there is a pending connection), then this function returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``s``,``sa``)`, where `s` is a new active socket with the same properties as `sock` and `sa` is the the address of the connecting entity. If there are no pending connections, then this function returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if `sock` has not been properly bound and enabled, or it `sock` has been closed.

<span id="SIG:SOCKET.connect:VAL"></span>
`connect (``sock``, ``sa``) `  
attempts to connect the socket `sock` to the address `sa`. If `sock` is a datagram socket, the address specifies the peer with which the socket is to be associated; `sa` is the address to which datagrams are to be sent, and the only address from which datagrams are to be received. If `sock` is a stream socket, the address specifies another socket to which to connect.

This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception when the address specified by `sa` is unreachable, when the connection is refused or times out, when `sock` is already connected, or when `sock` has been closed.

<span id="SIG:SOCKET.connectNB:VAL"></span>**`val`**` connectNB `**`:`**` (`_`'af`_`, `_`'sock_type`_`) sock `**`*`**` `_`'af`_` sock_addr`
`                  `**`->`**` bool`  
This function is the nonblocking form of [`connect`](socket.md#SIG:SOCKET.connect:VAL:SPEC). If the connection can be established without blocking the caller (which is typically true for datagram sockets, but not stream sockets), then `true` is returned. Otherwise, `false` is returned and the connection attempt is started; one can test for the completion of the connection by testing the socket for writing using the [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) function. This function will raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if it is called on a socket for which a previous connection attempt has not yet been completed.

<span id="SIG:SOCKET.close:VAL"></span>
`close ``sock`` `  
closes the connection to the socket `sock`. This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if the socket has already been closed.

<span id="SIG:SOCKET.shutdown:VAL"></span>
`shutdown (``sock``, ``mode``) `  
shuts down all or part of a full-duplex connection on socket `sock`. If `mode` is [`NO_RECVS`](socket.md#SIG:SOCKET.shutdown_mode:TY:SPEC), further receives will be disallowed. If `mode` is [`NO_SENDS`](socket.md#SIG:SOCKET.shutdown_mode:TY:SPEC), further sends will be disallowed. If `mode` is [`NO_RECVS_OR_SENDS`](socket.md#SIG:SOCKET.shutdown_mode:TY:SPEC), further sends and receives will be disallowed. This function raises the [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception if the socket is not connected or has been closed.

<span id="SIG:SOCKET.sock_desc:TY"></span>**`type`**` sock_desc`  
This type is an abstract name for a socket, which is used to support polling on collections of sockets.

<span id="SIG:SOCKET.sockDesc:VAL"></span>
`sockDesc ``sock`` `  
returns a socket descriptor that names the socket `sock`.

<span id="SIG:SOCKET.sameDesc:VAL"></span>
`sameDesc (``sd1``, ``sd2``) `  
returns `true` if the two socket descriptors `sd1` and `sd2` describe the same underlying socket. Thus, the expression `sameDesc(sockDesc ``sock``, sockDesc ``sock``)` will always return `true` for any socket `sock`.

<span id="SIG:SOCKET.select:VAL"></span>
`select {``rds``, ``wrs``, ``exs``, ``timeout``} `  
examines the sockets in `rds`, `wrs`, and `exs` to see if they are ready for reading, writing, or have an exceptional condition pending, respectively. The calling program is blocked until either one or more of the named sockets is \`\`_ready_ '' or the specified `timeout` expires (where a timeout of [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) never expires). The result of [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) is a record of three lists of socket descriptors containing the ready sockets from the corresponding argument lists. The order in which socket descriptors appear in the argument lists is preserved in the result lists. A timeout is signified by a result of three empty lists.

This function raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if any of the argument sockets have been closed or if the timeout value is negative.

Note that one can test if a call to [`accept`](socket.md#SIG:SOCKET.accept:VAL:SPEC) will block by using [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) to see if the socket is ready to read. Similarly, one can use [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) to test if a call to [`connect`](socket.md#SIG:SOCKET.connect:VAL:SPEC) will block by seeing if the socket is ready to write.

<span id="SIG:SOCKET.ioDesc:VAL"></span>
`ioDesc ``sock`` `  
returns the I/O descriptor corresponding to socket `sock`. This descriptor can be used to poll the socket via [`pollDesc`](os-io.md#SIG:OS_IO.pollDesc:VAL:SPEC) and [`poll`](os-io.md#SIG:OS_IO.poll:VAL:SPEC) in the [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC) structure. Using the polling mechanism from [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC) has the advantage that different kinds of I/O objects can be mixed, but not all systems support polling on sockets this way. If an application is only polling sockets, then it is more portable to use the [`select`](socket.md#SIG:SOCKET.select:VAL:SPEC) function defined above.

<span id="SIG:SOCKET.out_flags:TY"></span>**`type`**` out_flags = {don't_route `**`:`**` bool, oob `**`:`**` bool}`  
Flags used in the general form of socket output operations.

<span id="SIG:SOCKET.in_flags:TY"></span>**`type`**` in_flags = {peek `**`:`**` bool, oob `**`:`**` bool}`  
Flags used in the general form of socket input operations.

<span id="SIG:SOCKET.sendVec:VAL"></span>
`sendVec (``sock``, ``slice``) `
`sendArr (``sock``, ``slice``)`  
These functions send the bytes in the slice `slice` on the active stream socket `sock`. They return the number of bytes actually sent.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed.

<span id="SIG:SOCKET.sendVec':VAL"></span>
`sendVec' (``sock``, ``slice``, {``don't_route``, ``oob``}) `
`sendArr' (``sock``, ``slice``, {``don't_route``, ``oob``})`  
These functions send the bytes in the slice `slice` on the active stream socket `sock`. They return the number of bytes actually sent. If the `don't_route` flag is `true`, the data is sent bypassing the normal routing mechanism of the protocol. If `oob` is `true`, the data is sent out-of-band, that is, before any other data which may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed.

<span id="SIG:SOCKET.sendVecNB:VAL"></span>**`val`**` sendVecNB `**`:`**` (`_`'af`_`, active stream) sock`
`                  `**`*`**` Word8VectorSlice.slice `**`->`**` int option`
**`val`**` sendVecNB' `**`:`**` (`_`'af`_`, active stream) sock`
`                   `**`*`**` Word8VectorSlice.slice`
`                   `**`*`**` out_flags `**`->`**` int option`
**`val`**` sendArrNB `**`:`**` (`_`'af`_`, active stream) sock`
`                  `**`*`**` Word8ArraySlice.slice `**`->`**` int option`
**`val`**` sendArrNB' `**`:`**` (`_`'af`_`, active stream) sock`
`                   `**`*`**` Word8ArraySlice.slice`
`                   `**`*`**` out_flags `**`->`**` int option`  
These functions are the nonblocking versions of [`sendVec`](socket.md#SIG:SOCKET.sendVec:VAL:SPEC), [`sendVec'`](socket.md#SIG:SOCKET.sendVec':VAL:SPEC), [`sendArr`](socket.md#SIG:SOCKET.sendArr:VAL:SPEC), and [`sendArr'`](socket.md#SIG:SOCKET.sendArr':VAL:SPEC) (resp.). They have the same semantics as their blocking forms, with the exception that when the operation can complete without blocking, then the result is wrapped in [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) and if the operation would have to wait to send the data, then [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned instead.

<span id="SIG:SOCKET.recvVec:VAL"></span>
`recvVec (``sock``, ``n``) `
`recvVec'(``sock``, ``n``, {``peek``,``oob``})`  
These functions receive up to `n` bytes from the active stream socket `sock`. The size of the resulting vector is the number of bytes that were successfully received, which may be less than `n`. If the connection has been closed at the other end (or if `n` is `0`), then the empty vector will be returned.

In the second version, if `peek` is `true`, the data is received but not discarded from the connection. If `oob` is `true`, the data is received out-of-band, that is, before any other incoming data that may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if the socket `sock` has been closed and they raise [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if `n` \< 0 or `n` \> [`Word8Vector.maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC).

<span id="SIG:SOCKET.recvArr:VAL"></span>
`recvArr (``sock``, ``slice``) `
`recvArr' (``sock``, ``slice``, {``peek``, ``oob``})`  
These functions read data from the socket `sock` into the array slice `slice`. They return the number of bytes actually received. If the connection has been closed at the other end or the slice is empty, then 0 is returned.

For [`recvArr'`](socket.md#SIG:SOCKET.recvArr':VAL:SPEC), if `peek` is `true`, the data is received but not discarded from the connection. If `oob` is `true`, the data is received out-of-band, that is, before any other incoming data that may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed.

<span id="SIG:SOCKET.recvVecNB:VAL"></span>**`val`**` recvVecNB `**`:`**` (`_`'af`_`, active stream) sock `**`*`**` int`
`                  `**`->`**` Word8Vector.vector option`
**`val`**` recvVecNB' `**`:`**` (`_`'af`_`, active stream) sock `**`*`**` int `**`*`**` in_flags`
`                   `**`->`**` Word8Vector.vector option`
**`val`**` recvArrNB `**`:`**` (`_`'af`_`, active stream) sock`
`                  `**`*`**` Word8ArraySlice.slice `**`->`**` int option`
**`val`**` recvArrNB' `**`:`**` (`_`'af`_`, active stream) sock`
`                   `**`*`**` Word8ArraySlice.slice`
`                   `**`*`**` in_flags `**`->`**` int option`  
These functions are the nonblocking versions of [`recvVec`](socket.md#SIG:SOCKET.recvVec:VAL:SPEC), [`recvVec'`](socket.md#SIG:SOCKET.recvVec':VAL:SPEC), [`recvArr`](socket.md#SIG:SOCKET.recvArr:VAL:SPEC), and [`recvArr'`](socket.md#SIG:SOCKET.recvArr':VAL:SPEC) (resp.). They have the same semantics as their blocking forms, with the exception that when the operation can complete without blocking, then the result is wrapped in [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) and if the operation would have to wait for input, then [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned instead.

<span id="SIG:SOCKET.sendVecTo:VAL"></span>
`sendVecTo (``sock``, ``sa``, ``slice``) `
`sendArrTo (``sock``, ``sa``, ``slice``)`  
These functions send the message specified by the slice `slice` on the datagram socket `sock` to the address `sa`.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed or if the socket has been connected to a different address than `sa`.

<span id="SIG:SOCKET.sendVecTo':VAL"></span>
`sendVecTo' (``sock``, ``sa``, ``slice``, {``don't_route``, ``oob``}) `
`sendArrTo' (``sock``, ``sa``, ``slice``, {``don't_route``, ``oob``})`  
These functions send the message specified by the slice `slice` on the datagram socket `sock` to the address

If the `don't_route` flag is `true`, the data is sent bypassing the normal routing mechanism of the protocol. If `oob` is `true`, the data is sent out-of-band, that is, before any other data which may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed or if the socket has been connected to a different address than `sa`.

<span id="SIG:SOCKET.sendVecToNB:VAL"></span>**`val`**` sendVecToNB `**`:`**` (`_`'af`_`, dgram) sock`
`                    `**`*`**` `_`'af`_` sock_addr`
`                    `**`*`**` Word8VectorSlice.slice `**`->`**` bool`
**`val`**` sendVecToNB' `**`:`**` (`_`'af`_`, dgram) sock`
`                     `**`*`**` `_`'af`_` sock_addr`
`                     `**`*`**` Word8VectorSlice.slice`
`                     `**`*`**` out_flags `**`->`**` bool`
**`val`**` sendArrToNB `**`:`**` (`_`'af`_`, dgram) sock`
`                    `**`*`**` `_`'af`_` sock_addr`
`                    `**`*`**` Word8ArraySlice.slice `**`->`**` bool`
**`val`**` sendArrToNB' `**`:`**` (`_`'af`_`, dgram) sock`
`                     `**`*`**` `_`'af`_` sock_addr`
`                     `**`*`**` Word8ArraySlice.slice`
`                     `**`*`**` out_flags `**`->`**` bool`  
These functions are the nonblocking versions of [`sendVecTo`](socket.md#SIG:SOCKET.sendVecTo:VAL:SPEC), [`sendVecTo'`](socket.md#SIG:SOCKET.sendVecTo':VAL:SPEC), [`sendArrTo`](socket.md#SIG:SOCKET.sendArrTo:VAL:SPEC), and [`sendArrTo'`](socket.md#SIG:SOCKET.sendArrTo':VAL:SPEC) (resp.). They have the same semantics as their blocking forms, with the exception that if the operation can complete without blocking, then the operation is performed and `true` is returned. Otherwise, `false` is returned and the message is not sent.

<span id="SIG:SOCKET.recvVecFrom:VAL"></span>
`recvVecFrom (``sock``, ``n``) `
`recvVecFrom' (``sock``, ``n``, {``peek``, ``oob``})`  
These functions receive up to `n` bytes on the datagram socket `sock`, and return a pair `(``vec``,``sa``)`, where the vector `vec` is the received message, and `sa` is the socket address from the which the data originated. If the message is larger than `n`, then data may be lost.

In the second form, if `peek` is `true`, the data is received but not discarded from the connection. If `oob` is `true`, the data is received out-of-band, that is, before any other incoming data that may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed; they raise [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if `n` \< 0 or `n` \> [`Word8Vector.maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC).

<span id="SIG:SOCKET.recvArrFrom:VAL"></span>
`recvArrFrom (``sock``, ``slice``) `
`recvArrFrom' (``sock``, ``slice``)`  
These functions read a message from the datagram socket `sock` into the array slice `slice`. If the message is larger than the size of the slice, then data may be lost. They return the number of bytes actually received. If the connection has been closed at the other end or the slice is empty, then 0 is returned.

For [`recvArrFrom'`](socket.md#SIG:SOCKET.recvArrFrom':VAL:SPEC), if `peek` is `true`, the data is received but not discarded from the connection. If `oob` is `true`, the data is received out-of-band, that is, before any other incoming data that may have been buffered.

These functions raise [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `sock` has been closed.

<span id="SIG:SOCKET.recvVecFromNB:VAL"></span>**`val`**` recvVecFromNB `**`:`**` (`_`'af`_`, dgram) sock `**`*`**` int`
`                      `**`->`**` (Word8Vector.vector`
`                      `**`*`**` `_`'sock_type`_` sock_addr) option`
**`val`**` recvVecFromNB' `**`:`**` (`_`'af`_`, dgram) sock `**`*`**` int `**`*`**` in_flags`
`                       `**`->`**` (Word8Vector.vector`
`                       `**`*`**` `_`'sock_type`_` sock_addr) option`
**`val`**` recvArrFromNB `**`:`**` (`_`'af`_`, dgram) sock`
`                      `**`*`**` Word8ArraySlice.slice`
`                      `**`->`**` (int `**`*`**` `_`'af`_` sock_addr) option`
**`val`**` recvArrFromNB' `**`:`**` (`_`'af`_`, dgram) sock`
`                       `**`*`**` Word8ArraySlice.slice`
`                       `**`*`**` in_flags`
`                       `**`->`**` (int `**`*`**` `_`'af`_` sock_addr) option`  
These functions are the nonblocking versions of [`recvVecFrom`](socket.md#SIG:SOCKET.recvVecFrom:VAL:SPEC), [`recvVecFrom'`](socket.md#SIG:SOCKET.recvVecFrom':VAL:SPEC), [`recvArrFrom`](socket.md#SIG:SOCKET.recvArrFrom:VAL:SPEC), and [`recvArrFrom'`](socket.md#SIG:SOCKET.recvArrFrom':VAL:SPEC) (resp.). They have the same semantics as their blocking forms, with the exception that when the operation can complete without blocking, then the result is wrapped in [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) and if the operation would have to wait for input, then [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned instead.

#### Examples

```repl
INetSock.TCP.socket ();;
```

#### See Also

> [`GenericSock`](generic-sock.md#GenericSock:STR:SPEC), [`INetSock`](inet-sock.md#INetSock:STR:SPEC), [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC), [`NetServDB`](serv-db.md#NetServDB:STR:SPEC), [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)

#### Discussion

> **Implementation note:**
>
> On Unix systems, the non-blocking mode of socket operations is controlled by changing the socket's state using the `setsockopt()` system call. Thus, implementing the non-blocking operations in the [`Socket`](socket.md#Socket:STR:SPEC) structure may require tracking the socket's blocking/nonblocking state in the representation of the [`sock`](socket.md#SIG:SOCKET.sock:TY:SPEC) type.
