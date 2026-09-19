structure Posix =
struct
  structure Process =
  struct
    fun exit (status : Word8.word) = Prim.exit (Word8.toInt status)
  end
end

structure OS =
struct
  structure Process :>
  sig
    type status
    val success : status
    val failure : status
    val isSuccess : status -> bool
    val exit : status -> 'a
  end =
  struct
    type status = int

    val success = 0

    val failure = 1

    fun isSuccess status = status = 0

    fun exit status = Prim.exit status
  end
end
