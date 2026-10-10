signature LIST_PAIR =
sig
  exception UnequalLengths
  val zip : 'a list * 'b list -> ('a * 'b) list
  val zipEq : 'a list * 'b list -> ('a * 'b) list
  val unzip : ('a * 'b) list -> 'a list * 'b list
  val app : ('a * 'b -> unit) -> 'a list * 'b list -> unit
  val appEq : ('a * 'b -> unit) -> 'a list * 'b list -> unit
  val map : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
  val mapEq : ('a * 'b -> 'c) -> 'a list * 'b list -> 'c list
  val foldl : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
  val foldr : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
  val foldlEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
  val foldrEq : ('a * 'b * 'c -> 'c) -> 'c -> 'a list * 'b list -> 'c
  val all : ('a * 'b -> bool) -> 'a list * 'b list -> bool
  val exists : ('a * 'b -> bool) -> 'a list * 'b list -> bool
  val allEq : ('a * 'b -> bool) -> 'a list * 'b list -> bool
end

structure ListPair :> LIST_PAIR =
struct
  exception UnequalLengths

  fun zip (x :: xs, y :: ys) = (x, y) :: zip (xs, ys)
    | zip _ = []

  fun zipEq ([], []) = []
    | zipEq (x :: xs, y :: ys) = (x, y) :: zipEq (xs, ys)
    | zipEq _ = raise UnequalLengths

  fun unzip pairs =
    let
      fun go ([], xs, ys) = (List.rev xs, List.rev ys)
        | go ((x, y) :: rest, xs, ys) = go (rest, x :: xs, y :: ys)
    in
      go (pairs, [], [])
    end

  fun app f (x :: xs, y :: ys) = (f (x, y); app f (xs, ys))
    | app f _ = ()

  fun appEq f ([], []) = ()
    | appEq f (x :: xs, y :: ys) = (f (x, y); appEq f (xs, ys))
    | appEq f _ = raise UnequalLengths

  fun map f (x :: xs, y :: ys) =
        let val z = f (x, y)
        in z :: map f (xs, ys)
        end
    | map f _ = []

  fun mapEq f ([], []) = []
    | mapEq f (x :: xs, y :: ys) =
        let val z = f (x, y)
        in z :: mapEq f (xs, ys)
        end
    | mapEq f _ = raise UnequalLengths

  fun foldl f init (x :: xs, y :: ys) = foldl f (f (x, y, init)) (xs, ys)
    | foldl f init _ = init

  fun foldr f init (x :: xs, y :: ys) = f (x, y, foldr f init (xs, ys))
    | foldr f init _ = init

  fun foldlEq f init ([], []) = init
    | foldlEq f init (x :: xs, y :: ys) = foldlEq f (f (x, y, init)) (xs, ys)
    | foldlEq f init _ = raise UnequalLengths

  fun foldrEq f init ([], []) = init
    | foldrEq f init (x :: xs, y :: ys) = f (x, y, foldrEq f init (xs, ys))
    | foldrEq f init _ = raise UnequalLengths

  fun all p (x :: xs, y :: ys) = p (x, y) andalso all p (xs, ys)
    | all p _ = true

  fun exists p (x :: xs, y :: ys) = p (x, y) orelse exists p (xs, ys)
    | exists p _ = false

  fun allEq p ([], []) = true
    | allEq p (x :: xs, y :: ys) = p (x, y) andalso allEq p (xs, ys)
    | allEq p _ = false
end
