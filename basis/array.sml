signature ARRAY =
sig
  eqtype 'a array
  type 'a vector
  val maxLen : int
  val array : int * 'a -> 'a array
  val fromList : 'a list -> 'a array
  val tabulate : int * (int -> 'a) -> 'a array
  val length : 'a array -> int
  val sub : 'a array * int -> 'a
  val update : 'a array * int * 'a -> unit
  val vector : 'a array -> 'a vector
  val copy : {src : 'a array, dst : 'a array, di : int} -> unit
  val copyVec : {src : 'a vector, dst : 'a array, di : int} -> unit
  val appi : (int * 'a -> unit) -> 'a array -> unit
  val app : ('a -> unit) -> 'a array -> unit
  val modifyi : (int * 'a -> 'a) -> 'a array -> unit
  val modify : ('a -> 'a) -> 'a array -> unit
  val foldli : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
  val foldri : (int * 'a * 'b -> 'b) -> 'b -> 'a array -> 'b
  val foldl : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
  val foldr : ('a * 'b -> 'b) -> 'b -> 'a array -> 'b
  val findi : (int * 'a -> bool) -> 'a array -> (int * 'a) option
  val find : ('a -> bool) -> 'a array -> 'a option
  val exists : ('a -> bool) -> 'a array -> bool
  val all : ('a -> bool) -> 'a array -> bool
  val collate : ('a * 'a -> order) -> 'a array * 'a array -> order
end

structure Array :> ARRAY where type 'a array = 'a array where type 'a vector = 'a vector =
struct
  type 'a array = 'a array
  type 'a vector = 'a vector

  val maxLen = 1073741823

  fun checkSize n = if n < 0 orelse n > maxLen then raise Size

  fun array (n, init) =
    (checkSize n; Prim.arrayNew (n, init))

  fun length array = Prim.arrayLength array

  fun sub (array, index) =
    if index < 0 orelse index >= length array then raise Subscript
    else Prim.arraySub (array, index)

  fun update (array, index, value) =
    if index < 0 orelse index >= length array then raise Subscript
    else Prim.arrayUpdate (array, index, value)

  fun fromList [] = Prim.arrayNew (0, ())
    | fromList (first :: rest) =
        let
          val result = Prim.arrayNew (List.length (first :: rest), first)
          fun fill (_, []) = ()
            | fill (index, value :: tail) =
                (update (result, index, value); fill (index + 1, tail))
        in
          fill (1, rest);
          result
        end

  fun tabulate (n, f) =
    (checkSize n;
     if n = 0 then Prim.arrayNew (0, ())
     else
       let
         val result = Prim.arrayNew (n, f 0)
         fun fill index =
           if index = n then ()
           else (update (result, index, f index); fill (index + 1))
       in
         fill 1;
         result
       end)

  fun vector array = Prim.arrayVector array

  fun copy {src, dst, di} =
    let
      val count = length src
      val targetLength = length dst
      val _ = if di < 0 orelse di > targetLength orelse count > targetLength - di
              then raise Subscript else ()
      val snapshot = vector src
      fun loop index =
        if index = count then ()
        else (update (dst, di + index, Prim.vectorSub (snapshot, index)); loop (index + 1))
    in
      loop 0
    end

  fun copyVec {src, dst, di} =
    let
      val count = Prim.vectorLength src
      val targetLength = length dst
      val _ = if di < 0 orelse di > targetLength orelse count > targetLength - di
              then raise Subscript else ()
      fun loop index =
        if index = count then ()
        else (update (dst, di + index, Prim.vectorSub (src, index)); loop (index + 1))
    in
      loop 0
    end

  fun appi f array =
    let
      val n = length array
      fun loop index =
        if index = n then () else (f (index, sub (array, index)); loop (index + 1))
    in
      loop 0
    end

  fun app f array = appi (fn (_, value) => f value) array

  fun modifyi f array =
    let
      val n = length array
      fun loop index =
        if index = n then ()
        else (update (array, index, f (index, sub (array, index))); loop (index + 1))
    in
      loop 0
    end

  fun modify f array = modifyi (fn (_, value) => f value) array

  fun foldli f init array =
    let
      val n = length array
      fun loop (index, acc) =
        if index = n then acc
        else loop (index + 1, f (index, sub (array, index), acc))
    in
      loop (0, init)
    end

  fun foldri f init array =
    let
      fun loop (index, acc) =
        if index < 0 then acc
        else loop (index - 1, f (index, sub (array, index), acc))
    in
      loop (length array - 1, init)
    end

  fun foldl f init array = foldli (fn (_, value, acc) => f (value, acc)) init array
  fun foldr f init array = foldri (fn (_, value, acc) => f (value, acc)) init array

  fun findi pred array =
    let
      val n = length array
      fun loop index =
        if index = n then NONE
        else
          let val value = sub (array, index)
          in if pred (index, value) then SOME (index, value) else loop (index + 1) end
    in
      loop 0
    end

  fun find pred array =
    case findi (fn (_, value) => pred value) array of
      NONE => NONE
    | SOME (_, value) => SOME value

  fun exists pred array =
    case find pred array of NONE => false | SOME _ => true
  fun all pred array = not (exists (not o pred) array)

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
