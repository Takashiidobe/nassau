# <span id="section:0"></span>The `NetProtDB` structure

---

#### Synopsis

<span id="NET_PROT_DB:SIG:SPEC"></span>
<span id="NetProtDB:STR:SPEC"></span>

```sml
signature NET_PROT_DB (* OPTIONAL *)
structure NetProtDB :> NET_PROT_DB (* OPTIONAL *)
```

This structure accesses the information contained in the network protocol data base. The data may be retrieved from a file, such as `/etc/protocols` on many Unix systems, or via the NIS protocols map.

---

#### Interface

<span id="SIG:NET_PROT_DB.entry:TY:SPEC"></span>
<span id="SIG:NET_PROT_DB.name:VAL:SPEC"></span>
<span id="SIG:NET_PROT_DB.aliases:VAL:SPEC"></span>
<span id="SIG:NET_PROT_DB.protocol:VAL:SPEC"></span>
<span id="SIG:NET_PROT_DB.getByName:VAL:SPEC"></span>
<span id="SIG:NET_PROT_DB.getByNumber:VAL:SPEC"></span>

```sml
type entry
val name : entry -> string
val aliases : entry -> string list
val protocol : entry -> int
val getByName : string -> entry option
val getByNumber : int -> entry option
```

#### Description

<span id="SIG:NET_PROT_DB.entry:TY"></span>**`type`**` entry`  
The type of a network protocol data base entry.

<span id="SIG:NET_PROT_DB.name:VAL"></span>
`name ``en`` `  
returns the official name of the protocol described by entry `en` (_e.g._, `"ip"`).

<span id="SIG:NET_PROT_DB.aliases:VAL"></span>
`aliases ``en`` `  
returns the alias list of the protocol described by entry `en`.

<span id="SIG:NET_PROT_DB.protocol:VAL"></span>
`protocol ``en`` `  
returns the protocol number of the protocol described by entry `en`.

<span id="SIG:NET_PROT_DB.getByName:VAL"></span>
`getByName ``s`` `  
reads the network protocol data base for a protocol with name `s`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

<span id="SIG:NET_PROT_DB.getByNumber:VAL"></span>
`getByNumber ``i`` `  
reads the network protocol data base for a protocol with protocol number `i`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

#### Examples

```repl
NetProtDB.getByName "tcp";;
```

#### See Also

> [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC)
