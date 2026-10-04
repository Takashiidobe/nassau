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
User identifier; identical to [`Posix.ProcEnv.uid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.uid:TY:SPEC).

<span id="SIG:POSIX_SYS_DB.gid:TY"></span>**`eqtype`**` gid`  
Group identifier; identical to [`Posix.ProcEnv.gid`](posix-proc-env.md#SIG:POSIX_PROC_ENV.gid:TY:SPEC).

<span id="SIG:POSIX_SYS_DB.Passwd:STR"></span>
**`structure`**` Passwd`  

<span id="SIG:POSIX_SYS_DB.Passwd.passwd:TY"></span>**`type`**` passwd`  
Information related to a user.

<span id="SIG:POSIX_SYS_DB.Passwd.name:VAL"></span>**`val`**` name `**`:`**` passwd `**`->`**` string`
**`val`**` uid `**`:`**` passwd `**`->`**` uid`
**`val`**` gid `**`:`**` passwd `**`->`**` gid`
**`val`**` home `**`:`**` passwd `**`->`**` string`
**`val`**` shell `**`:`**` passwd `**`->`**` string`  
These extract the name, the user ID, the group ID, the path of the initial working, or home, directory, and the initial command shell, respectively, of the user corresponding to the [`passwd`](posix-sys-db.md#SIG:POSIX_SYS_DB.Passwd.passwd:TY:SPEC) value. The names of the corresponding fields in C are the same, but prefixed with `"pw_"`. The one exception is that C uses `pw_dir` for the home directory.

<span id="SIG:POSIX_SYS_DB.Group:STR"></span>
**`structure`**` Group`  

<span id="SIG:POSIX_SYS_DB.Group.group:TY"></span>**`type`**` group`  
Information related to a group.

<span id="SIG:POSIX_SYS_DB.Group.name:VAL"></span>**`val`**` name `**`:`**` group `**`->`**` string`
**`val`**` gid `**`:`**` group `**`->`**` gid`
**`val`**` members `**`:`**` group `**`->`**` string list`  
These extract the name, the group ID, and the names of users belonging to the group, respectively, of the group corresponding to the [`group`](posix-sys-db.md#SIG:POSIX_SYS_DB.Group.group:TY:SPEC) value. In C, these fields are named `gr_name`, `gr_gid`, and `gr_mem`, respectively.

<span id="SIG:POSIX_SYS_DB.getgrgid:VAL"></span>**`val`**` getgrgid `**`:`**` gid `**`->`**` Group.group`
**`val`**` getgrnam `**`:`**` string `**`->`**` Group.group`
**`val`**` getpwuid `**`:`**` uid `**`->`**` Passwd.passwd`
**`val`**` getpwnam `**`:`**` string `**`->`**` Passwd.passwd`  
These return the group or user database entry associated with the given group ID or name, or user ID or name. It raises [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if there is no group or user with the given ID or name.

#### Examples

```repl
Posix.SysDB.getgrnam "wheel";;
```

#### See Also

> [`Posix`](posix.md#Posix:STR:SPEC)
