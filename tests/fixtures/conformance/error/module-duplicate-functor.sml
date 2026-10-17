(* nassau-ugc.23 *)
functor F () = struct end and F () = struct end
(* CHECK-ERR: × duplicate functor name 'F' *)
(* CHECK-ERR: :2:45] *)
