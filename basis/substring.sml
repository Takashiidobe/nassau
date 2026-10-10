signature SUBSTRING =
sig
  type substring
  eqtype char
  eqtype string
  val sub : substring * int -> char
  val size : substring -> int
  val base : substring -> string * int * int
  val extract : string * int * int option -> substring
  val substring : string * int * int -> substring
  val full : string -> substring
  val string : substring -> string
  val isEmpty : substring -> bool
  val getc : substring -> (char * substring) option
  val first : substring -> char option
  val triml : int -> substring -> substring
  val trimr : int -> substring -> substring
  val slice : substring * int * int option -> substring
  val concat : substring list -> string
  val concatWith : string -> substring list -> string
  val explode : substring -> char list
  val isPrefix : string -> substring -> bool
  val isSubstring : string -> substring -> bool
  val isSuffix : string -> substring -> bool
  val compare : substring * substring -> order
  val collate : (char * char -> order) -> substring * substring -> order
  val splitl : (char -> bool) -> substring -> substring * substring
  val splitr : (char -> bool) -> substring -> substring * substring
  val splitAt : substring * int -> substring * substring
  val dropl : (char -> bool) -> substring -> substring
  val dropr : (char -> bool) -> substring -> substring
  val takel : (char -> bool) -> substring -> substring
  val taker : (char -> bool) -> substring -> substring
  val position : string -> substring -> substring * substring
  val span : substring * substring -> substring
  val translate : (char -> string) -> substring -> string
  val tokens : (char -> bool) -> substring -> substring list
  val fields : (char -> bool) -> substring -> substring list
  val app : (char -> unit) -> substring -> unit
  val foldl : (char * 'a -> 'a) -> 'a -> substring -> 'a
  val foldr : (char * 'a -> 'a) -> 'a -> substring -> 'a
end

structure Substring :> SUBSTRING
where type string = String.string
where type char = Char.char =
struct
  type string = String.string
  type char = Char.char
  type substring = string * int * int

  fun base ss = ss
  fun size (_, _, n) = n

  fun extract (s, i, len) =
    if i < 0 orelse i > String.size s then raise Subscript
    else
      let val n = case len of NONE => String.size s - i | SOME n => n
      in if n < 0 orelse n > String.size s - i then raise Subscript
         else (s, i, n)
      end

  fun substring (s, i, n) = extract (s, i, SOME n)
  fun full s = (s, 0, String.size s)
  fun string (s, i, n) = String.extract (s, i, SOME n)
  fun isEmpty ss = size ss = 0

  fun sub ((s, i, n), j) =
    if j < 0 orelse j >= n then raise Subscript else String.sub (s, i + j)

  fun getc ss =
    if isEmpty ss then NONE
    else SOME (sub (ss, 0), slice (ss, 1, NONE))

  fun first ss =
    case getc ss of NONE => NONE | SOME (c, _) => SOME c

  fun slice ((s, i, n), j, len) =
    if j < 0 orelse j > n then raise Subscript
    else
      let val count = case len of NONE => n - j | SOME count => count
      in if count < 0 orelse count > n - j then raise Subscript
         else (s, i + j, count)
      end

  fun triml count ss =
    let val n = if count < 0 then 0 else if count > size ss then size ss else count
    in slice (ss, n, NONE) end

  fun trimr count ss =
    let val n = if count < 0 then 0 else if count > size ss then size ss else count
    in slice (ss, 0, SOME (size ss - n)) end

  fun concat ss = String.concat (List.map string ss)
  fun concatWith sep ss = String.concatWith sep (List.map string ss)
  fun explode ss = String.explode (string ss)

  fun isPrefix prefix ss =
    let val n = String.size prefix
    in n <= size ss andalso String.substring (string ss, 0, n) = prefix end

  fun isSubstring pattern ss = String.isSubstring pattern (string ss)
  fun isSuffix suffix ss = String.isSuffix suffix (string ss)
  fun compare (a, b) = String.compare (string a, string b)
  fun collate cmp (a, b) = String.collate cmp (string a, string b)

  fun splitAt (ss, i) =
    if i < 0 orelse i > size ss then raise Subscript
    else (slice (ss, 0, SOME i), slice (ss, i, NONE))

  fun splitl pred ss =
    let fun loop i = if i < size ss andalso pred (sub (ss, i)) then loop (i + 1) else i
    in splitAt (ss, loop 0) end

  fun splitr pred ss =
    let fun loop i = if i > 0 andalso pred (sub (ss, i - 1)) then loop (i - 1) else i
    in splitAt (ss, loop (size ss)) end

  fun dropl pred ss = #2 (splitl pred ss)
  fun dropr pred ss = #1 (splitr pred ss)
  fun takel pred ss = #1 (splitl pred ss)
  fun taker pred ss = #2 (splitr pred ss)

  fun position pattern ss =
    let
      fun loop i =
        if i + String.size pattern > size ss then (ss, slice (ss, size ss, SOME 0))
        else if String.substring (string (slice (ss, i, NONE)), 0,
                                  String.size pattern) = pattern
        then splitAt (ss, i)
        else loop (i + 1)
    in loop 0 end

  fun span (left as (s, i, n), right as (s', j, m)) =
    if s <> s' orelse j + m < i then raise Span
    else (s, i, j + m - i)

  fun translate f ss = String.translate f (string ss)

  fun tokens pred (ss as (s, start, count)) =
    let
      fun skip i = if i < count andalso pred (String.sub (s, start + i)) then skip (i + 1) else i
      fun take i = if i < count andalso not (pred (String.sub (s, start + i))) then take (i + 1) else i
      fun loop (i, acc) =
        let val first = skip i
        in if first >= count then reverse acc
           else let val last = take first
                in loop (last, (s, start + first, last - first) :: acc) end
        end
      and reverse xs =
        let fun rev ([], acc) = acc | rev (x :: rest, acc) = rev (rest, x :: acc)
        in rev (xs, []) end
    in loop (0, []) end

  fun fields pred (ss as (s, start, count)) =
    let
      fun loop (i, fieldStart, acc) =
        if i = count then reverse ((s, start + fieldStart, i - fieldStart) :: acc)
        else if pred (String.sub (s, start + i)) then
          loop (i + 1, i + 1, (s, start + fieldStart, i - fieldStart) :: acc)
        else loop (i + 1, fieldStart, acc)
      and reverse xs =
        let fun rev ([], acc) = acc | rev (x :: rest, acc) = rev (rest, x :: acc)
        in rev (xs, []) end
    in loop (0, 0, []) end

  fun app f ss =
    let fun loop i = if i = size ss then () else (f (sub (ss, i)); loop (i + 1))
    in loop 0 end

  fun foldl f init ss =
    let fun loop (i, acc) = if i = size ss then acc else loop (i + 1, f (sub (ss, i), acc))
    in loop (0, init) end

  fun foldr f init ss =
    let fun loop (i, acc) = if i < 0 then acc else loop (i - 1, f (sub (ss, i), acc))
    in loop (size ss - 1, init) end
end
