(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* GC-STRESS: 100 *)
val strings = ["" ^ "", "123" ^ "4567", "1234" ^ "5678", "12345678" ^ "12345678"];
val () = if strings = ["", "1234567", "12345678", "1234567812345678"] then () else raise Fail "string boundaries";
datatype node = End | Node of {next: node ref, text: string, weight: real};
fun make n = let val cell = ref n in fn delta => (cell := !cell + delta; !cell) end;
val counter = make 10;
val link = ref End;
val node = Node {next = link, text = "mmtk" ^ Int.toString (counter 1), weight = 1.0 + 1.5};
val () = link := node;
exception Saved of node;
val saved = Saved node;
fun allocate 0 = ()
  | allocate n = let val values = ref [n, n + 1] in
      if !values = [n, n + 1] then allocate (n - 1) else raise Fail "allocation"
    end;
val () = allocate 10000;
val result = ((raise saved) handle Saved (Node {next, text, weight}) =>
    if weight >= 2.5 andalso weight <= 2.5 then
      (case !next of Node {text = again, ...} =>
         text ^ ":" ^ again ^ ":" ^ Int.toString (counter 4)
       | End => raise Fail "cycle")
    else raise Fail "real");
val () = print (result ^ "\n");
(* CHECK-REPL: val counter = fn : int -> int *)
(* CHECK-REPL: val saved = Saved {{.*}} : exn *)
(* CHECK-REPL: val result = "mmtk11:mmtk11:15" : string *)
(* CHECK-REPL-NEXT: mmtk11:mmtk11:15 *)
(* CHECK-STDOUT: mmtk11:mmtk11:15 *)
