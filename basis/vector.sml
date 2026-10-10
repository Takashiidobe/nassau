signature VECTOR =
sig
  eqtype 'a vector
  val maxLen : int
  val fromList : 'a list -> 'a vector
  val tabulate : int * (int -> 'a) -> 'a vector
  val length : 'a vector -> int
  val sub : 'a vector * int -> 'a
  val update : 'a vector * int * 'a -> 'a vector
  val concat : 'a vector list -> 'a vector
  val appi : (int * 'a -> unit) -> 'a vector -> unit
  val app : ('a -> unit) -> 'a vector -> unit
  val mapi : (int * 'a -> 'b) -> 'a vector -> 'b vector
  val map : ('a -> 'b) -> 'a vector -> 'b vector
  val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
  val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a vector -> 'b
  val foldl : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
  val foldr : ('a * 'b -> 'b) -> 'b -> 'a vector -> 'b
  val findi : (int * 'a -> bool) -> 'a vector -> (int * 'a) option
  val find : ('a -> bool) -> 'a vector -> 'a option
  val exists : ('a -> bool) -> 'a vector -> bool
  val all : ('a -> bool) -> 'a vector -> bool
  val collate : ('a * 'a -> order) -> 'a vector * 'a vector -> order
end

structure Vector :> VECTOR where type 'a vector = 'a vector =
struct
  type 'a vector = 'a vector

  val maxLen = Array.maxLen

  fun fromList values = Array.vector (Array.fromList values)
  fun tabulate (length, f) = Array.vector (Array.tabulate (length, f))
  fun length vector = Prim.vectorLength vector

  fun sub (vector, index) =
    if index < 0 orelse index >= length vector then raise Subscript
    else Prim.vectorSub (vector, index)

  fun update (vector, index, value) =
    if index < 0 orelse index >= length vector then raise Subscript
    else
      let
        val array = Array.tabulate (length vector, fn i => if i = index then value else sub (vector, i))
      in
        Array.vector array
      end

  fun toList vector =
    let
      fun loop (index, acc) =
        if index < 0 then acc else loop (index - 1, sub (vector, index) :: acc)
    in
      loop (length vector - 1, [])
    end

  fun concat vectors = fromList (List.concat (List.map toList vectors))

  fun appi f vector =
    let
      val n = length vector
      fun loop index =
        if index = n then () else (f (index, sub (vector, index)); loop (index + 1))
    in
      loop 0
    end

  fun app f vector = appi (fn (_, value) => f value) vector
  fun mapi f vector = tabulate (length vector, fn index => f (index, sub (vector, index)))
  fun map f vector = mapi (fn (_, value) => f value) vector

  fun foldli f init vector =
    let
      val n = length vector
      fun loop (index, acc) =
        if index = n then acc
        else loop (index + 1, f (index, sub (vector, index), acc))
    in
      loop (0, init)
    end

  fun foldri f init vector =
    let
      fun loop (index, acc) =
        if index < 0 then acc
        else loop (index - 1, f (index, sub (vector, index), acc))
    in
      loop (length vector - 1, init)
    end

  fun foldl f init vector = foldli (fn (_, value, acc) => f (value, acc)) init vector
  fun foldr f init vector = foldri (fn (_, value, acc) => f (value, acc)) init vector

  fun findi pred vector =
    let
      val n = length vector
      fun loop index =
        if index = n then NONE
        else
          let val value = sub (vector, index)
          in if pred (index, value) then SOME (index, value) else loop (index + 1) end
    in
      loop 0
    end

  fun find pred vector =
    case findi (fn (_, value) => pred value) vector of
      NONE => NONE
    | SOME (_, value) => SOME value

  fun exists pred vector =
    case find pred vector of NONE => false | SOME _ => true
  fun all pred vector = not (exists (not o pred) vector)

  fun collate cmp (left, right) =
    let
      val leftLength = length left
      val rightLength = length right
      val limit = if leftLength < rightLength then leftLength else rightLength
      fun loop index =
        if index = limit then Int.compare (leftLength, rightLength)
        else
          case cmp (sub (left, index), sub (right, index)) of
            EQUAL => loop (index + 1)
          | order => order
    in
      loop 0
    end
end
