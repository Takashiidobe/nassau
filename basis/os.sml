signature OS_PATH =
sig
  exception Path
  exception InvalidArc
  val parentArc : string
  val currentArc : string
  val fromString : string -> {isAbs : bool, vol : string, arcs : string list}
  val toString : {isAbs : bool, vol : string, arcs : string list} -> string
  val validVolume : {isAbs : bool, vol : string} -> bool
  val getVolume : string -> string
  val getParent : string -> string
  val splitDirFile : string -> {dir : string, file : string}
  val joinDirFile : {dir : string, file : string} -> string
  val dir : string -> string
  val file : string -> string
  val splitBaseExt : string -> {base : string, ext : string option}
  val joinBaseExt : {base : string, ext : string option} -> string
  val base : string -> string
  val ext : string -> string option
  val mkCanonical : string -> string
  val isCanonical : string -> bool
  val mkAbsolute : {path : string, relativeTo : string} -> string
  val mkRelative : {path : string, relativeTo : string} -> string
  val isAbsolute : string -> bool
  val isRelative : string -> bool
  val isRoot : string -> bool
  val concat : string * string -> string
  val fromUnixPath : string -> string
  val toUnixPath : string -> string
end

signature OS_PROCESS =
sig
  type status
  val success : status
  val failure : status
  val isSuccess : status -> bool
  val system : string -> status
  val atExit : (unit -> unit) -> unit
  val exit : status -> 'a
  val terminate : status -> 'a
  val getEnv : string -> string option
  val sleep : Time.time -> unit
end

structure OS =
struct
  type syserror = int

  exception SysErr of string * syserror option

  fun errorMsg code = Prim.osErrorMessage code

  val errorNames =
    [("E2BIG", 7), ("EACCES", 13), ("EADDRINUSE", 98), ("EADDRNOTAVAIL", 99),
     ("EAGAIN", 11), ("EALREADY", 114), ("EBADF", 9), ("EBUSY", 16),
     ("ECHILD", 10), ("ECONNREFUSED", 111), ("ECONNRESET", 104),
     ("EDEADLK", 35), ("EDOM", 33), ("EEXIST", 17), ("EFAULT", 14),
     ("EFBIG", 27), ("EINPROGRESS", 115), ("EINTR", 4), ("EINVAL", 22),
     ("EIO", 5), ("EISDIR", 21), ("EMFILE", 24), ("EMLINK", 31),
     ("ENAMETOOLONG", 36), ("ENFILE", 23), ("ENODEV", 19), ("ENOENT", 2),
     ("ENOEXEC", 8), ("ENOMEM", 12), ("ENOSPC", 28), ("ENOSYS", 38),
     ("ENOTCONN", 107), ("ENOTDIR", 20), ("ENOTEMPTY", 39), ("ENOTSOCK", 88),
     ("ENOTTY", 25), ("ENXIO", 6), ("EPERM", 1), ("EPIPE", 32),
     ("EPROTO", 71), ("ERANGE", 34), ("EROFS", 30), ("ESPIPE", 29),
     ("ESRCH", 3), ("ETIMEDOUT", 110), ("EXDEV", 18)]

  fun errorName code =
    case List.find (fn (_, number) => number = code) errorNames of
      SOME (name, _) => name
    | NONE => "E" ^ Int.toString code

  fun syserror name =
    case List.find (fn (known, _) => known = name) errorNames of
      SOME (_, number) => SOME number
    | NONE => NONE

  structure Process :>
  sig
    include OS_PROCESS where type status = int
    val runAtExit: unit -> unit
    val addCleanup: (unit -> unit) -> unit
  end =
  struct
    type status = int

    val hooks : (unit -> unit) list ref = ref []
    val cleanups : (unit -> unit) list ref = ref []
    val exiting = ref false

    fun atExit f = if !exiting then () else hooks := f :: !hooks

    fun addCleanup f = cleanups := f :: !cleanups

    fun drain actions =
      case !actions of
        [] => ()
      | f :: rest =>
          (actions := rest; (f () handle _ => ()); drain actions)

    fun runAtExit () = (exiting := true; drain hooks; drain cleanups)

    fun getEnv name =
      let val value = Prim.osGetenv name
      in if size value = 0 then NONE else SOME (String.extract (value, 1, NONE)) end

    val success = 0

    val failure = 1

    fun isSuccess status = status = 0

    fun system command =
      let val status = Prim.osSystem command
      in if status < 0 then raise SysErr (errorMsg (~status), SOME (~status)) else status end

    fun exit status = (runAtExit (); Prim.exit status)

    fun terminate status = Prim.terminate status

    fun sleep time = Prim.osSleep (Time.toReal time)
  end

  structure IO :
  sig
    eqtype iodesc
    val hash : iodesc -> word
    val compare : iodesc * iodesc -> order
    eqtype iodesc_kind
    structure Kind :
    sig
      val file : iodesc_kind
      val dir : iodesc_kind
      val symlink : iodesc_kind
      val tty : iodesc_kind
      val pipe : iodesc_kind
      val socket : iodesc_kind
      val device : iodesc_kind
    end
    val kind : iodesc -> iodesc_kind
    exception Poll
    eqtype poll_desc
    type poll_info
    val pollDesc : iodesc -> poll_desc option
    val pollToIODesc : poll_desc -> iodesc
    val pollIn : poll_desc -> poll_desc
    val pollOut : poll_desc -> poll_desc
    val pollPri : poll_desc -> poll_desc
    val poll : poll_desc list * Time.time option -> poll_info list
    val isIn : poll_info -> bool
    val isOut : poll_info -> bool
    val isPri : poll_info -> bool
    val infoToPollDesc : poll_info -> poll_desc
    val ofHandle : int -> iodesc
  end =
  struct
    type iodesc = int

    fun hash d = Word.fromInt d
    fun compare (a, b) = Int.compare (a, b)

    type iodesc_kind = int

    structure Kind =
    struct
      val file = 0
      val dir = 1
      val symlink = 2
      val tty = 3
      val pipe = 4
      val socket = 5
      val device = 6
    end

    fun check result =
      if result < 0 then raise SysErr (errorMsg (~result), SOME (~result)) else result

    fun kind d = check (Prim.osIoKind d)

    exception Poll

    type poll_desc = int * int
    type poll_info = int * int

    fun pollDesc d = if Prim.osIoKind d < 0 then NONE else SOME (d, 0)
    fun pollToIODesc (d, _) = d

    fun withEvent bit (d, events) =
      (d, if events div bit mod 2 = 1 then events else events + bit)

    val pollIn = withEvent 1
    val pollPri = withEvent 2
    val pollOut = withEvent 4

    fun specOf descs =
      String.concatWith ","
        (List.map (fn (d, e) => Int.toString d ^ ":" ^ Int.toString e) descs)

    fun split text =
      List.map (fn part => valOf (Int.fromString part))
        (String.fields (fn c => c = #",") text)

    fun poll (descs, timeout) =
      let
        val ms =
          case timeout of
            NONE => ~1
          | SOME t => Int.max (0, Time.toMilliseconds t)
        val reply = Prim.osPoll (specOf descs, ms)
      in
        if String.sub (reply, 0) = #"=" then
          let
            val revents = if null descs then [] else split (String.extract (reply, 1, NONE))
            fun pair ((d, _) :: ds, r :: rs) =
                  if r = 0 then pair (ds, rs) else (d, r) :: pair (ds, rs)
              | pair _ = []
          in pair (descs, revents) end
        else
          let val code = valOf (Int.fromString reply)
          in raise SysErr (errorMsg code, SOME code) end
      end

    fun has bit (_, revents) = revents div bit mod 2 = 1
    fun isIn info = has 1 info orelse has 16 info
    val isPri = has 2
    val isOut = has 4
    fun infoToPollDesc (d, _) = (d, 0)
    fun ofHandle d = d
  end

  structure Path : OS_PATH =
  struct
    exception Path
    exception InvalidArc

    val parentArc = ".."
    val currentArc = "."

    fun reverse xs =
      let fun loop ([], acc) = acc
            | loop (x :: rest, acc) = loop (rest, x :: acc)
      in loop (xs, []) end

    fun take (xs, n) =
      if n <= 0 then [] else
      case xs of [] => [] | x :: rest => x :: take (rest, n - 1)

    fun drop (xs, n) =
      if n <= 0 then xs else
      case xs of [] => [] | _ :: rest => drop (rest, n - 1)

    fun count xs =
      let fun loop ([], n) = n | loop (_ :: rest, n) = loop (rest, n + 1)
      in loop (xs, 0) end

    fun containsSlash [] = false
      | containsSlash (#"/" :: _) = true
      | containsSlash (_ :: rest) = containsSlash rest

    fun validArcs [] = true
      | validArcs (arc :: rest) =
          not (containsSlash (String.explode arc)) andalso validArcs rest

    fun join [] = ""
      | join [s] = s
      | join (s :: rest) = s ^ "/" ^ join rest

    fun splitChars cs =
      let
        fun loop ([], part, parts) = reverse (String.implode (reverse part) :: parts)
          | loop (#"/" :: rest, part, parts) =
              loop (rest, [], String.implode (reverse part) :: parts)
          | loop (c :: rest, part, parts) = loop (rest, c :: part, parts)
      in
        loop (cs, [], [])
      end

    fun fromString path =
      let
        val chars = String.explode path
      in
        case chars of
          #"/" :: rest =>
            {isAbs = true, vol = "", arcs = splitChars rest}
        | [] => {isAbs = false, vol = "", arcs = []}
        | _ => {isAbs = false, vol = "", arcs = splitChars chars}
      end

    fun validVolume ({vol, ...} : {isAbs : bool, vol : string}) = vol = ""

    fun toString {isAbs, vol, arcs} =
      if not (validArcs arcs) then raise InvalidArc
      else if vol <> "" orelse (not isAbs andalso
        (case arcs of "" :: _ => true | _ => false)) then raise Path
      else if isAbs then
        (case arcs of [] => "/" | _ => "/" ^ join arcs)
      else join arcs

    fun getVolume _ = ""

    fun splitDirFile path =
      let
        val cs = String.explode path
        fun lastSlash ([], _, idx) = idx
          | lastSlash (c :: rest, i, idx) =
              lastSlash (rest, i + 1, if c = #"/" then i else idx)
        val i = lastSlash (cs, 0, ~1)
      in
        if path = "" then {dir = "", file = ""}
        else if i = ~1 then {dir = "", file = path}
        else if i = 0 then {dir = "/", file = String.implode (drop (cs, 1))}
        else {dir = String.implode (take (cs, i)),
              file = String.implode (drop (cs, i + 1))}
      end

    fun joinDirFile {dir, file} =
      if containsSlash (String.explode file) then raise InvalidArc
      else if dir = "" then file
      else if dir = "/" then "/" ^ file
      else dir ^ "/" ^ file

    fun dir path = #dir (splitDirFile path)
    fun file path = #file (splitDirFile path)

    fun splitBaseExt path =
      let
        val f = file path
        fun findDot ([], _, found) = found
          | findDot (#"." :: rest, i, found) = findDot (rest, i + 1, if i > 0 then SOME i else found)
          | findDot (_ :: rest, i, found) = findDot (rest, i + 1, found)
      in
        case findDot (String.explode f, 0, NONE) of
          NONE => {base = path, ext = NONE}
        | SOME i =>
            let
              val baseFile = String.implode (take (String.explode f, i - 1))
              val extFile = String.implode (drop (String.explode f, i))
            in
              if extFile = "" then {base = path, ext = NONE}
              else {base = (let val d = dir path in
                         if d = "" then baseFile else joinDirFile {dir=d,file=baseFile}
                       end),
                    ext = SOME extFile}
            end
      end

    fun joinBaseExt {base, ext} =
      case ext of NONE => base | SOME "" => base | SOME e => base ^ "." ^ e
    fun base path = #base (splitBaseExt path)
    fun ext path = #ext (splitBaseExt path)

    fun isAbsolute path = String.size path > 0 andalso String.sub (path, 0) = #"/"
    fun isRelative path = not (isAbsolute path)

    fun normalize (isAbs, arcs) =
      let
        fun loop ([], acc) = reverse acc
          | loop ("" :: rest, acc) = loop (rest, acc)
          | loop ("." :: rest, acc) = loop (rest, acc)
          | loop (".." :: rest, x :: xs) =
              if x = ".." then loop (rest, ".." :: x :: xs) else loop (rest, xs)
          | loop (".." :: rest, []) =
              if isAbs then loop (rest, []) else loop (rest, [".."])
          | loop (x :: rest, acc) = loop (rest, x :: acc)
      in
        loop (arcs, [])
      end

    fun mkCanonical path =
      let val {isAbs, vol, arcs} = fromString path
          val normalized = normalize (isAbs, arcs)
          val result = toString {isAbs=isAbs, vol=vol, arcs=normalized}
      in if result = "" then "." else result end

    fun isCanonical path = path = mkCanonical path

    fun isRoot path = isCanonical path andalso isAbsolute path andalso path = "/"

    fun getParent path =
      if isRoot path then path
      else
        let val {isAbs, arcs, ...} = fromString path
            val parent =
              case reverse arcs of
                [] => [".."]
              | "" :: rest => reverse (".." :: rest)
              | ".." :: rest => reverse (".." :: ".." :: rest)
              | "." :: rest => reverse (".." :: rest)
              | _ :: rest => reverse rest
        in toString {isAbs=isAbs, vol="", arcs=parent} end

    fun concat (p, t) =
      if isAbsolute t then raise Path
      else
        let val {isAbs, arcs=a, ...} = fromString p
            val {arcs=b, ...} = fromString t
            val a' = case reverse a of "" :: rest => reverse rest | _ => a
        in toString {isAbs=isAbs, vol="", arcs=a' @ b} end

    fun mkAbsolute {path, relativeTo} =
      if isAbsolute path then path
      else if not (isAbsolute relativeTo) then raise Path
      else mkCanonical (concat (relativeTo, path))

    fun mkRelative {path, relativeTo} =
      if isRelative path then path
      else if not (isAbsolute relativeTo) then raise Path
      else
        let
          val {isAbs=pathAbs, vol=pathVol, arcs=pa} = fromString path
          val {isAbs=baseAbs, vol=baseVol, arcs=ra} =
            fromString (mkCanonical relativeTo)
          fun withoutRoot (true, "" :: arcs) = arcs
            | withoutRoot (_, arcs) = arcs
          val pa = withoutRoot (pathAbs, pa)
          val ra = withoutRoot (baseAbs, ra)
          val _ =
            if pathAbs andalso baseAbs andalso pathVol = baseVol then ()
            else raise Path
          fun common (x :: xs, y :: ys) = if x = y then common (xs, ys) else (x :: xs, y :: ys)
            | common (xs, ys) = (xs, ys)
          val (p, r) = common (pa, ra)
          fun parents (0, acc) = acc
            | parents (n, acc) = parents (n - 1, ".." :: acc)
          val result = parents (count r, []) @ p
          val result = if result = [] then ["."]
                       else case result of "" :: _ => "." :: result | _ => result
        in join result end

    fun fromUnixPath path = path
    fun toUnixPath path = path
  end
  structure FileSys :>
  sig
    type dirstream
    datatype access_mode = A_READ | A_WRITE | A_EXEC
    val openDir : string -> dirstream
    val readDir : dirstream -> string option
    val rewindDir : dirstream -> unit
    val closeDir : dirstream -> unit
    val isDir : string -> bool
    val fileSize : string -> int
    val mkDir : string -> unit
    val rmDir : string -> unit
    val remove : string -> unit
    val getDir : unit -> string
    val chDir : string -> unit
    val access : string * access_mode list -> bool
    val rename : {old : string, new : string} -> unit
    val modTime : string -> Time.time
    val setTime : string * Time.time option -> unit
    val isLink : string -> bool
    val readLink : string -> string
    val fullPath : string -> string
    val realPath : string -> string
    val tmpName : unit -> string
    eqtype file_id
    val fileId : string -> file_id
    val compare : file_id * file_id -> order
    val hash : file_id -> word
  end =
  struct
    type dirstream = int
    datatype access_mode = A_READ | A_WRITE | A_EXEC

    fun check result =
      if result < 0 then raise SysErr (errorMsg (~result), SOME (~result)) else result

    fun openDir path = check (Prim.osOpenDir path)
    fun readDir stream =
      let val name = Prim.osReadDir stream
      in if size name = 0 then NONE else SOME name end
    fun rewindDir stream = (check (Prim.osRewindDir stream); ())
    fun closeDir stream = (Prim.osClose stream; ())
    fun isDir path = check (Prim.osIsDir path) = 1
    fun fileSize path = check (Prim.osFileSize path)
    fun mkDir path = (check (Prim.osMkDir path); ())
    fun rmDir path = (check (Prim.osRmDir path); ())
    fun remove path = (check (Prim.osRemove path); ())
    fun getDir () = Prim.osGetDir ()
    fun chDir path = (check (Prim.osChDir path); ())

    fun accessBit A_READ = 1
      | accessBit A_WRITE = 2
      | accessBit A_EXEC = 4

    fun access (path, modes) =
      Prim.osAccess (path, List.foldl (fn (mode, bits) => bits + accessBit mode) 0 modes) = 1

    fun rename {old, new} = (check (Prim.osRename (old, new)); ())
    fun modTime path = Time.fromMicroseconds (check (Prim.osModTime path))

    fun setTime (path, time) =
      case time of
        NONE => (check (Prim.osSetTime (path, 0, 1)); ())
      | SOME t => (check (Prim.osSetTime (path, Time.toMicroseconds t, 0)); ())

    fun isLink path = check (Prim.osIsLink path) = 1

    fun textResult reply =
      if String.sub (reply, 0) = #"=" then String.extract (reply, 1, NONE)
      else
        let val code = valOf (Int.fromString reply)
        in raise SysErr (errorMsg code, SOME code) end

    fun readLink path = textResult (Prim.osReadLink path)
    fun fullPath path = textResult (Prim.osFullPath path)
    fun tmpName () = textResult (Prim.osTmpName ())

    type file_id = string

    fun fileId path = textResult (Prim.osFileId path)
    fun compare (a, b) = String.compare (a, b)

    fun hash id =
      List.foldl
        (fn (c, h) => Word.andb (h * 0w31 + Word.fromInt (Char.ord c), 0w1073741823))
        0w5381 (String.explode id)
    fun realPath path =
      let val full = fullPath path
      in
        if Path.isAbsolute path then full
        else Path.mkRelative {path = full, relativeTo = fullPath (getDir ())}
      end
  end

end

structure CommandLine =
struct
  fun split s =
    let
      fun go (i, start, acc) =
        if i = size s then rev (String.substring (s, start, i - start) :: acc)
        else if String.sub (s, i) = #"\000" then go (i + 1, i + 1, String.substring (s, start, i - start) :: acc)
        else go (i + 1, start, acc)
    in
      if size s = 0 then [] else go (0, 0, [])
    end

  fun name () =
    case split (Prim.osArgs ()) of
      name :: _ => name
    | [] => ""

  fun arguments () =
    case split (Prim.osArgs ()) of
      _ :: rest => rest
    | [] => []
end
