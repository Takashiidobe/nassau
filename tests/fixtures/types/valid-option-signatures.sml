(* The Basis signatures of the common Option functions. *)
val _ = Option.valOf : 'a option -> 'a
val _ = Option.getOpt : 'a option * 'a -> 'a
val _ = Option.isSome : 'a option -> bool
val _ = Option.map : ('a -> 'b) -> 'a option -> 'b option
val _ = Option.join : 'a option option -> 'a option
val _ = Option.filter : ('a -> bool) -> 'a -> 'a option
val _ = Option.mapPartial : ('a -> 'b option) -> 'a option -> 'b option
val _ = Option.app : ('a -> unit) -> 'a option -> unit
val _ = valOf : 'a option -> 'a
val _ = getOpt : 'a option * 'a -> 'a
val _ = isSome : 'a option -> bool
(* CHECK-RUN-EXIT: 0 *)
