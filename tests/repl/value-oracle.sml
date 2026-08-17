datatype 'a box = Box of 'a | Empty;
val value = Box (SOME (1, "two words"));
val tuple = (#"\255", "\001\127\255", 0wxFF, true, 1.5);
val record = {a = [1,2], b = ref (SOME "ok")};
exception E of int * string;
val ex = E (3, "payload");
(* ORACLE-VALUES *)
(* CHECK-REPL: val value = Box (SOME (1,"two words")) : (int * string) option box *)
(* CHECK-REPL: val tuple = (#"\255","\^A\127\255",0wxFF,true,1.5) : char * string * word * bool * real *)
(* CHECK-REPL: val record = {a={{[[]}}1,2],b=ref (SOME "ok")} : {a:int list, b:string option ref} *)
(* CHECK-REPL: val ex = E (3,"payload") : exn *)
