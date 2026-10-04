# <span id="section:0"></span>The `NetServDB` structure

---

#### Synopsis

<span id="NET_SERV_DB:SIG:SPEC"></span>
<span id="NetServDB:STR:SPEC"></span>

```sml
signature NET_SERV_DB (* OPTIONAL *)
structure NetServDB :> NET_SERV_DB (* OPTIONAL *)
```

This structure accesses the information contained in the network services data base. This data may be retrieved from the file `/etc/services` on many Unix systems, or from some other data base.

---

#### Interface

<span id="SIG:NET_SERV_DB.entry:TY:SPEC"></span>
<span id="SIG:NET_SERV_DB.name:VAL:SPEC"></span>
<span id="SIG:NET_SERV_DB.aliases:VAL:SPEC"></span>
<span id="SIG:NET_SERV_DB.port:VAL:SPEC"></span>
<span id="SIG:NET_SERV_DB.protocol:VAL:SPEC"></span>
<span id="SIG:NET_SERV_DB.getByName:VAL:SPEC"></span>
<span id="SIG:NET_SERV_DB.getByPort:VAL:SPEC"></span>

```sml
type entry
val name : entry -> string
val aliases : entry -> string list
val port : entry -> int
val protocol : entry -> string
val getByName : string * string option -> entry option
val getByPort : int * string option -> entry option
```

#### Description

<span id="SIG:NET_SERV_DB.entry:TY"></span>**`type`**` entry`  
The abstract type of a network service database entry.

<span id="SIG:NET_SERV_DB.name:VAL"></span>
`name ``ent`` `  
returns the official name of the service described by entry `ent` (_e.g._, `"ftp"`, `"telnet"`, etc.).

<span id="SIG:NET_SERV_DB.aliases:VAL"></span>
`aliases ``ent`` `  
returns the alias list of the service described by entry `ent`.

<span id="SIG:NET_SERV_DB.port:VAL"></span>
`port ``ent`` `  
returns the port number of the service described by entry `ent`.

<span id="SIG:NET_SERV_DB.protocol:VAL"></span>
`protocol ``ent`` `  
returns the name of the protocol to use for the service described by the entry `ent` (_e.g._, `"tcp"` or `"udp"`).

<span id="SIG:NET_SERV_DB.getByName:VAL"></span>
`getByName (``s``, ``prot``) `  
reads the network service data base for a service with name `s`. If `prot` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(protname)`, the protocol of the service must also match `protname`; if `prot` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), no protocol restriction is imposed. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

<span id="SIG:NET_SERV_DB.getByPort:VAL"></span>
`getByPort (``i``, ``prot``) `  
reads the network service data base for a service with port number `i`. If `prot` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(protname)`, the protocol of the service must also match `protname`; if `prot` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), no protocol restriction is imposed. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

#### Examples

```repl
NetServDB.getByName "http";;
```

#### See Also

> [`Socket`](socket.md#Socket:STR:SPEC), [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC), [`NetProtDB`](prot-db.md#NetProtDB:STR:SPEC)
