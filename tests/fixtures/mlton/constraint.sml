(* mlton regression/constraint.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S = sig type ('a, 'b) t 
                  val A : 'a * int -> ('a, int) t
                  val pr : ('a -> string) -> ('b -> string) -> ('a, 'b) t -> string
              end

structure S = 
  struct
    datatype ('a, 'b) t = A of 'a * 'b
    fun pr pr_a pr_b (A(a,b)) = "A(" ^ pr_a a ^ "," ^ pr_b b ^ ")"
  end

structure S' = S : S

val s = S'.pr (fn s => s) (Int.toString) (S'.A("hello",5))

val _ = print (s ^ "\n") 
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: A(hello,5) *)
