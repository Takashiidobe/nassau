structure Posix =
struct
  fun check result =
    if result < 0 then raise OS.SysErr (OS.errorMsg (~result), SOME (~result))
    else result

  fun textOf reply =
    if String.sub (reply, 0) = #"=" then String.extract (reply, 1, NONE)
    else
      let val code = valOf (Int.fromString reply)
      in raise OS.SysErr (OS.errorMsg code, SOME code) end

  fun terminated [] = ""
    | terminated (x :: rest) = x ^ "\000" ^ terminated rest

  fun fields text =
    let
      fun loop (i, start, acc) =
        if i >= size text then List.rev acc
        else if String.sub (text, i) = #"\000" then
          loop (i + 1, i + 1, String.substring (text, start, i - start) :: acc)
        else loop (i + 1, start, acc)
    in loop (0, 0, []) end

  structure Signal :>
  sig
    eqtype signal
    val toWord : signal -> SysWord.word
    val fromWord : SysWord.word -> signal
    val abrt : signal
    val alrm : signal
    val bus : signal
    val chld : signal
    val cont : signal
    val fpe : signal
    val hup : signal
    val ill : signal
    val int : signal
    val kill : signal
    val pipe : signal
    val quit : signal
    val segv : signal
    val stop : signal
    val term : signal
    val tstp : signal
    val ttin : signal
    val ttou : signal
    val usr1 : signal
    val usr2 : signal
  end =
  struct
    type signal = int

    fun toWord s = SysWord.fromInt s
    fun fromWord w = SysWord.toInt w

    fun number index = check (Prim.posixInt (20, index, 0))

    val hup = number 0
    val int = number 1
    val quit = number 2
    val ill = number 3
    val abrt = number 4
    val fpe = number 5
    val kill = number 6
    val pipe = number 7
    val alrm = number 8
    val term = number 9
    val segv = number 10
    val bus = number 11
    val usr1 = number 12
    val usr2 = number 13
    val chld = number 14
    val cont = number 15
    val stop = number 16
    val tstp = number 17
    val ttin = number 18
    val ttou = number 19
  end

  structure FileSys :>
  sig
    eqtype file_desc
    val wordToFD : SysWord.word -> file_desc
    val fdToWord : file_desc -> SysWord.word
  end =
  struct
    type file_desc = int
    fun wordToFD w = SysWord.toInt w
    fun fdToWord fd = SysWord.fromInt fd
  end

  structure Process :>
  sig
    eqtype pid
    val wordToPid : SysWord.word -> pid
    val pidToWord : pid -> SysWord.word
    val fork : unit -> pid option
    val exec : string * string list -> 'a
    val exece : string * string list * string list -> 'a
    val execp : string * string list -> 'a
    datatype waitpid_arg =
      W_ANY_CHILD
    | W_CHILD of pid
    | W_SAME_GROUP
    | W_GROUP of pid
    datatype exit_status =
      W_EXITED
    | W_EXITSTATUS of Word8.word
    | W_SIGNALED of Signal.signal
    | W_STOPPED of Signal.signal
    val fromStatus : OS.Process.status -> exit_status
    structure W :
    sig
      include BIT_FLAGS
      val untraced : flags
    end
    val wait : unit -> pid * exit_status
    val waitpid : waitpid_arg * W.flags list -> pid * exit_status
    val waitpid_nh : waitpid_arg * W.flags list -> (pid * exit_status) option
    val exit : Word8.word -> 'a
    datatype killpid_arg = K_PROC of pid | K_SAME_GROUP | K_GROUP of pid
    val kill : killpid_arg * Signal.signal -> unit
    val alarm : Time.time -> Time.time
    val sleep : Time.time -> Time.time
    val pause : unit -> unit
  end =
  struct
    type pid = int

    fun wordToPid w = SysWord.toInt w
    fun pidToWord p = SysWord.fromInt p

    fun fork () =
      let val result = check (Prim.posixInt (11, 0, 0))
      in if result = 0 then NONE else SOME result end

    fun failed code = raise OS.SysErr (OS.errorMsg (~code), SOME (~code))

    fun exec (path, args) = failed (Prim.posixExec (path, terminated args, "", 0))

    fun exece (path, args, env) =
      failed (Prim.posixExec (path, terminated args, terminated env, 1))

    fun execp (path, args) = failed (Prim.posixExec (path, terminated args, "", 2))

    datatype waitpid_arg =
      W_ANY_CHILD
    | W_CHILD of pid
    | W_SAME_GROUP
    | W_GROUP of pid

    datatype exit_status =
      W_EXITED
    | W_EXITSTATUS of Word8.word
    | W_SIGNALED of Signal.signal
    | W_STOPPED of Signal.signal

    fun fromStatus status =
      if status = 0 then W_EXITED else W_EXITSTATUS (Word8.fromInt status)

    structure W =
    struct
      type flags = SysWord.word
      val toWord = fn (f : flags) => f
      val fromWord = fn (w : SysWord.word) => w
      val untraced : flags = 0w2
      val all : flags = untraced
      fun flags fs = List.foldl SysWord.orb 0w0 fs
      fun intersect fs = List.foldl SysWord.andb all fs
      fun clear (a, b) = SysWord.andb (a, SysWord.notb b)
      fun allSet (a, b) = SysWord.andb (a, b) = a
      fun anySet (a, b) = SysWord.andb (a, b) <> 0w0
    end

    fun target W_ANY_CHILD = ~1
      | target (W_CHILD p) = p
      | target W_SAME_GROUP = 0
      | target (W_GROUP p) = ~p

    fun status () =
      let val value = Prim.posixInt (19, 0, 0)
      in
        case Prim.posixInt (18, 0, 0) of
          0 => if value = 0 then W_EXITED else W_EXITSTATUS (Word8.fromInt value)
        | 1 => W_SIGNALED (Signal.fromWord (SysWord.fromInt value))
        | _ => W_STOPPED (Signal.fromWord (SysWord.fromInt value))
      end

    fun wait () =
      let val pid = check (Prim.posixInt (17, ~1, 0)) in (pid, status ()) end

    fun waitFor (arg, flags, extra) =
      let
        val bits = SysWord.toInt (W.flags flags) + extra
        val pid = check (Prim.posixInt (17, target arg, bits))
      in if pid = 0 then NONE else SOME (pid, status ()) end

    fun waitpid (arg, flags) = valOf (waitFor (arg, flags, 0))
    fun waitpid_nh (arg, flags) = waitFor (arg, flags, 1)

    fun exit status = Prim.exit (Word8.toInt status)

    datatype killpid_arg = K_PROC of pid | K_SAME_GROUP | K_GROUP of pid

    fun kill (arg, signal) =
      let
        val pid =
          case arg of
            K_PROC p => p
          | K_SAME_GROUP => 0
          | K_GROUP p => ~p
      in ignore (check (Prim.posixInt (12, pid, SysWord.toInt (Signal.toWord signal)))) end

    fun seconds time = Int.max (0, Time.toSeconds time)

    fun alarm time =
      Time.fromSeconds (Prim.posixInt (13, seconds time, 0))

    fun sleep time =
      Time.fromSeconds (Prim.posixInt (23, seconds time, 0))

    fun pause () = ignore (Prim.posixInt (14, 0, 0))
  end

  structure ProcEnv :>
  sig
    eqtype uid
    eqtype gid
    val uidToWord : uid -> SysWord.word
    val wordToUid : SysWord.word -> uid
    val gidToWord : gid -> SysWord.word
    val wordToGid : SysWord.word -> gid
    val getpid : unit -> Process.pid
    val getppid : unit -> Process.pid
    val getuid : unit -> uid
    val geteuid : unit -> uid
    val getgid : unit -> gid
    val getegid : unit -> gid
    val setuid : uid -> unit
    val setgid : gid -> unit
    val getgroups : unit -> gid list
    val getlogin : unit -> string
    val getpgrp : unit -> Process.pid
    val setsid : unit -> Process.pid
    val setpgid : {pid : Process.pid option, pgid : Process.pid option} -> unit
    val uname : unit -> (string * string) list
    val time : unit -> Time.time
    val times :
      unit ->
        {elapsed : Time.time, utime : Time.time, stime : Time.time,
         cutime : Time.time, cstime : Time.time}
    val getenv : string -> string option
    val environ : unit -> string list
    val ctermid : unit -> string
    val ttyname : FileSys.file_desc -> string
    val isatty : FileSys.file_desc -> bool
    val sysconf : string -> SysWord.word
  end =
  struct
    type uid = int
    type gid = int

    fun uidToWord u = SysWord.fromInt u
    fun wordToUid w = SysWord.toInt w
    fun gidToWord g = SysWord.fromInt g
    fun wordToGid w = SysWord.toInt w

    fun pidOf n = Process.wordToPid (SysWord.fromInt (check n))
    fun numberOf pid = SysWord.toInt (Process.pidToWord pid)

    fun getpid () = pidOf (Prim.posixInt (0, 0, 0))
    fun getppid () = pidOf (Prim.posixInt (1, 0, 0))
    fun getuid () = check (Prim.posixInt (2, 0, 0))
    fun geteuid () = check (Prim.posixInt (3, 0, 0))
    fun getgid () = check (Prim.posixInt (4, 0, 0))
    fun getegid () = check (Prim.posixInt (5, 0, 0))
    fun setuid u = ignore (check (Prim.posixInt (6, u, 0)))
    fun setgid g = ignore (check (Prim.posixInt (7, g, 0)))

    fun getgroups () =
      let
        val count = check (Prim.posixInt (21, 0, 0))
        fun collect i =
          if i >= count then []
          else check (Prim.posixInt (22, count, i)) :: collect (i + 1)
      in collect 0 end

    fun getlogin () = textOf (Prim.posixText (1, 0))
    fun getpgrp () = pidOf (Prim.posixInt (8, 0, 0))
    fun setsid () = pidOf (Prim.posixInt (9, 0, 0))

    fun numberOf' pid = case pid of NONE => 0 | SOME p => numberOf p

    fun setpgid {pid, pgid} =
      ignore (check (Prim.posixInt (10, numberOf' pid, numberOf' pgid)))

    fun uname () =
      ListPair.zip
        (["sysname", "nodename", "release", "version", "machine"],
         fields (textOf (Prim.posixText (0, 0))))

    fun time () = Time.fromReal (Prim.posixReal 10)

    fun times () =
      let
        val elapsed = Prim.posixReal 0
        fun at i = Time.fromReal (Prim.posixReal i)
      in
        {elapsed = Time.fromReal elapsed, utime = at 1, stime = at 2,
         cutime = at 3, cstime = at 4}
      end

    fun getenv name = OS.Process.getEnv name
    fun environ () = fields (textOf (Prim.posixText (4, 0)))
    fun ctermid () = textOf (Prim.posixText (2, 0))
    fun ttyname fd = textOf (Prim.posixText (3, SysWord.toInt (FileSys.fdToWord fd)))
    fun isatty fd = Prim.posixInt (15, SysWord.toInt (FileSys.fdToWord fd), 0) = 1

    val sysconfNames =
      ["ARG_MAX", "CHILD_MAX", "CLK_TCK", "NGROUPS_MAX", "OPEN_MAX",
       "STREAM_MAX", "TZNAME_MAX", "JOB_CONTROL", "SAVED_IDS", "VERSION"]

    fun sysconf name =
      let
        fun find (_, []) = raise OS.SysErr (OS.errorMsg 22, SOME 22)
          | find (i, n :: rest) = if n = name then i else find (i + 1, rest)
      in SysWord.fromInt (check (Prim.posixInt (16, find (0, sysconfNames), 0))) end
  end
end

