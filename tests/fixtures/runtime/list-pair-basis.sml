fun check (name, ok) = print (name ^ (if ok then " ok\n" else " wrong\n"))
val () = check ("zip", ListPair.zip ([1, 2], ["a"]) = [(1, "a")])
  and () = check ("zip reverse", ListPair.zip ([1], ["a", "b"]) = [(1, "a")])
val () = check ("zipEq", ListPair.zipEq ([1, 2], ["a", "b"]) = [(1, "a"), (2, "b")])
val () = check ("unzip", ListPair.unzip [(1, "a"), (2, "b")] = ([1, 2], ["a", "b"]))
val () = check ("map", ListPair.map (op +) ([1, 2], [10]) = [11])
val () = check ("mapEq", ListPair.mapEq (op +) ([1, 2], [10, 20]) = [11, 22])
fun collect (x, y, acc) = (x, y) :: acc
val () = check ("foldl", ListPair.foldl collect [] ([1, 2, 3], [4, 5]) = [(2, 5), (1, 4)])
val () = check ("foldr", ListPair.foldr collect [] ([1, 2], [4, 5, 6]) = [(1, 4), (2, 5)])
val () = check ("foldlEq", ListPair.foldlEq collect [] ([1, 2], [4, 5]) = [(2, 5), (1, 4)])
val () = check ("foldrEq", ListPair.foldrEq collect [] ([1, 2], [4, 5]) = [(1, 4), (2, 5)])
val calls : int list ref = ref []
fun visit (x, y) = calls := !calls @ [x + y]
val () = ListPair.app visit ([1, 2, 3], [10, 20])
val () = ListPair.appEq visit ([3], [30])
val () = check ("app order", !calls = [11, 22, 33])
fun unequal f = (f (); false) handle ListPair.UnequalLengths => true
val () = check ("unequal", unequal (fn () => ListPair.zipEq ([1], []))
  andalso unequal (fn () => ListPair.zipEq ([], [1]))
  andalso unequal (fn () => ListPair.mapEq (op +) ([], [1]))
  andalso unequal (fn () => ListPair.foldlEq collect [] ([1], []))
  andalso unequal (fn () => ListPair.foldrEq collect [] ([], [1])))
val () = calls := []
val () = check ("appEq lazy", unequal (fn () => ListPair.appEq visit ([1, 2], [10])) andalso !calls = [11])
val () = calls := []
val () = check ("mapEq lazy", unequal (fn () => ListPair.mapEq (fn pair => (visit pair; 0)) ([1], [10, 20])) andalso !calls = [11])
val () = calls := []
val () = check ("foldlEq lazy", unequal (fn () => ListPair.foldlEq (fn (x, y, a) => (visit (x, y); a)) 0 ([1, 2], [10])) andalso !calls = [11])
val () = calls := []
val () = check ("foldrEq lazy", unequal (fn () => ListPair.foldrEq (fn (x, y, a) => (visit (x, y); a)) 0 ([1], [10, 20])) andalso !calls = [])
fun stop _ = raise Fail "unexpected callback"
val () = check ("empty", ListPair.zip ([], [1]) = []
  andalso ListPair.zipEq ([], []) = []
  andalso ListPair.unzip [] = ([], [])
  andalso ListPair.map stop ([], [1]) = []
  andalso ListPair.mapEq stop ([], []) = []
  andalso ListPair.foldl stop 7 ([], [1]) = 7
  andalso ListPair.foldr stop 7 ([1], []) = 7
  andalso ListPair.foldlEq stop 7 ([], []) = 7
  andalso ListPair.foldrEq stop 7 ([], []) = 7
  andalso ListPair.all stop ([], [1])
  andalso not (ListPair.exists stop ([1], []))
  andalso ListPair.allEq stop ([], [])
  andalso not (ListPair.allEq stop ([], [1])))
val () = ListPair.app stop ([], [1])
val () = ListPair.appEq stop ([], [])
val () = check ("predicates", ListPair.all (op =) ([1], [1, 2])
  andalso ListPair.exists (op =) ([1, 2], [3, 2])
  andalso ListPair.allEq (op =) ([1, 2], [1, 2])
  andalso not (ListPair.allEq (op =) ([1], [1, 2])))
val count = ref 0
fun predicate result _ = (count := !count + 1; result)
val () = check ("short circuit", not (ListPair.all (predicate false) ([1, 2], [3, 4]))
  andalso ListPair.exists (predicate true) ([1, 2], [3, 4])
  andalso not (ListPair.allEq (predicate false) ([1, 2], [3, 4]))
  andalso !count = 3)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: zip ok *)
(* CHECK-STDOUT-NEXT: zip reverse ok *)
(* CHECK-STDOUT-NEXT: zipEq ok *)
(* CHECK-STDOUT-NEXT: unzip ok *)
(* CHECK-STDOUT-NEXT: map ok *)
(* CHECK-STDOUT-NEXT: mapEq ok *)
(* CHECK-STDOUT-NEXT: foldl ok *)
(* CHECK-STDOUT-NEXT: foldr ok *)
(* CHECK-STDOUT-NEXT: foldlEq ok *)
(* CHECK-STDOUT-NEXT: foldrEq ok *)
(* CHECK-STDOUT-NEXT: app order ok *)
(* CHECK-STDOUT-NEXT: unequal ok *)
(* CHECK-STDOUT-NEXT: appEq lazy ok *)
(* CHECK-STDOUT-NEXT: mapEq lazy ok *)
(* CHECK-STDOUT-NEXT: foldlEq lazy ok *)
(* CHECK-STDOUT-NEXT: foldrEq lazy ok *)
(* CHECK-STDOUT-NEXT: empty ok *)
(* CHECK-STDOUT-NEXT: predicates ok *)
(* CHECK-STDOUT-NEXT: short circuit ok *)
