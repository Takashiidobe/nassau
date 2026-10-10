structure Word8Vector :> MONO_VECTOR
where type vector = Word8.word Vector.vector
where type elem = Word8.word =
struct
  type vector = Word8.word Vector.vector
  type elem = Word8.word
  val maxLen = Vector.maxLen
  val fromList = Vector.fromList
  val tabulate = Vector.tabulate
  val length = Vector.length
  val sub = Vector.sub
  val update = Vector.update
  val concat = Vector.concat
  val appi = Vector.appi
  val app = Vector.app
  val mapi = Vector.mapi
  val map = Vector.map
  val foldli = Vector.foldli
  val foldri = Vector.foldri
  val foldl = Vector.foldl
  val foldr = Vector.foldr
  val findi = Vector.findi
  val find = Vector.find
  val exists = Vector.exists
  val all = Vector.all
  val collate = Vector.collate
end

structure Word8Array :> MONO_ARRAY
where type array = Word8.word Array.array
where type vector = Word8Vector.vector
where type elem = Word8.word =
struct
  type array = Word8.word Array.array
  type vector = Word8Vector.vector
  type elem = Word8.word
  val maxLen = Array.maxLen
  val array = Array.array
  val fromList = Array.fromList
  val tabulate = Array.tabulate
  val length = Array.length
  val sub = Array.sub
  val update = Array.update
  fun vector a = Word8Vector.tabulate (length a, fn i => sub (a, i))
  fun copy {src, dst, di} =
    if src = dst then
      if di = 0 then () else raise Subscript
    else Array.copy {src = src, dst = dst, di = di}
  val copyVec = Array.copyVec
  val appi = Array.appi
  val app = Array.app
  val modifyi = Array.modifyi
  val modify = Array.modify
  val foldli = Array.foldli
  val foldri = Array.foldri
  val foldl = Array.foldl
  val foldr = Array.foldr
  val findi = Array.findi
  val find = Array.find
  val exists = Array.exists
  val all = Array.all
  val collate = Array.collate
end

structure CharArray :> MONO_ARRAY
where type array = char Array.array
where type vector = CharVector.vector
where type elem = char =
struct
  type array = char Array.array
  type vector = CharVector.vector
  type elem = char
  val maxLen = Array.maxLen
  val array = Array.array
  val fromList = Array.fromList
  val tabulate = Array.tabulate
  val length = Array.length
  val sub = Array.sub
  val update = Array.update
  fun vector a = CharVector.tabulate (length a, fn i => sub (a, i))
  fun copy {src, dst, di} =
    if src = dst then
      if di = 0 then () else raise Subscript
    else
      let val values = vector src
          val count = CharVector.length values
          val dstLen = length dst
          val _ = if di < 0 orelse di > dstLen orelse count > dstLen - di then raise Subscript else ()
          fun loop i = if i = count then () else (update (dst, di + i, CharVector.sub (values, i)); loop (i + 1))
      in loop 0 end
  fun copyVec {src, dst, di} =
    let val count = CharVector.length src
        val dstLen = length dst
        val _ = if di < 0 orelse di > dstLen orelse count > dstLen - di then raise Subscript else ()
        fun loop i = if i = count then () else (update (dst, di + i, CharVector.sub (src, i)); loop (i + 1))
    in loop 0 end
  val appi = Array.appi
  val app = Array.app
  val modifyi = Array.modifyi
  val modify = Array.modify
  val foldli = Array.foldli
  val foldri = Array.foldri
  val foldl = Array.foldl
  val foldr = Array.foldr
  val findi = Array.findi
  val find = Array.find
  val exists = Array.exists
  val all = Array.all
  val collate = Array.collate
end
