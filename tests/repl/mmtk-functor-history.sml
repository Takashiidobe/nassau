(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* GC-STRESS: 1 *)
fun grow 0 text = text | grow n text = grow (n - 1) (text ^ text);
val retained = grow 18 "r";
structure Captured = struct val text = retained end;
exception Saved of string;
val saved = Saved retained;
functor Original (X : sig end) = struct
  val text = Captured.text
  fun check e = case e of Saved text => size text | _ => 0
end;
functor Forward (X : sig end) = Original (X);
structure Captured = struct val text = "shadow" end;
exception Saved;
val retained = "shadow";
val payload = grow 18 "x";
functor Unused0 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused1 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused2 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused3 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused4 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused5 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused6 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused7 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused8 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused9 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused10 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused11 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused12 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused13 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused14 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused15 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused16 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused17 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused18 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused19 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused20 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused21 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused22 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused23 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused24 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused25 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused26 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused27 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused28 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused29 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused30 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused31 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused32 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused33 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused34 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused35 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused36 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused37 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused38 (X : sig end) = struct val value = 42 end;
val payload = grow 18 "x";
functor Unused39 (X : sig end) = struct val value = 42 end;
val payload = "released";
structure Result = Forward (struct end);
structure Answer = Unused0 (struct end);
val length = size Result.text;
val answer = Answer.value;
val checked = Result.check saved;
val () = if length = 262144 andalso answer = 42 andalso checked = 262144 then print "functor history reclaimed, dependencies retained\n" else raise Fail "functors";
(* CHECK-REPL: val length = 262144 : int *)
(* CHECK-REPL-NEXT: val answer = 42 : int *)
(* CHECK-REPL-NEXT: val checked = 262144 : int *)
(* CHECK-REPL-NEXT: functor history reclaimed, dependencies retained *)
(* CHECK-STDOUT: functor history reclaimed, dependencies retained *)
