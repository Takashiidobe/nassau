(* nassau-ugc.25 *)
val x = (let val id:'a -> 'a = fn z => z in id id end; fn z => z:'a)
