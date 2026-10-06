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

### `name`

```sml
val name : entry -> string
```
`name ``en`` `
returns the official name of the protocol described by entry `en` (_e.g._, `"ip"`).

```repl
case NetProtDB.getByName "tcp" of SOME en => NetProtDB.name en | NONE => "";; (* "tcp", or empty string if absent *)
```
<span id="SIG:NET_PROT_DB.aliases:VAL"></span>

### `aliases`

```sml
val aliases : entry -> string list
```
`aliases ``en`` `
returns the alias list of the protocol described by entry `en`.

```repl
case NetProtDB.getByName "tcp" of SOME en => NetProtDB.aliases en | NONE => [];; (* aliases, or [] if absent *)
```
<span id="SIG:NET_PROT_DB.protocol:VAL"></span>

### `protocol`

```sml
val protocol : entry -> int
```
`protocol ``en`` `
returns the protocol number of the protocol described by entry `en`.

```repl
case NetProtDB.getByName "tcp" of SOME en => NetProtDB.protocol en | NONE => ~1;; (* protocol number, or ~1 if absent *)
```
<span id="SIG:NET_PROT_DB.getByName:VAL"></span>

### `getByName`

```sml
val getByName : string -> entry option
```
`getByName ``s`` `
reads the network protocol data base for a protocol with name `s`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

```repl
NetProtDB.getByName "tcp";; (* protocol entry when present in the database *)
```
<span id="SIG:NET_PROT_DB.getByNumber:VAL"></span>

### `getByNumber`

```sml
val getByNumber : int -> entry option
```
```
`getByNumber ``i`` `
reads the network protocol data base for a protocol with protocol number `i`. If successful, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(en)` where `en` is the corresponding data base entry; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

#### See Also

> [`NetHostDB`](net-host-db.md#NetHostDB:STR:SPEC)

```repl
NetProtDB.getByNumber 6;; (* protocol entry when present in the database *)
```
