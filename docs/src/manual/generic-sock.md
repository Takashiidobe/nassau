# <span id="section:0"></span>The `GenericSock` structure

---

#### Synopsis

<span id="GENERIC_SOCK:SIG:SPEC"></span>
<span id="GenericSock:STR:SPEC"></span>

```sml
signature GENERIC_SOCK (* OPTIONAL *)
structure GenericSock :> GENERIC_SOCK (* OPTIONAL *)
```

Implementations may provide the `GenericSock` structure as a way to provide access to additional address families and socket types (beyond those supported by [`INetSock`](inet-sock.md#INetSock:STR:SPEC) and [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)).

---

#### Interface

<span id="SIG:GENERIC_SOCK.socket:VAL:SPEC"></span>
<span id="SIG:GENERIC_SOCK.socketPair:VAL:SPEC"></span>
<span id="SIG:GENERIC_SOCK.socket':VAL:SPEC"></span>
<span id="SIG:GENERIC_SOCK.socketPair':VAL:SPEC"></span>

```sml
val socket : Socket.AF.addr_family * Socket.SOCK.sock_type -> ('af, 'sock_type) Socket.sock
val socketPair : Socket.AF.addr_family
* Socket.SOCK.sock_type -> ('af, 'sock_type) Socket.sock
* ('af, 'sock_type) Socket.sock
val socket' : Socket.AF.addr_family
* Socket.SOCK.sock_type
* int -> ('af, 'sock_type) Socket.sock
val socketPair' : Socket.AF.addr_family
* Socket.SOCK.sock_type
* int -> ('af, 'sock_type) Socket.sock
* ('af, 'sock_type) Socket.sock
```

#### Description

<span id="SIG:GENERIC_SOCK.socket:VAL"></span>

### `socket`

```sml
val socket : Socket.AF.addr_family * Socket.SOCK.sock_type -> ('af, 'sock_type) Socket.sock
```
creates a socket in the address family specified by `af` and the socket type specified by `st`, with the default protocol.


```repl
GenericSock.socket (Socket.AF_INET, Socket.SOCK_STREAM);; (* stream socket *)
```

<span id="SIG:GENERIC_SOCK.socketPair:VAL"></span>

### `socketPair`

```sml
val socketPair : Socket.AF.addr_family * Socket.SOCK.sock_type -> ('af, 'sock_type) Socket.sock * ('af, 'sock_type) Socket.sock
```
creates an unnamed pair of connected sockets in the address family specified by `af` and the socket type specified by `st`, with the default protocol.


```repl
GenericSock.socketPair (Socket.AF_UNIX, Socket.SOCK_STREAM);; (* connected socket pair *)
```

<span id="SIG:GENERIC_SOCK.socket':VAL"></span>

### `socket'`

```sml
val socket' : Socket.AF.addr_family * Socket.SOCK.sock_type * int -> ('af, 'sock_type) Socket.sock
```
creates a socket in the address family specified by `af` and the socket type specified by `st`, with protocol number `i`.


```repl
GenericSock.socket' (Socket.AF_INET, Socket.SOCK_STREAM, 0);; (* stream socket using protocol 0 *)
```

<span id="SIG:GENERIC_SOCK.socketPair':VAL"></span>

### `socketPair'`

```sml
val socketPair' : Socket.AF.addr_family * Socket.SOCK.sock_type * int -> ('af, 'sock_type) Socket.sock * ('af, 'sock_type) Socket.sock
```
creates an unnamed pair of connected sockets in the address family specified by `af` and the socket type specified by `st`, with protocol number `i`.

```repl
GenericSock.socketPair' (Socket.AF_UNIX, Socket.SOCK_STREAM, 0);; (* connected socket pair using protocol 0 *)
```


#### See Also

> [`INetSock`](inet-sock.md#INetSock:STR:SPEC), [`NetProtDB`](prot-db.md#NetProtDB:STR:SPEC), [`Socket`](socket.md#Socket:STR:SPEC), [`UnixSock`](unix-sock.md#UnixSock:STR:SPEC)

#### Discussion

> **Question:**
>
> `addressFamilies : unit -> Socket.AF.addr_family list`?
>
> `socketTypes : unit -> Socket.SOCK.sock_type`?
