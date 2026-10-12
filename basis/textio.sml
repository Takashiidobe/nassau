structure TextIO :>
sig
  type vector = string
  type elem = char
  type instream
  type outstream
  val stdIn : instream
  val stdOut : outstream
  val stdErr : outstream
  val openIn : string -> instream
  val openOut : string -> outstream
  val openAppend : string -> outstream
  val closeIn : instream -> unit
  val closeOut : outstream -> unit
  val input : instream -> vector
  val input1 : instream -> elem option
  val inputN : instream * int -> vector
  val inputAll : instream -> vector
  val inputLine : instream -> string option
  val lookahead : instream -> elem option
  val endOfStream : instream -> bool
  val output : outstream * vector -> unit
  val output1 : outstream * elem -> unit
  val flushOut : outstream -> unit
  val print : string -> unit
end =
struct
  type vector = string
  type elem = char

  datatype instream =
    Instream of
      {name : string, fd : int, buffer : string ref, position : int ref, closed : bool ref}

  datatype outstream = Outstream of {name : string, fd : int, closed : bool ref}

  fun failure (name, function, code) =
    IO.Io {name = name, function = function, cause = OS.SysErr (OS.errorMsg (~code), SOME (~code))}

  fun closedError (name, function) =
    IO.Io {name = name, function = function, cause = IO.ClosedStream}

  fun instream (name, fd) =
    Instream {name = name, fd = fd, buffer = ref "", position = ref 0, closed = ref false}

  fun outstream (name, fd) =
    Outstream {name = name, fd = fd, closed = ref false}

  val stdIn = instream ("<stdIn>", 0)
  val stdOut = outstream ("<stdOut>", 1)
  val stdErr = outstream ("<stdErr>", 2)

  fun openIn path =
    let val fd = Prim.osOpen (path, 0)
    in if fd < 0 then raise failure (path, "TextIO.openIn", fd) else instream (path, fd) end

  fun openFor (function, mode) path =
    let val fd = Prim.osOpen (path, mode)
    in if fd < 0 then raise failure (path, function, fd) else outstream (path, fd) end

  val openOut = openFor ("TextIO.openOut", 1)
  val openAppend = openFor ("TextIO.openAppend", 2)

  fun closeIn (Instream {fd, closed, ...}) =
    if !closed then () else (closed := true; ignore (Prim.osClose fd))

  fun closeOut (Outstream {fd, closed, ...}) =
    if !closed then () else (closed := true; ignore (Prim.osClose fd))

  fun fill (Instream {fd, buffer, position, closed, ...}) =
    if !position < size (!buffer) then true
    else if !closed then false
    else
      let val chunk = Prim.osRead (fd, 4096)
      in buffer := chunk; position := 0; size chunk > 0 end

  fun input (stream as Instream {buffer, position, ...}) =
    if fill stream then
      let val chunk = String.extract (!buffer, !position, NONE)
      in position := size (!buffer); chunk end
    else ""

  fun lookahead (stream as Instream {buffer, position, ...}) =
    if fill stream then SOME (Prim.stringSub (!buffer, !position)) else NONE

  fun input1 (stream as Instream {buffer, position, ...}) =
    if fill stream then
      let val c = Prim.stringSub (!buffer, !position)
      in position := !position + 1; SOME c end
    else NONE

  fun endOfStream stream = not (fill stream)

  fun inputAll stream =
    let
      fun loop acc =
        case input stream of
          "" => String.concat (rev acc)
        | chunk => loop (chunk :: acc)
    in loop [] end

  fun inputN (stream as Instream {buffer, position, ...}, n) =
    let
      fun loop (wanted, acc) =
        if wanted <= 0 orelse not (fill stream) then String.concat (rev acc)
        else
          let
            val available = size (!buffer) - !position
            val count = if wanted < available then wanted else available
            val chunk = String.extract (!buffer, !position, SOME count)
          in position := !position + count; loop (wanted - count, chunk :: acc) end
    in if n < 0 then raise Size else loop (n, []) end

  fun inputLine (stream as Instream {buffer, position, ...}) =
    let
      fun collect acc =
        if not (fill stream) then String.concat (rev acc) ^ "\n"
        else
          let
            val b = !buffer
            val start = !position
            fun find i =
              if i >= size b then NONE
              else if Prim.stringSub (b, i) = #"\n" then SOME i
              else find (i + 1)
          in
            case find start of
              SOME i =>
                (position := i + 1;
                 String.concat (rev (String.extract (b, start, SOME (i + 1 - start)) :: acc)))
            | NONE =>
                (position := size b;
                 collect (String.extract (b, start, NONE) :: acc))
          end
    in if fill stream then SOME (collect []) else NONE end

  fun output (Outstream {name, fd, closed}, data) =
    if !closed then raise closedError (name, "TextIO.output")
    else
      let val result = Prim.osWrite (fd, data)
      in if result < 0 then raise failure (name, "TextIO.output", result) else () end

  fun output1 (stream, c) = output (stream, Prim.charToString c)

  fun flushOut _ = ()

  val print = print
end
