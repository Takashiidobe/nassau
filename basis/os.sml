structure Posix =
struct
  structure Process =
  struct
    fun exit (status: Word8.word) =
      Prim.exit (Word8.toInt status)
  end
end

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

structure OS =
struct
  structure Process :>
  sig
    type status
    val success: status
    val failure: status
    val isSuccess: status -> bool
    val exit: status -> 'a
  end =
  struct
    type status = int

    val success = 0

    val failure = 1

    fun isSuccess status = status = 0

    fun exit status = Prim.exit status
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

    fun validVolume {vol, ...} = vol = ""

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

    fun isRoot path = isCanonical path andalso isAbsolute path andalso path = "/"

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
end
