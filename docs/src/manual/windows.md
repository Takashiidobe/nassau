# <span id="section:0"></span>The `Windows` structure

---

#### Synopsis

<span id="WINDOWS:SIG:SPEC"></span>
<span id="Windows:STR:SPEC"></span>

```sml
signature WINDOWS (* OPTIONAL *)
structure Windows :> WINDOWS (* OPTIONAL *)
```

The `Windows` structure provides a high-level interface to various system features based on the Microsoft Microsoft Windows operating system model. These functions include the ability to create and communicate with separate processes, as well as to interact with the registry and file subsystems. In particular, using this module, a program can invoke a separate process and obtain input and output streams connected to the standard output and input streams, respectively, of the other process. The functions provide a richer and more detailed interface than the comparable functions provided by the substructures in [`OS`](os.md#OS:STR:SPEC).

---

#### Interface

<span id="SIG:WINDOWS.allAccess:VAL:SPEC"></span>
<span id="SIG:WINDOWS.createLink:VAL:SPEC"></span>
<span id="SIG:WINDOWS.createSubKey:VAL:SPEC"></span>
<span id="SIG:WINDOWS.enumerateSubKeys:VAL:SPEC"></span>
<span id="SIG:WINDOWS.execute:VAL:SPEC"></span>
<span id="SIG:WINDOWS.notify:VAL:SPEC"></span>
<span id="SIG:WINDOWS.queryValue:VAL:SPEC"></span>
<span id="SIG:WINDOWS.read:VAL:SPEC"></span>
<span id="SIG:WINDOWS.setValue:VAL:SPEC"></span>
<span id="SIG:WINDOWS.write:VAL:SPEC"></span>
<span id="SIG:WINDOWS.hkey:TY:SPEC"></span>
<span id="SIG:WINDOWS.classesRoot:VAL:SPEC"></span>
<span id="SIG:WINDOWS.currentUser:VAL:SPEC"></span>
<span id="SIG:WINDOWS.localMachine:VAL:SPEC"></span>
<span id="SIG:WINDOWS.users:VAL:SPEC"></span>
<span id="SIG:WINDOWS.performanceData:VAL:SPEC"></span>
<span id="SIG:WINDOWS.currentConfig:VAL:SPEC"></span>
<span id="SIG:WINDOWS.dynData:VAL:SPEC"></span>
<span id="SIG:WINDOWS.create_result:TY:SPEC"></span>
<span id="SIG:WINDOWS.CREATED_NEW_KEY:TY:SPEC"></span>
<span id="SIG:WINDOWS.OPENED_EXISTING_KEY:TY:SPEC"></span>
<span id="SIG:WINDOWS.createKeyEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.openKeyEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.closeKey:VAL:SPEC"></span>
<span id="SIG:WINDOWS.deleteKey:VAL:SPEC"></span>
<span id="SIG:WINDOWS.deleteValue:VAL:SPEC"></span>
<span id="SIG:WINDOWS.enumKeyEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.enumValueEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.value:TY:SPEC"></span>
<span id="SIG:WINDOWS.SZ:TY:SPEC"></span>
<span id="SIG:WINDOWS.DWORD:TY:SPEC"></span>
<span id="SIG:WINDOWS.BINARY:TY:SPEC"></span>
<span id="SIG:WINDOWS.MULTI_SZ:TY:SPEC"></span>
<span id="SIG:WINDOWS.EXPAND_SZ:TY:SPEC"></span>
<span id="SIG:WINDOWS.queryValueEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.setValueEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.platformWin32s:VAL:SPEC"></span>
<span id="SIG:WINDOWS.platformWin32Windows:VAL:SPEC"></span>
<span id="SIG:WINDOWS.platformWin32NT:VAL:SPEC"></span>
<span id="SIG:WINDOWS.platformWin32CE:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getVersionEx:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getWindowsDirectory:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getSystemDirectory:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getComputerName:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getUserName:VAL:SPEC"></span>
<span id="SIG:WINDOWS.info:TY:SPEC"></span>
<span id="SIG:WINDOWS.startDialog:VAL:SPEC"></span>
<span id="SIG:WINDOWS.executeString:VAL:SPEC"></span>
<span id="SIG:WINDOWS.stopDialog:VAL:SPEC"></span>
<span id="SIG:WINDOWS.getVolumeInformation:VAL:SPEC"></span>
<span id="SIG:WINDOWS.findExecutable:VAL:SPEC"></span>
<span id="SIG:WINDOWS.launchApplication:VAL:SPEC"></span>
<span id="SIG:WINDOWS.openDocument:VAL:SPEC"></span>
<span id="SIG:WINDOWS.simpleExecute:VAL:SPEC"></span>
<span id="SIG:WINDOWS.proc:TY:SPEC"></span>
<span id="SIG:WINDOWS.textInstreamOf:VAL:SPEC"></span>
<span id="SIG:WINDOWS.binInstreamOf:VAL:SPEC"></span>
<span id="SIG:WINDOWS.textOutstreamOf:VAL:SPEC"></span>
<span id="SIG:WINDOWS.binOutstreamOf:VAL:SPEC"></span>
<span id="SIG:WINDOWS.reap:VAL:SPEC"></span>
<span id="SIG:WINDOWS.status:TY:SPEC"></span>
<span id="SIG:WINDOWS.accessViolation:VAL:SPEC"></span>
<span id="SIG:WINDOWS.arrayBoundsExceeded:VAL:SPEC"></span>
<span id="SIG:WINDOWS.breakpoint:VAL:SPEC"></span>
<span id="SIG:WINDOWS.controlCExit:VAL:SPEC"></span>
<span id="SIG:WINDOWS.datatypeMisalignment:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatDenormalOperand:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatDivideByZero:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatInexactResult:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatInvalidOperation:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatOverflow:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatStackCheck:VAL:SPEC"></span>
<span id="SIG:WINDOWS.floatUnderflow:VAL:SPEC"></span>
<span id="SIG:WINDOWS.guardPageViolation:VAL:SPEC"></span>
<span id="SIG:WINDOWS.integerDivideByZero:VAL:SPEC"></span>
<span id="SIG:WINDOWS.integerOverflow:VAL:SPEC"></span>
<span id="SIG:WINDOWS.illegalInstruction:VAL:SPEC"></span>
<span id="SIG:WINDOWS.invalidDisposition:VAL:SPEC"></span>
<span id="SIG:WINDOWS.invalidHandle:VAL:SPEC"></span>
<span id="SIG:WINDOWS.inPageError:VAL:SPEC"></span>
<span id="SIG:WINDOWS.noncontinuableException:VAL:SPEC"></span>
<span id="SIG:WINDOWS.pending:VAL:SPEC"></span>
<span id="SIG:WINDOWS.privilegedInstruction:VAL:SPEC"></span>
<span id="SIG:WINDOWS.singleStep:VAL:SPEC"></span>
<span id="SIG:WINDOWS.stackOverflow:VAL:SPEC"></span>
<span id="SIG:WINDOWS.timeout:VAL:SPEC"></span>
<span id="SIG:WINDOWS.userAPC:VAL:SPEC"></span>
<span id="SIG:WINDOWS.fromStatus:VAL:SPEC"></span>
<span id="SIG:WINDOWS.exit:VAL:SPEC"></span>

```sml
structure Key : sig
include BIT_FLAGS
val allAccess : flags
val createLink : flags
val createSubKey : flags
val enumerateSubKeys : flags
val execute : flags
val notify : flags
val queryValue : flags
val read : flags
val setValue : flags
val write : flags
end
structure Reg : sig
eqtype hkey
val classesRoot : hkey
val currentUser : hkey
val localMachine : hkey
val users : hkey
val performanceData : hkey
val currentConfig : hkey
val dynData : hkey
datatype create_result
= CREATED_NEW_KEY of hkey
| OPENED_EXISTING_KEY of hkey
val createKeyEx : hkey * string * Key.flags -> create_result
val openKeyEx : hkey * string * Key.flags -> hkey
val closeKey : hkey -> unit
val deleteKey : hkey * string -> unit
val deleteValue : hkey * string -> unit
val enumKeyEx : hkey * int -> string option
val enumValueEx : hkey * int -> string option
datatype value
= SZ of string
| DWORD of SysWord.word
| BINARY of Word8Vector.vector
| MULTI_SZ of string list
| EXPAND_SZ of string
val queryValueEx : hkey * string -> value option
val setValueEx : hkey * string * value -> unit
end
structure Config : sig
val platformWin32s : SysWord.word
val platformWin32Windows : SysWord.word
val platformWin32NT : SysWord.word
val platformWin32CE : SysWord.word
val getVersionEx : unit -> {
majorVersion : SysWord.word,
minorVersion : SysWord.word,
buildNumber : SysWord.word,
platformId : SysWord.word,
csdVersion : string
}
val getWindowsDirectory : unit -> string
val getSystemDirectory : unit -> string
val getComputerName : unit -> string
val getUserName : unit -> string
end
structure DDE : sig
type info
val startDialog : string * string -> info
val executeString : info * string * int * Time.time -> unit
val stopDialog : info -> unit
end
val getVolumeInformation : string -> {
volumeName : string,
systemName : string,
serialNumber : SysWord.word,
maximumComponentLength : int
}
val findExecutable : string -> string option
val launchApplication : string * string -> unit
val openDocument : string -> unit
val simpleExecute : string * string -> OS.Process.status
type ('a,'b) proc
val execute : string * string -> ('a, 'b) proc
val textInstreamOf : (TextIO.instream, 'a) proc -> TextIO.instream
val binInstreamOf : (BinIO.instream, 'a) proc -> BinIO.instream
val textOutstreamOf : ('a, TextIO.outstream) proc -> TextIO.outstream
val binOutstreamOf : ('a, BinIO.outstream) proc -> BinIO.outstream
val reap : ('a, 'b) proc -> OS.Process.status
structure Status : sig
type status = SysWord.word
val accessViolation : status
val arrayBoundsExceeded : status
val breakpoint : status
val controlCExit : status
val datatypeMisalignment : status
val floatDenormalOperand : status
val floatDivideByZero : status
val floatInexactResult : status
val floatInvalidOperation : status
val floatOverflow : status
val floatStackCheck : status
val floatUnderflow : status
val guardPageViolation : status
val integerDivideByZero : status
val integerOverflow : status
val illegalInstruction : status
val invalidDisposition : status
val invalidHandle : status
val inPageError : status
val noncontinuableException : status
val pending : status
val privilegedInstruction : status
val singleStep : status
val stackOverflow : status
val timeout : status
val userAPC : status
end
val fromStatus : OS.Process.status -> Status.status
val exit : Status.status -> 'a
```

#### Description

<span id="SIG:WINDOWS.Key:STR"></span>
**`structure`**` Key`  
The [`Key`](windows.md#SIG:WINDOWS.Key:STR:SPEC) substructure contains flags for specifying security settings when opening and creating keys in the registry.

<span id="SIG:WINDOWS.Key.allAccess:VAL"></span>**`val`**` allAccess `**`:`**` flags`  
The union of the [`queryValue`](windows.md#SIG:WINDOWS.Key.queryValue:VAL:SPEC), [`enumerateSubKeys`](windows.md#SIG:WINDOWS.Key.enumerateSubKeys:VAL:SPEC), [`notify`](windows.md#SIG:WINDOWS.Key.notify:VAL:SPEC), [`createSubKey`](windows.md#SIG:WINDOWS.Key.createSubKey:VAL:SPEC), [`createLink`](windows.md#SIG:WINDOWS.Key.createLink:VAL:SPEC), and [`setValue`](windows.md#SIG:WINDOWS.Key.setValue:VAL:SPEC) flags.

<span id="SIG:WINDOWS.Key.createLink:VAL"></span>**`val`**` createLink `**`:`**` flags`  
Permission to create a symbolic link. This value is included for completeness, as the rest of the structure does not support links.

<span id="SIG:WINDOWS.Key.createSubKey:VAL"></span>**`val`**` createSubKey `**`:`**` flags`  
Permission to create subkeys.

<span id="SIG:WINDOWS.Key.enumerateSubKeys:VAL"></span>**`val`**` enumerateSubKeys `**`:`**` flags`  
Permission to enumerate subkeys.

<span id="SIG:WINDOWS.Key.execute:VAL"></span>**`val`**` execute `**`:`**` flags`  
Permission for read access.

<span id="SIG:WINDOWS.Key.notify:VAL"></span>**`val`**` notify `**`:`**` flags`  
Permission for change notification. This value is included for completeness, as the rest of the structure does not support notification.

<span id="SIG:WINDOWS.Key.queryValue:VAL"></span>**`val`**` queryValue `**`:`**` flags`  
Permission to query subkey data.

<span id="SIG:WINDOWS.Key.read:VAL"></span>**`val`**` read `**`:`**` flags`  
The union of the `queryValue`, `enumerateSubKeys`, and `notify` flags.

<span id="SIG:WINDOWS.Key.setValue:VAL"></span>**`val`**` setValue `**`:`**` flags`  
Permission to set subkey data.

<span id="SIG:WINDOWS.Key.write:VAL"></span>**`val`**` write `**`:`**` flags`  
The union of the [`setValue`](windows.md#SIG:WINDOWS.Key.setValue:VAL:SPEC) and [`createSubKey`](windows.md#SIG:WINDOWS.Key.createSubKey:VAL:SPEC) flags.

<span id="SIG:WINDOWS.Reg:STR"></span>
**`structure`**` Reg`  
This substructure provides Microsoft Windows registry functions.

<span id="SIG:WINDOWS.Reg.hkey:TY"></span>**`eqtype`**` hkey`  
Type of registry key values.

<span id="SIG:WINDOWS.Reg.classesRoot:VAL"></span>**`val`**` classesRoot `**`:`**` hkey`
**`val`**` currentUser `**`:`**` hkey`
**`val`**` localMachine `**`:`**` hkey`
**`val`**` users `**`:`**` hkey`
**`val`**` performanceData `**`:`**` hkey`
**`val`**` currentConfig `**`:`**` hkey`
**`val`**` dynData `**`:`**` hkey`  
These are identifiers for top-level registry keys.

<span id="SIG:WINDOWS.Reg.createKeyEx:VAL"></span>
`createKeyEx (``hkey``, ``skey``, ``regsam``) `  
opens or creates a subkey of `hkey`, with the name `skey` and security access specified by `regsam`.

> **Implementation note:**
>
> This passes `REG_OPTION_NON_VOLATILE` option, `NULL` Class, and `SECURITY_ATTRIBUTE` arguments to the Win32 call.


<span id="SIG:WINDOWS.Reg.openKeyEx:VAL"></span>
`openKeyEx (``hkey``, ``skey``, ``regsam``) `  
opens a subkey of `hkey` with the name `skey` and security access specified by `regsam`.

<span id="SIG:WINDOWS.Reg.closeKey:VAL"></span>
`closeKey ``hkey`` `  
closes the key `hkey`.

<span id="SIG:WINDOWS.Reg.deleteKey:VAL"></span>
`deleteKey (``hkey``, ``skey``) `  
deletes the subkey `skey` of `hkey`.

<span id="SIG:WINDOWS.Reg.deleteValue:VAL"></span>
`deleteValue (``hkey``, ``valname``) `  
deletes the value `valname` of `hkey`.

<span id="SIG:WINDOWS.Reg.enumKeyEx:VAL"></span>
`enumKeyEx (``hkey``, ``ind``) `  
returns the subkey of index `ind` of the key `hkey`, where indices start from zero. The function returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) of a string for each defined subkey. To enumerate all the subkeys, start with the index at zero and increment it until the function returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The function raises the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `ind` is invalid.

<span id="SIG:WINDOWS.Reg.enumValueEx:VAL"></span>
`enumValueEx (``hkey``, ``ind``) `  
returns the value of index `ind` of the key `hkey`, where indices start from zero. The function returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC) of a string for each defined value. To enumerate all the values, start with the index at zero and increment it until the function returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The function raises the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `ind` is invalid.

<span id="SIG:WINDOWS.Reg.value:TY"></span>**`datatype`**` value`
`  = SZ `**`of`**` string`
`  | DWORD `**`of`**` SysWord.word`
`  | BINARY `**`of`**` Word8Vector.vector`
`  | MULTI_SZ `**`of`**` string list`
`  | EXPAND_SZ `**`of`**` string`  
This type describes the kind of values that can be saved to the registry or extracted from it. The constructor [`SZ`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) corresponds to strings, [`DWORD`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) to 32-bit numbers, [`BINARY`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) to arbitrary binary values, [`MULTI_SZ`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) to lists of strings, and [`EXPAND_SZ`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) to strings containing environment variables.

<span id="SIG:WINDOWS.Reg.queryValueEx:VAL"></span>
`queryValueEx (``hkey``, ``name``) `  
returns the data associated with `name` in the open registry key `hkey`. A value whose type does not correspond to a more specific instance of the [`value`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) datatype is returned as a [`BINARY`](windows.md#SIG:WINDOWS.Reg.value:TY:SPEC) value. If the value does not exist in the key, the function returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). Any other error, such as having insufficient access rights to the registry key, results in the [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) exception being raised.

A common use of a registry value is to override the default behavior of a program. The normal case is when the registry value is unset. Using an option type allows for the result of [`queryValueEx`](windows.md#SIG:WINDOWS.Reg.queryValueEx:VAL:SPEC) to indicate the presence or absence of the key.

<span id="SIG:WINDOWS.Reg.setValueEx:VAL"></span>
`setValueEx (``hkey``, ``name``, ``v``) `  
associates the value `v` with `name` in the open key `hkey`.

<span id="SIG:WINDOWS.Config:STR"></span>
**`structure`**` Config`  
This substructure contains functions to obtain information about the operating system.

<span id="SIG:WINDOWS.Config.platformWin32s:VAL"></span>**`val`**` platformWin32s `**`:`**` SysWord.word`
**`val`**` platformWin32Windows `**`:`**` SysWord.word`
**`val`**` platformWin32NT `**`:`**` SysWord.word`
**`val`**` platformWin32CE `**`:`**` SysWord.word`  
These are values corresponding to the indicated Microsoft Windows platforms.

<span id="SIG:WINDOWS.Config.getVersionEx:VAL"></span>**`val`**` getVersionEx `**`:`**` unit`
`                     `**`->`**` {`
`                       majorVersion `**`:`**` SysWord.word,`
`                       minorVersion `**`:`**` SysWord.word,`
`                       buildNumber `**`:`**` SysWord.word,`
`                       platformId `**`:`**` SysWord.word,`
`                       csdVersion `**`:`**` string`
`                     }`  
This returns the major and minor versions of the operating system, the build number, platform identifier, and a supplementary version string. The platform identifier `platformId` can be compared with values [`platformWin32s`](windows.md#SIG:WINDOWS.Config.platformWin32s:VAL:SPEC), [`platformWin32Windows`](windows.md#SIG:WINDOWS.Config.platformWin32Windows:VAL:SPEC), [`platformWin32NT`](windows.md#SIG:WINDOWS.Config.platformWin32NT:VAL:SPEC), and [`platformWin32CE`](windows.md#SIG:WINDOWS.Config.platformWin32CE:VAL:SPEC) to determine the type of platform. Note that additional values for other platforms may be returned.

The major and minor version numbers allow additional distinctions. In the case where `platformId` is [`platformWin32Windows`](windows.md#SIG:WINDOWS.Config.platformWin32Windows:VAL:SPEC), we have:

---

`minorVersion`

**System**

0

Windows 95

\> 0

Windows 98

---

In the case where `platformId` is [`platformWin32NT`](windows.md#SIG:WINDOWS.Config.platformWin32NT:VAL:SPEC), we have:

---

`majorVersion`

`minorVersion`

**System**

4

0

Windows NT

5

0

Windows 2000

5

\> 0

Windows XP

---


<span id="SIG:WINDOWS.Config.getWindowsDirectory:VAL"></span>**`val`**` getWindowsDirectory `**`:`**` unit `**`->`**` string`  
The Windows directory, typically `"C:\Windows"` on Windows 95 or `"C:\Winnt"` on Windows NT.

<span id="SIG:WINDOWS.Config.getSystemDirectory:VAL"></span>**`val`**` getSystemDirectory `**`:`**` unit `**`->`**` string`  
The Windows system directory, typically `"C:\Windows\System"` or `"C:\Winnt\System32"`.

<span id="SIG:WINDOWS.Config.getComputerName:VAL"></span>**`val`**` getComputerName `**`:`**` unit `**`->`**` string`  
The name of the computer.

<span id="SIG:WINDOWS.Config.getUserName:VAL"></span>**`val`**` getUserName `**`:`**` unit `**`->`**` string`  
The name of the current user.

<span id="SIG:WINDOWS.DDE:STR"></span>
**`structure`**` DDE`  
This substructure provides a high-level, client-side interface for simple dynamic data exchange (DDE) interactions. All transactions are synchronous. Advise loops and poke transactions are not supported by this interface.

<span id="SIG:WINDOWS.DDE.startDialog:VAL"></span>
`startDialog (``service``, ``topic``) `  
initiates DDE and connects to the given service and topic. It returns the [`info`](windows.md#SIG:WINDOWS.DDE.info:TY:SPEC) value created by these operations.

<span id="SIG:WINDOWS.DDE.executeString:VAL"></span>
`executeString (``info``, ``cmd``, ``retry``, ``delay``) `  
attempts to execute the command `cmd` on the service and topic specified by the `info` value. The `retry` argument specifies the number of times to attempt the transaction if the server is busy, pausing for `delay` between each attempt.

<span id="SIG:WINDOWS.DDE.stopDialog:VAL"></span>
`stopDialog ``info`` `  
disconnects the service and topic specified by the `info` argument and frees the associated resources.

<span id="SIG:WINDOWS.getVolumeInformation:VAL"></span>
`getVolumeInformation ``root`` `  
returns information about the filesystem and volume specified by the root pathname `root`. The `volumeName` field contains the name of the volume; the `systemName` field contains its type (_e.g._, "FAT" or "NTFS"); the `serialNumber` field contains the serial number; and the `maximumComponentLength` field specifies the maximum length of any component of a pathname on this system.

<span id="SIG:WINDOWS.findExecutable:VAL"></span>
`findExecutable ``name`` `  
returns the full executable name associated with `name`, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if no such file exists.

<span id="SIG:WINDOWS.launchApplication:VAL"></span>
`launchApplication (``file``, ``arg``) `  
runs the specified executable `file` passing it the argument `arg`. It raises [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if `file` is not executable or if it cannot be run.

> **Implementation note:**
>
> This should be implemented using `ShellExecute`, passing `SW_SHOWNORMAL` to the underlying API call.


<span id="SIG:WINDOWS.openDocument:VAL"></span>
`openDocument ``file`` `  
opens `file` using its associated application.

> **Implementation note:**
>
> This should pass `SW_SHOWNORMAL` to the underlying `ShellExecute` API call.


<span id="SIG:WINDOWS.simpleExecute:VAL"></span>
`simpleExecute (``cmd``, ``arg``) `  
spawns the process specified by `cmd` with command-line arguments represented by the string `arg`, redirecting standard input and standard output to the null device. It then waits for the subprocess to terminate, and returns its exit status. This is similar to [`OS.Process.system`](os-process.md#SIG:OS_PROCESS.system:VAL:SPEC) but it can be used in cases where the latter does not work, and its return value provides more information about the exit status of the child process.

> **Implementation note:**
>
> This corresponds to the use of `CreateProcess`.


<span id="SIG:WINDOWS.proc:TY"></span>**`type`**` (`_`'a`_`,`_`'b`_`) proc`  
The type of a process created by [`execute`](windows.md#SIG:WINDOWS.execute:VAL:SPEC). The type parameters are witness types for the types of streams that can be returned.

<span id="SIG:WINDOWS.execute:VAL"></span>
`execute (``cmd``, ``arg``) `  
spawns a process specified by `cmd` with command-line arguments represented by the string `arg` and returns a handle for the resulting process.

> **Implementation note:**
>
> This also corresponds to the use of `CreateProcess`. Redirection of the standard streams can be handled using the `hStdInput` and `hStdOutput` fields in the `STARTUPINFO` parameter.


<span id="SIG:WINDOWS.textInstreamOf:VAL"></span>
`textInstreamOf ``pr`` `
` binInstreamOf ``pr`` `  
These functions return a text or binary [`instream`](imperative-io.md#SIG:IMPERATIVE_IO.instream:TY:SPEC) connected to the standard output stream of the process `pr`.

Note that multiple calls to these functions on the same [`proc`](windows.md#SIG:WINDOWS.proc:TY:SPEC) value will result in multiple streams that all share the same underlying open file descriptor, which can lead to unpredictable effects because

<span id="SIG:WINDOWS.textOutstreamOf:VAL"></span>
`textOutstreamOf ``pr`` `
` binOutstreamOf ``pr`` `  
These functions return a text or binary [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) connected to the standard input stream of the process `pr`.

Note that multiple calls to these functions on the same [`proc`](windows.md#SIG:WINDOWS.proc:TY:SPEC) value will result in multiple streams that all share the same underlying open file descriptor, which can lead to unpredictable effects due to buffering.

<span id="SIG:WINDOWS.reap:VAL"></span>
`reap ``pr`` `  
closes the standard streams associated with `pr`, and then suspends the current process until the system process corresponding to `pr` terminates. It returns the exit status given by `pr` when it terminated. If `reap` is applied again to `pr`, it should immediately return the previous exit status.

> **Implementation note:**
>
> Typically, one cannot rely on the underlying operating system to provide the exit status of a terminated process after it has done so once. Thus, the exit status probably needs to be cached. Also note that `reap` should not return until the process being monitored has terminated. In particular, implementations should be careful not to return if the process has only been suspended.


<span id="SIG:WINDOWS.Status:STR"></span>
**`structure`**` Status`  
The [`Status`](windows.md#SIG:WINDOWS.Status:STR:SPEC) substructure defines the possible system-specific interpretations of [`OS.Process.status`](os-process.md#SIG:OS_PROCESS.status:TY:SPEC) values.

<span id="SIG:WINDOWS.fromStatus:VAL"></span>

`fromStatus ``s`` `

decodes the abstract exit status `s` into system-specific information.

<span id="SIG:WINDOWS.exit:VAL"></span>

`exit ``st`` `

executes all actions registered with [`OS.Process.atExit`](os-process.md#SIG:OS_PROCESS.atExit:VAL:SPEC), flushes and closes all I/O streams, then terminates the SML process with termination status `st`.

#### Examples

```repl
Windows.Reg.currentUser;;
```

#### See Also

> [`BIT_FLAGS`](bit-flags.md#BIT_FLAGS:SIG:SPEC), [`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC), [`OS.Process`](os.md#SIG:OS.Process:STR:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC), [`Time`](time.md#Time:STR:SPEC)

#### Discussion

This structure provides a minimal view of the system calls available across Microsoft operating systems. It focuses on managing the registry, and executing programs. The function [`Windows.findExecutable`](windows.md#SIG:WINDOWS.findExecutable:VAL:SPEC) and the facilities in the [`Config`](windows.md#SIG:WINDOWS.Config:STR:SPEC) substructure allow the programmer to determine if and where a program can be found on a given machine.

Future extensions of the Basis Library might give access to more features, either by including additional substructures or as a separate top-level module.

> **Rationale:**
>
> As usual, platform identification and exit status values are not handled by datatypes to allow for future extensions.
