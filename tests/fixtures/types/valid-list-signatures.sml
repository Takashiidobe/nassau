(* The Basis signatures of the common List functions. *)
val _ = List.map : ('a -> 'b) -> 'a list -> 'b list
val _ = List.foldl : ('a * 'b -> 'b) -> 'b -> 'a list -> 'b
val _ = List.foldr : ('a * 'b -> 'b) -> 'b -> 'a list -> 'b
val _ = List.app : ('a -> unit) -> 'a list -> unit
val _ = List.find : ('a -> bool) -> 'a list -> 'a option
val _ = List.exists : ('a -> bool) -> 'a list -> bool
val _ = List.all : ('a -> bool) -> 'a list -> bool
val _ = List.rev : 'a list -> 'a list
val _ = List.concat : 'a list list -> 'a list
val _ = List.take : 'a list * int -> 'a list
val _ = List.drop : 'a list * int -> 'a list
val _ = List.null : 'a list -> bool
val _ = List.length : 'a list -> int
val _ = List.hd : 'a list -> 'a
val _ = List.tl : 'a list -> 'a list
(* CHECK-RUN-EXIT: 0 *)
