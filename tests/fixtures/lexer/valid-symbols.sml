infix 6 ++
fun left ++ right = left + right
datatype item = A | B of int
type callback = int -> int
fun identity (value : 'a) = value
val record : {item : item, callback : callback} =
  {item = B (1 ++ 2), callback = fn n => n + 1}
val apostrophe' = identity 5
val selected = #item record
val {item = _, ...} = record
val dotted = List.length [1, 2]
val semicolon = 3; val afterSemicolon = 4
signature S = sig type t val value : t end
structure Opaque :> S = struct type t = int val value = 3 end
