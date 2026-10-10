structure Word8VectorSlice :> MONO_VECTOR_SLICE
where type vector = Word8Vector.vector
where type elem = Word8.word =
struct
  type elem = Word8.word
  type vector = Word8Vector.vector
  type slice = elem VectorSlice.slice
  val length = VectorSlice.length
  val sub = VectorSlice.sub
  val full = VectorSlice.full
  val slice = VectorSlice.slice
  val subslice = VectorSlice.subslice
  val base = VectorSlice.base
  val vector = VectorSlice.vector
  val concat = VectorSlice.concat
  val isEmpty = VectorSlice.isEmpty
  val getItem = VectorSlice.getItem
  val appi = VectorSlice.appi
  val app = VectorSlice.app
  val mapi = VectorSlice.mapi
  val map = VectorSlice.map
  val foldli = VectorSlice.foldli
  val foldr = VectorSlice.foldr
  val foldl = VectorSlice.foldl
  val foldri = VectorSlice.foldri
  val findi = VectorSlice.findi
  val find = VectorSlice.find
  val exists = VectorSlice.exists
  val all = VectorSlice.all
  val collate = VectorSlice.collate
end

structure Word8ArraySlice :> MONO_ARRAY_SLICE
where type vector = Word8Vector.vector
where type vector_slice = Word8VectorSlice.slice
where type array = Word8Array.array
where type elem = Word8.word =
struct
  type elem = Word8.word
  type array = Word8Array.array
  type slice = elem ArraySlice.slice
  type vector = Word8Vector.vector
  type vector_slice = Word8VectorSlice.slice
  val length = ArraySlice.length
  val sub = ArraySlice.sub
  val update = ArraySlice.update
  val full = ArraySlice.full
  val slice = ArraySlice.slice
  val subslice = ArraySlice.subslice
  val base = ArraySlice.base
  val vector = ArraySlice.vector
  val copy = ArraySlice.copy
  val copyVec = ArraySlice.copyVec
  val isEmpty = ArraySlice.isEmpty
  val getItem = ArraySlice.getItem
  val appi = ArraySlice.appi
  val app = ArraySlice.app
  val modifyi = ArraySlice.modifyi
  val modify = ArraySlice.modify
  val foldli = ArraySlice.foldli
  val foldr = ArraySlice.foldr
  val foldl = ArraySlice.foldl
  val foldri = ArraySlice.foldri
  val findi = ArraySlice.findi
  val find = ArraySlice.find
  val exists = ArraySlice.exists
  val all = ArraySlice.all
  val collate = ArraySlice.collate
end

structure CharArraySlice :> MONO_ARRAY_SLICE
where type vector = CharVector.vector
where type vector_slice = CharVectorSlice.slice
where type array = CharArray.array
where type elem = char =
struct
  type elem = char
  type array = CharArray.array
  type slice = elem ArraySlice.slice
  type vector = CharVector.vector
  type vector_slice = CharVectorSlice.slice
  val length = ArraySlice.length
  val sub = ArraySlice.sub
  val update = ArraySlice.update
  val full = ArraySlice.full
  val slice = ArraySlice.slice
  val subslice = ArraySlice.subslice
  val base = ArraySlice.base
  fun vector s = CharVector.tabulate (length s, fn i => sub (s, i))
  val copy = ArraySlice.copy
  fun copyVec {src, dst, di} =
    let val count = CharVectorSlice.length src
        val dstLen = CharArray.length dst
        val _ = if di < 0 orelse di > dstLen orelse count > dstLen - di then raise Subscript else ()
        fun loop i = if i = count then () else (CharArray.update (dst, di + i, CharVectorSlice.sub (src, i)); loop (i + 1))
    in loop 0 end
  val isEmpty = ArraySlice.isEmpty
  val getItem = ArraySlice.getItem
  val appi = ArraySlice.appi
  val app = ArraySlice.app
  val modifyi = ArraySlice.modifyi
  val modify = ArraySlice.modify
  val foldli = ArraySlice.foldli
  val foldr = ArraySlice.foldr
  val foldl = ArraySlice.foldl
  val foldri = ArraySlice.foldri
  val findi = ArraySlice.findi
  val find = ArraySlice.find
  val exists = ArraySlice.exists
  val all = ArraySlice.all
  val collate = ArraySlice.collate
end

structure Word8Array2 :> MONO_ARRAY2
where type vector = Word8Vector.vector
where type elem = Word8.word =
struct
  type elem = Word8.word
  type array = elem Array2.array
  type vector = Word8Vector.vector
  type region = {base : array, row : int, col : int, nrows : int option, ncols : int option}
  datatype traversal = datatype Array2.traversal
  val array = Array2.array
  val fromList = Array2.fromList
  val tabulate = Array2.tabulate
  val sub = Array2.sub
  val update = Array2.update
  val dimensions = Array2.dimensions
  val nCols = Array2.nCols
  val nRows = Array2.nRows
  fun row (a, i) =
    if i < 0 orelse i >= nRows a then raise Subscript
    else Word8Vector.tabulate (nCols a, fn j => sub (a, i, j))
  fun column (a, j) =
    if j < 0 orelse j >= nCols a then raise Subscript
    else Word8Vector.tabulate (nRows a, fn i => sub (a, i, j))
  val copy = Array2.copy
  val appi = Array2.appi
  val app = Array2.app
  val foldi = Array2.foldi
  val fold = Array2.fold
  val modifyi = Array2.modifyi
  val modify = Array2.modify
end

structure CharArray2 :> MONO_ARRAY2
where type vector = CharVector.vector
where type elem = char =
struct
  type elem = char
  type array = elem Array2.array
  type vector = CharVector.vector
  type region = {base : array, row : int, col : int, nrows : int option, ncols : int option}
  datatype traversal = datatype Array2.traversal
  val array = Array2.array
  val fromList = Array2.fromList
  val tabulate = Array2.tabulate
  val sub = Array2.sub
  val update = Array2.update
  val dimensions = Array2.dimensions
  val nCols = Array2.nCols
  val nRows = Array2.nRows
  fun row (a, i) =
    if i < 0 orelse i >= nRows a then raise Subscript
    else CharVector.tabulate (nCols a, fn j => sub (a, i, j))
  fun column (a, j) =
    if j < 0 orelse j >= nCols a then raise Subscript
    else CharVector.tabulate (nRows a, fn i => sub (a, i, j))
  val copy = Array2.copy
  val appi = Array2.appi
  val app = Array2.app
  val foldi = Array2.foldi
  val fold = Array2.fold
  val modifyi = Array2.modifyi
  val modify = Array2.modify
end
