# <span id="section:0"></span>The `UnixSock` structure

---

#### Synopsis

<span id="UNIX_SOCK:SIG:SPEC"></span>
<span id="UnixSock:STR:SPEC"></span>

```sml
signature UNIX_SOCK (* OPTIONAL *)
structure UnixSock :> UNIX_SOCK (* OPTIONAL *)
```

This structure is used to create sockets in the Unix address family. This structure is only present when the underlying operating system supports Unix-domain sockets.

Binding a name to a Unix-domain socket with [`bind`](socket.md#SIG:SOCKET.bind:VAL:SPEC) causes a socket file to be created in the filesystem. This file is not removed when the socket is closed; [`OS.FileSys.remove`](os-file-sys.md#SIG:OS_FILE_SYS.remove:VAL:SPEC) must be used to remove the file. The usual filesystem access-control mechanisms are applied when referencing Unix-domain sockets; _e.g._, the file representing the destination of a [`connect`](socket.md#SIG:SOCKET.connect:VAL:SPEC) or [`sendVec`](socket.md#SIG:SOCKET.sendVec:VAL:SPEC) must be writable.

---

#### Interface

<span id="SIG:UNIX_SOCK.unix:TY:SPEC"></span>
<span id="SIG:UNIX_SOCK.sock:TY:SPEC"></span>
<span id="SIG:UNIX_SOCK.stream_sock:TY:SPEC"></span>
<span id="SIG:UNIX_SOCK.dgram_sock:TY:SPEC"></span>
<span id="SIG:UNIX_SOCK.sock_addr:TY:SPEC"></span>
<span id="SIG:UNIX_SOCK.unixAF:VAL:SPEC"></span>
<span id="SIG:UNIX_SOCK.toAddr:VAL:SPEC"></span>
<span id="SIG:UNIX_SOCK.fromAddr:VAL:SPEC"></span>
<span id="SIG:UNIX_SOCK.socket:VAL:SPEC"></span>
<span id="SIG:UNIX_SOCK.socketPair:VAL:SPEC"></span>

```sml
type unix
type 'sock_type sock = (unix, 'sock_type) Socket.sock
type 'mode stream_sock = 'mode Socket.stream sock
type dgram_sock = Socket.dgram sock
type sock_addr = unix Socket.sock_addr
val unixAF : Socket.AF.addr_family
val toAddr : string -> sock_addr
val fromAddr : sock_addr -> string
structure Strm : sig
val socket : unit -> 'mode stream_sock
val socketPair : unit -> 'mode stream_sock
* 'mode stream_sock
end
structure DGrm : sig
val socket : unit -> dgram_sock
val socketPair : unit -> dgram_sock * dgram_sock
end
```

#### Description

<span id="SIG:UNIX_SOCK.unix:TY"></span>**`type`**` unix`  
The witness type of the Unix address family.

<span id="SIG:UNIX_SOCK.sock:TY"></span>**`type`**` `_`'sock_type`_` sock = (unix, `_`'sock_type`_`) Socket.sock`  
The type-scheme for all Unix-domain sockets.

<span id="SIG:UNIX_SOCK.stream_sock:TY"></span>**`type`**` `_`'mode`_` stream_sock = `_`'mode`_` Socket.stream sock`  
The type-scheme of Unix-domain (passive or active) stream sockets.

<span id="SIG:UNIX_SOCK.dgram_sock:TY"></span>**`type`**` dgram_sock = Socket.dgram sock`  
The type of Unix-domain datagram sockets.

<span id="SIG:UNIX_SOCK.sock_addr:TY"></span>**`type`**` sock_addr = unix Socket.sock_addr`  
The type of a Unix-domain socket address.

<span id="SIG:UNIX_SOCK.unixAF:VAL"></span>**`val`**` unixAF `**`:`**` Socket.AF.addr_family`  
The Unix address family value.

<span id="SIG:UNIX_SOCK.toAddr:VAL"></span>
`toAddr ``s`` `  
converts a pathname `s` into a socket address (in the Unix address family); it does not check the validity of the path `s`.

<span id="SIG:UNIX_SOCK.fromAddr:VAL"></span>
`fromAddr ``addr`` `  
returns the Unix file system path corresponding to the Unix-domain socket address `addr`.

<span id="SIG:UNIX_SOCK.Strm:STR"></span>
**`structure`**` Strm`  

<span id="SIG:UNIX_SOCK.Strm.socket:VAL"></span>**`val`**` socket `**`:`**` unit `**`->`**` `_`'mode`_` stream_sock`  
This function creates a stream socket in the Unix address family. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:UNIX_SOCK.Strm.socketPair:VAL"></span>**`val`**` socketPair `**`:`**` unit`
`                   `**`->`**` `_`'mode`_` stream_sock `**`*`**` `_`'mode`_` stream_sock`  
This function creates an unnamed pair of connected stream sockets in the Unix address family. It is similar to the [`Posix.IO.pipe`](posix-io.md#SIG:POSIX_IO.pipe:VAL:SPEC) function in that the returned sockets are connected, but unlike [`pipe`](posix-io.md#SIG:POSIX_IO.pipe:VAL:SPEC), the sockets are bidirectional. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:UNIX_SOCK.DGrm:STR"></span>
**`structure`**` DGrm`  

<span id="SIG:UNIX_SOCK.DGrm.socket:VAL"></span>**`val`**` socket `**`:`**` unit `**`->`**` dgram_sock`  
This function creates a datagram socket in the Unix address family. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

<span id="SIG:UNIX_SOCK.DGrm.socketPair:VAL"></span>**`val`**` socketPair `**`:`**` unit `**`->`**` dgram_sock `**`*`**` dgram_sock`  
This function creates an unnamed pair of connected datagram sockets in the Unix address family. It raises [`SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there are too many sockets in use.

#### Examples

```repl
UnixSock.Strm.socket ();;
```

#### See Also

> [`GenericSock`](generic-sock.md#GenericSock:STR:SPEC), [`INetSock`](inet-sock.md#INetSock:STR:SPEC), [`Socket`](socket.md#Socket:STR:SPEC)
