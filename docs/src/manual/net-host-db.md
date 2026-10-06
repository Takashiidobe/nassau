# <span id="section:0"></span>The `NetHostDB` structure

---

#### Synopsis

<span id="NET_HOST_DB:SIG:SPEC"></span>
<span id="NetHostDB:STR:SPEC"></span>

```sml
signature NET_HOST_DB (* OPTIONAL *)
structure NetHostDB :> NET_HOST_DB (* OPTIONAL *)
```

This structure accesses the information contained in the network host data base. The data might be retrieved from a file such as `/etc/hosts` on Unix systems, or dynamically via some network communication. The structure can be used to convert host names (_e.g._, `"cs.princeton.edu"`) to Internet addresses (_e.g._, `"128.112.136.10"`).

---

#### Interface

<span id="SIG:NET_HOST_DB.in_addr:TY:SPEC"></span>
<span id="SIG:NET_HOST_DB.addr_family:TY:SPEC"></span>
<span id="SIG:NET_HOST_DB.entry:TY:SPEC"></span>
<span id="SIG:NET_HOST_DB.name:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.aliases:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.addrType:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.addr:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.addrs:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.getByName:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.getByAddr:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.getHostName:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.toString:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.scan:VAL:SPEC"></span>
<span id="SIG:NET_HOST_DB.fromString:VAL:SPEC"></span>

```sml
eqtype in_addr
eqtype addr_family
type entry
val name : entry -> string
val aliases : entry -> string list
val addrType : entry -> addr_family
val addr : entry -> in_addr
val addrs : entry -> in_addr list
val getByName : string -> entry option
val getByAddr : in_addr -> entry option
val getHostName : unit -> string
val toString : in_addr -> string
val scan : (char, 'a) StringCvt.reader -> (in_addr, 'a) StringCvt.reader
val fromString : string -> in_addr option
```

#### Description

<span id="SIG:NET_HOST_DB.in_addr:TY"></span>**`eqtype`**` in_addr`
The type representing an Internet address.

<span id="SIG:NET_HOST_DB.addr_family:TY"></span>**`eqtype`**` addr_family`
The type representing address families (also known as domains).

<span id="SIG:NET_HOST_DB.entry:TY"></span>**`type`**` entry`
The type representing an entry from the host database.

<span id="SIG:NET_HOST_DB.name:VAL"></span>

### `name`

```sml
val name : entry -> string
```
`name ``en`` `
returns the official name of the host described by entry `en`.

```repl
case NetHostDB.getByName "localhost" of SOME en => NetHostDB.name en | NONE => "";; (* host name, or empty string if absent *)
```
<span id="SIG:NET_HOST_DB.aliases:VAL"></span>

### `aliases`

```sml
val aliases : entry -> string list
```
`aliases ``en`` `
returns the alias list of the host described by entry `en`.

```repl
case NetHostDB.getByName "localhost" of SOME en => NetHostDB.aliases en | NONE => [];; (* aliases, or [] if absent *)
```
<span id="SIG:NET_HOST_DB.addrType:VAL"></span>

### `addrType`

```sml
val addrType : entry -> addr_family
```
`addrType ``en`` `
returns the address family of the host described by entry `en`.

```repl
case NetHostDB.getByName "localhost" of SOME en => NetHostDB.addrType en | NONE => raise Fail "host not found";; (* address family *)
```
<span id="SIG:NET_HOST_DB.addr:VAL"></span>

### `addr`

```sml
val addr : entry -> in_addr
```
`addr ``en`` `
returns the main Internet address of the host described by entry `en`. This is the first address of the list returned by [`addrs`](net-host-db.md#SIG:NET_HOST_DB.addrs:VAL:SPEC).

```repl
case NetHostDB.getByName "localhost" of SOME en => NetHostDB.toString (NetHostDB.addr en) | NONE => "";; (* primary address, or empty string if absent *)
```
<span id="SIG:NET_HOST_DB.addrs:VAL"></span>

### `addrs`

```sml
val addrs : entry -> in_addr list
```
`addrs ``en`` `
returns the list of Internet addresses of the host described by entry `en`. The list is guaranteed to be non-empty.

```repl
case NetHostDB.getByName "localhost" of SOME en => map NetHostDB.toString (NetHostDB.addrs en) | NONE => [];; (* addresses, or [] if absent *)
```
<span id="SIG:NET_HOST_DB.getByName:VAL"></span>

### `getByName`

```sml
val getByName : string -> entry option
```
`getByName ``s`` `
reads the network host data base for a host with name `s`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

```repl
NetHostDB.getByName "localhost";; (* SOME host entry, if localhost is in the host database *)
```
<span id="SIG:NET_HOST_DB.getByAddr:VAL"></span>

### `getByAddr`

```sml
val getByAddr : in_addr -> entry option
```
`getByAddr ``ia`` `
reads the network host data base for a host with Internet address `ia`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

```repl
case NetHostDB.fromString "127.0.0.1" of SOME a => NetHostDB.getByAddr a | NONE => NONE;; (* optional host entry *)
```
<span id="SIG:NET_HOST_DB.getHostName:VAL"></span>

### `getHostName`

```sml
val getHostName : unit -> string
```
**`val`**` getHostName `**`:`**` unit `**`->`**` string`
The standard hostname for the current processor.

```repl
NetHostDB.getHostName ();; (* local host name *)
```
<span id="SIG:NET_HOST_DB.toString:VAL"></span>

### `toString`

```sml
val toString : in_addr -> string
```
`toString ``ia`` `
returns a string representation of the Internet address `ia` in the form `"`_`a`_`.`_`b`_`.`_`c`_`.`_`d`_`"`.

```repl
case NetHostDB.fromString "127.0.0.1" of SOME a => NetHostDB.toString a | NONE => "";; (* "127.0.0.1" *)
```
<span id="SIG:NET_HOST_DB.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (in_addr, 'a) StringCvt.reader
```
`scan ``getc`` ``strm`` `
` fromString ``s`` `
These functions scan Internet address from a character source. The first returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(ia,rest)` if an Internet address can be parsed from a prefix of the character stream `strm` after skipping initial whitespace. `ia` is the resulting address, and `rest` is the remainder of the character stream. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise.

The second form returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(ia)` if an Internet address `ia` can be parsed from a prefix of string `s`. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise. It is equivalent to `StringCvt.scanString scan`.

Addresses in this notation have one of the following forms:

_`a`_
where _`a`_ is a 32-bit unsigned integer constant.

_`a`_`.`_`b`_
where _`a`_ is an 8-bit unsigned integer constant, and _`b`_ is a 24-bit integer constant.

_`a`_`.`_`b`_`.`_`c`_
where _`a`_ and _`b`_ are 8-bit unsigned integer constants, and _`c`_ is a 16-bit integer constant.

_`a`_`.`_`b`_`.`_`c`_`.`_`d`_
where _`a`_, _`b`_, _`c`_, and _`d`_ are 8-bit integer constants.

The integer constants may be decimal, octal, or hexadecimal, as specified in the C language.

#### See Also

> [`GenericSock`](generic-sock.md#GenericSock:STR:SPEC), [`INetSock`](inet-sock.md#INetSock:STR:SPEC), [`NetProtDB`](prot-db.md#NetProtDB:STR:SPEC), [`Socket`](socket.md#Socket:STR:SPEC)

```repl
NetHostDB.fromString "127.0.0.1";; (* SOME address *)
```
