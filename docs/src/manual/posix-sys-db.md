# <span id="section:0"></span>The `Posix.SysDB` structure

---

#### Synopsis

<span id="POSIX_SYS_DB:SIG:SPEC"></span>
<span id="SysDB:STR:SPEC"></span>

```sml
signature POSIX_SYS_DB
structure SysDB : POSIX_SYS_DB
```

The `Posix.SysDB` structure implements operations on the user database and the group database (in POSIX parlance, the password file and the group file). These are the data and operations described in Section 9 of the POSIX standard 1003.1,1996**\[CITE\]**.

---

#### Interface

<span id="SIG:POSIX_SYS_DB.uid:TY:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.gid:TY:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.passwd:TY:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.name:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.uid:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.gid:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.home:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.shell:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.group:TY:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.members:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.getgrgid:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.getgrnam:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.getpwuid:VAL:SPEC"></span>
<span id="SIG:POSIX_SYS_DB.getpwnam:VAL:SPEC"></span>

```sml
eqtype uid
eqtype gid
structure Passwd : sig
type passwd
val name : passwd -> string
val uid : passwd -> uid
val gid : passwd -> gid
val home : passwd -> string
val shell : passwd -> string
end
structure Group : sig
type group
val name : group -> string
val gid : group -> gid
val members : group -> string list
end
val getgrgid : gid -> Group.group
val getgrnam : string -> Group.group
val getpwuid : uid -> Passwd.passwd
val getpwnam : string -> Passwd.passwd
```

#### Description

<span id="SIG:POSIX_SYS_DB.uid:TY"></span>**`eqtype`**` uid`  
A user identifier, identical to [`Posix.ProcEnv.uid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.uid:TY:SPEC).

<span id="SIG:POSIX_SYS_DB.gid:TY"></span>**`eqtype`**` gid`  
A group identifier, identical to [`Posix.ProcEnv.gid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.gid:TY:SPEC).

<span id="SIG:POSIX_SYS_DB.Passwd:STR"></span>
**`structure`**` Passwd`

<span id="SIG:POSIX_SYS_DB.Passwd.passwd:TY"></span>**`type`**` passwd`  
Information about a user account.

<span id="SIG:POSIX_SYS_DB.Passwd.name:VAL"></span>

### `Passwd.name`

```sml
val name : passwd -> string
```
Returns the login name for a password database entry.

```repl
Posix.SysDB.Passwd.name (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ()));; (* current login name *)
```

<span id="SIG:POSIX_SYS_DB.Passwd.uid:VAL"></span>

### `Passwd.uid`

```sml
val uid : passwd -> uid
```
Returns the user ID in a password database entry.

```repl
Posix.SysDB.Passwd.uid (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ()));; (* current user ID *)
```

<span id="SIG:POSIX_SYS_DB.Passwd.gid:VAL"></span>

### `Passwd.gid`

```sml
val gid : passwd -> gid
```
Returns the primary group ID in a password database entry.

```repl
Posix.SysDB.Passwd.gid (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ()));; (* primary group ID *)
```

<span id="SIG:POSIX_SYS_DB.Passwd.home:VAL"></span>

### `Passwd.home`

```sml
val home : passwd -> string
```
Returns the user's home directory.

```repl
Posix.SysDB.Passwd.home (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ()));; (* home directory *)
```

<span id="SIG:POSIX_SYS_DB.Passwd.shell:VAL"></span>

### `Passwd.shell`

```sml
val shell : passwd -> string
```
Returns the user's initial command shell.

```repl
Posix.SysDB.Passwd.shell (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ()));; (* login shell *)
```

<span id="SIG:POSIX_SYS_DB.Group:STR"></span>
**`structure`**` Group`

<span id="SIG:POSIX_SYS_DB.Group.group:TY"></span>**`type`**` group`  
Information about a group database entry.

<span id="SIG:POSIX_SYS_DB.Group.name:VAL"></span>

### `Group.name`

```sml
val name : group -> string
```
Returns the name of a group database entry.

```repl
Posix.SysDB.Group.name (Posix.SysDB.getgrgid (Posix.ProcEnv.getgid ()));; (* current group name *)
```

<span id="SIG:POSIX_SYS_DB.Group.gid:VAL"></span>

### `Group.gid`

```sml
val gid : group -> gid
```
Returns the group ID in a group database entry.

```repl
Posix.SysDB.Group.gid (Posix.SysDB.getgrgid (Posix.ProcEnv.getgid ()));; (* current group ID *)
```

<span id="SIG:POSIX_SYS_DB.Group.members:VAL"></span>

### `Group.members`

```sml
val members : group -> string list
```
Returns the member names in a group database entry.

```repl
Posix.SysDB.Group.members (Posix.SysDB.getgrgid (Posix.ProcEnv.getgid ()));; (* group members *)
```

<span id="SIG:POSIX_SYS_DB.getgrgid:VAL"></span>

### `getgrgid`

```sml
val getgrgid : gid -> Group.group
```
Looks up the group associated with a group ID.

```repl
Posix.SysDB.getgrgid (Posix.ProcEnv.getgid ());; (* current group entry *)
```

<span id="SIG:POSIX_SYS_DB.getgrnam:VAL"></span>

### `getgrnam`

```sml
val getgrnam : string -> Group.group
```
Looks up a group by name.

```repl
Posix.SysDB.getgrnam (Posix.SysDB.Group.name (Posix.SysDB.getgrgid (Posix.ProcEnv.getgid ())));; (* current group entry *)
```

<span id="SIG:POSIX_SYS_DB.getpwuid:VAL"></span>

### `getpwuid`

```sml
val getpwuid : uid -> Passwd.passwd
```
Looks up a user by user ID.

```repl
Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ());; (* current user entry *)
```

<span id="SIG:POSIX_SYS_DB.getpwnam:VAL"></span>

### `getpwnam`

```sml
val getpwnam : string -> Passwd.passwd
```
Looks up a user by login name.

```repl
Posix.SysDB.getpwnam (Posix.SysDB.Passwd.name (Posix.SysDB.getpwuid (Posix.ProcEnv.getuid ())));; (* current user entry *)
```

#### See Also

> [`Posix`](posix.md#Posix:STR:SPEC)
