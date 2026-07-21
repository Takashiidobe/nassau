signature BASE = sig val x : int end
signature EXT = sig include BASE val y : int end
structure S : EXT = struct val x = 1 end
(* CHECK-ERR: × the structure does not provide value y, which the signature specifies *)
(* CHECK-ERR: :3:21] *)
