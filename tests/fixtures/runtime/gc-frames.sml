(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* GC-STRESS: 1 *)
fun churn 0 = ()
  | churn n = let val garbage = ref [n] in if !garbage = [n] then churn (n - 1) else raise Fail "churn" end
fun frames 0 = (churn 50; 0)
  | frames n = let
      val text = "frame" ^ Int.toString n
      val cell = ref text
      val read = fn () => !cell
      val prior = read ()
      val result = frames (n - 1)
    in if read () = prior then result + size prior else raise Fail "frame" end
fun mutual n = let
    val text = ref ("mutual" ^ Int.toString n)
    fun even 0 = !text | even k = odd (k - 1)
    and odd 0 = !text | odd k = even (k - 1)
  in even n end
fun saved n = let
    exception Local of {read: unit -> string, number: int ref, weight: real}
    val cell = ref ("saved" ^ Int.toString n)
    val raised = Local {read = fn () => !cell, number = ref n, weight = 1.0 + 0.5}
  in (churn 50; raise raised)
     handle Local {read, number, weight} =>
       if !number = n andalso weight > 1.4 andalso weight < 1.6 then read () else raise Fail "saved"
  end
val result = frames 12
val caught = ((churn 10; 1 div 0) handle Div => 42)
val () = if result = 75 andalso caught = 42 then
    print (Int.toString result ^ ":" ^ mutual 20 ^ ":" ^ saved 9 ^ "\n")
  else raise Fail "result"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 75:mutual20:saved9 *)
