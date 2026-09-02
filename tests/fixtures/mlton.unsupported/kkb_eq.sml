(* mlton regression/kkb_eq.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(*kkb_eq.sml*)

(*

   kitknuth-bendixnewcopy.sml
 
 This is a revised version of knuth-bendix.sml in which 
    (a) val has been converted to fun for function values
    (b) exceptions that carry values have been avoided
    (c) functions have been moved around to pass fewer of them
        as parameters
    (d) long tail-recursions have been broken into batches of 1,
        with user-programmed copying between the batches

*)
val _ =
   let

(* 
signature KB =
  sig
    datatype term = Var of int | Term of string * term list
    datatype ordering = Greater | Equal | NotGE
    val rpo: (string -> string -> ordering) ->
           ((term * term -> ordering) -> term * term -> ordering) ->
           term * term -> ordering
    val lex_ext:  (term * term -> ordering) -> term * term -> ordering
    val kb_complete:
      (term * term -> bool) -> (int * (int * (term * term))) list ->
      ('a * ('b * (term * term))) list -> unit
    include BMARK
  end;
*)  
(*
structure Main : KB = 
  struct
*)
    fun length l = let
          fun j(k, nil) = k
            | j(k, a::x) = j(k+1,x)
          in
            j(0,l)
          end
    fun op @ (nil, l) = l
      | op @ (a::r, l) = a :: (r@l)
    fun rev l = let
          fun f (nil, h) = h
            | f (a::r, h) = f(r, a::h)
          in
            f(l,nil)
          end
    fun app f = let
          fun app_rec [] = ()
            | app_rec (a::L) = (f a; app_rec L)
        in
          app_rec
        end
    fun map f = let
          fun map_rec [] = []
            | map_rec (a::L) = f a :: map_rec L
          in
            map_rec
          end

(******* Quelques definitions du prelude CAML **************)

    exception Failure of string
    exception FailItList2
    exception FailTryFind
    exception FailFind
    exception FailChange
    exception FailReplace
    exception FailMatching
    exception FailUnify
    exception FailPretty
    exception Fail
    exception FailMrewrite1
    exception FailRemEQ
    exception FailMultExt
    exception FailLexExt
    exception FailKbComplettion

    fun failwith s = raise(Failure s)

    fun fst (x,y) = x
    and snd (x,y) = y


fun it_list f =
  let fun it_rec a [] = a
        | it_rec a (b::L) = it_rec (f a b) L
  in it_rec
  end


fun it_list2 f =
  let fun it_rec  a    []       []    = a
        | it_rec  a (a1::L1) (a2::L2) = it_rec (f a (a1,a2)) L1 L2
        | it_rec  _    _        _     = raise FailItList2
  in it_rec
  end

fun exists p =
  let fun exists_rec []  = false
        | exists_rec (a::L) = (p a) orelse (exists_rec L)
  in exists_rec
  end

fun for_all p =
  let fun for_all_rec []  = true
        | for_all_rec (a::L) = (p a) andalso (for_all_rec L)
  in for_all_rec
  end

fun rev_append   []    L  = L 
  | rev_append (x::L1) L2 = rev_append L1 (x::L2)

fun try_find f =
  let fun try_find_rec  []    = raise FailTryFind
        | try_find_rec (a::L) = (f a) handle _ => try_find_rec L
  in try_find_rec
  end

fun partition p =
  let fun part_rec   []   = ([],[])
        | part_rec (a::L) =
            let val (pos,neg) = part_rec L in
              if p a then  ((a::pos), neg) else (pos, (a::neg))
            end
  in part_rec
  end

(* 3- Les ensembles et les listes d'association *)

fun mem a =
  let fun mem_rec [] = false
        | mem_rec (b::L) = a = b orelse mem_rec L
  in mem_rec
  end

fun union L1 L2 =
  let fun union_rec [] = L2
        | union_rec (a::L) =
            if mem a L2 then union_rec L else a :: union_rec L
  in union_rec L1
  end

fun mem_assoc a =
  let fun mem_rec [] = false
        | mem_rec ((b,_)::L) = a = b orelse mem_rec L
  in mem_rec
  end

fun assoc a =
  let fun assoc_rec [] = raise FailFind
        | assoc_rec ((b,d)::L) = if a = b then d else assoc_rec L
  in assoc_rec
  end

(* 4- Les sorties *)

(* val print_string =  String.print; *)
(* Lars *)
fun print_string x = print x

(* val print_num = Integer.print; *)
(* Lars *)
local
  fun digit n = chr(ord #"0" + n)
  fun digits(n,acc) =
    if n >=0 andalso n<=9 then digit n:: acc
    else digits (n div 10, digit(n mod 10) :: acc)
  fun string(n) = implode(digits(n,[]))
in
  fun print_num n = print_string(string n)
end

(* fun print_newline () = String.print "\n"; *)
(* Lars *)
fun print_newline () = print "\n"

(* fun message s = (String.print s; String.print "\n"); *)
(* Lars *)
fun message s = (print s; print "\n")

(* 5- Les ensembles *)

fun union L1 =
  let fun union_rec [] = L1
        | union_rec (a::L) = if mem a L1 then union_rec L else a :: union_rec L
  in union_rec
  end

(****************** Term manipulations *****************)


datatype term
  = Var of int
  | Term of string * term list

(* Lars, from now on: seek on eq_X to see what I have modified *)

fun copy_term (Var n) = Var (n+0)
  | copy_term (Term(s, l)) = Term(s, map copy_term l)


fun vars (Var n) = [n]
  | vars (Term(_,L)) = vars_of_list L
and vars_of_list [] = []
  | vars_of_list (t::r) = union (vars t) (vars_of_list r)

fun substitute subst =
  let fun subst_rec (Term(oper,sons)) = Term(oper, map subst_rec sons)
        | subst_rec (t as (Var n))     = (assoc n subst) handle _ => t
  in subst_rec
  end

fun change f =
  let fun change_rec (h::t) n = if n = 1 then f h :: t
                                       else h :: change_rec t (n-1)
        | change_rec _ _      = raise FailChange
  in change_rec
  end

(* Term replacement replace M u N => M[u<-N] *)
fun replace M u N = 
  let fun reprec (_, []) = N
        | reprec (Term(oper,sons), (n::u)) =
             Term(oper, change (fn P => reprec(P,u)) sons n)
        | reprec _ = raise FailReplace
  in reprec(M,u)
  end

(* matching = - : (term -> term -> subst) *)
fun matching term1 term2 =
  let fun match_rec subst (Var v, M) =
          if mem_assoc v subst then
            if M = assoc v subst then subst else raise FailMatching
          else
            (v,M) :: subst
        | match_rec subst (Term(op1,sons1), Term(op2,sons2)) =
          if op1 = op2 then it_list2 match_rec subst sons1 sons2
                       else raise FailMatching
        | match_rec _ _ = raise FailMatching
  in match_rec [] (term1,term2)
  end

(* A naive unification algorithm *)

fun compsubst subst1 subst2 = 
  (map (fn (v,t) => (v, substitute subst1 t)) subst2) @ subst1

fun occurs n =
  let fun occur_rec (Var m) = m = n
        | occur_rec (Term(_,sons)) = exists occur_rec sons
  in occur_rec
  end

fun unify ((term1 as (Var n1)), term2) =
      if term1 = term2 then []
      else if occurs n1 term2 then raise FailUnify
      else [(n1,term2)]
  | unify (term1, Var n2) =
      if occurs n2 term1 then raise FailUnify
      else [(n2,term1)]
  | unify (Term(op1,sons1), Term(op2,sons2)) =
      if op1 = op2 then 
        it_list2 (fn s => fn (t1,t2) => compsubst (unify(substitute s t1,
                                                         substitute s t2)) s)
                 [] sons1 sons2
      else raise FailUnify

(* We need to print terms with variables independently from input terms
  obtained by parsing. We give arbitrary names v1,v2,... to their variables. *)

val INFIXES = ["+","*"]

fun pretty_term (Var n) =
      (print_string "v"; print_num n)
  | pretty_term (Term (oper,sons)) =
      if mem oper INFIXES then
        case sons of
            [s1,s2] =>
              (pretty_close s1; print_string oper; pretty_close s2)
          | _ =>
              raise FailPretty (* "pretty_term : infix arity <> 2"*)
      else
       (print_string oper;
        case sons of
             []   => ()
          | t::lt =>(print_string "(";
                     pretty_term t;
                     app (fn t => (print_string ","; pretty_term t)) lt;
                     print_string ")"))
and pretty_close (M as Term(oper, _)) =
      if mem oper INFIXES then
        (print_string "("; pretty_term M; print_string ")")
      else pretty_term M
  | pretty_close M = pretty_term M

(****************** Equation manipulations *************)

(* standardizes an equation so its variables are 1,2,... *)

fun mk_rule M N =
  let val all_vars = union (vars M) (vars N)
      val (k,subst) =
        it_list (fn (i,sigma) => fn v => (i+1,(v,Var(i))::sigma))
               (1,[]) all_vars
  in (k-1, (substitute subst M, substitute subst N))
  end

(* checks that rules are numbered in sequence and returns their number *)
fun check_rules x =
  it_list (fn n => fn (k,_) =>
             if k = n+1 then k
             else raise Fail (*failwith "Rule numbers not in sequence"*)
          ) 0 x

fun pretty_rule (k,(n,(M,N))) =
 (print_num k; print_string " : ";
  pretty_term M; print_string " = "; pretty_term N;
  print_newline())

fun pretty_rules l = app pretty_rule l

fun copy_rules [] = []
  | copy_rules ((k,(n,(M,N)))::rest) = (k+0,(n+0,(copy_term M, copy_term N))):: copy_rules rest

(****************** Rewriting **************************)

(* Top-level rewriting. Let eq:L=R be an equation, M be a term such that L<=M.
   With sigma = matching L M, we define the image of M by eq as sigma(R) *)
(* inserted eta; 03/07/1996-Martin *)
fun reduce L M t =
  substitute (matching L M) t

(* A more efficient version of can (rewrite1 (L,R)) for R arbitrary *)
fun reducible L =
  let
     fun redrec M =
        (matching L M; true)
        handle _ =>
           case M of Term(_,sons) => exists redrec sons
         |        _     => false
  in redrec
  end

(* mreduce : rules -> term -> term *)
fun mreduce rules M =
  let fun redex (_,(_,(L,R))) = reduce L M R in try_find redex rules end

(* One step of rewriting in leftmost-outermost strategy, with multiple rules *)
(* fails if no redex is found *)
(* mrewrite1 : rules -> term -> term *)
fun mrewrite1 rules =
  let
     fun rewrec M =
        (mreduce rules M)
        handle _ =>
           let fun tryrec [] = raise FailMrewrite1 (*failwith "mrewrite1"*)
                 | tryrec (son::rest) =
                   (rewrec son :: rest) handle _ => son :: tryrec rest
           in case M of
              Term(f, sons) => Term(f, tryrec sons)
            | _ => raise FailMrewrite1 (*failwith "mrewrite1"*)
           end
  in rewrec
  end

(* Iterating rewrite1. Returns a normal form. May loop forever *)
(* mrewrite_all : rules -> term -> term *)
fun mrewrite_all rules M =
  let
     fun rew_loop M =
        rew_loop(mrewrite1 rules M)  handle  _ => M
  in rew_loop M
  end

(*
pretty_term (mrewrite_all Group_rules M where M,_=<<A*(I(B)*B)>>);;
==> A*U
*)


(************************ Recursive Path Ordering ****************************)

datatype ordering = Greater | Equal | NotGE

fun ge_ord order pair = case order pair of NotGE => false | _ => true
and gt_ord order pair = case order pair of Greater => true | _ => false
and eq_ord order pair = case order pair of Equal => true | _ => false

fun rem_eq equiv =
  let fun remrec x [] = raise FailRemEQ (*failwith "rem_eq"*)
        | remrec x (y::l) = if equiv (x,y) then l else y :: remrec x l
  in remrec
  end

fun diff_eq equiv (x,y) =
  let fun diffrec (p as ([],_)) = p
        | diffrec ((h::t), y) =
            diffrec (t,rem_eq equiv h y) handle _ =>
              let val (x',y') = diffrec (t,y) in (h::x',y') end
  in if length x > length y then diffrec(y,x) else diffrec(x,y)
  end

(* multiset extension of order *)
fun mult_ext order (Term(_,sons1), Term(_,sons2)) =
      (case diff_eq (eq_ord order) (sons1,sons2) of
           ([],[]) => Equal
         | (l1,l2) =>
             if for_all (fn N => exists (fn M => order (M,N) = Greater) l1) l2
             then Greater else NotGE)
  | mult_ext order (_, _) = raise FailMultExt (*failwith "mult_ext"*)

(* lexicographic extension of order *)
fun lex_ext order ((M as Term(_,sons1)), (N as Term(_,sons2))) =
      let fun lexrec ([] , []) = Equal
            | lexrec ([] , _ ) = NotGE
            | lexrec ( _ , []) = Greater
            | lexrec (x1::l1, x2::l2) =
                case order (x1,x2) of
                  Greater => if for_all (fn N' => gt_ord order (M,N')) l2 
                             then Greater else NotGE
                | Equal => lexrec (l1,l2)
                | NotGE => if exists (fn M' => ge_ord order (M',N)) l1 
                           then Greater else NotGE
      in lexrec (sons1, sons2)
      end
  | lex_ext order _ = raise FailLexExt (*failwith "lex_ext"*)

(* recursive path ordering *)
fun Group_rules() = [
  (1, (1, (Term("*", [Term("U",[]), Var 1]), Var 1))),
  (2, (1, (Term("*", [Term("I",[Var 1]), Var 1]), Term("U",[])))),
  (3, (3, (Term("*", [Term("*", [Var 1, Var 2]), Var 3]),
           Term("*", [Var 1, Term("*", [Var 2, Var 3])]))))]

fun Geom_rules() = [
 (1,(1,(Term ("*",[(Term ("U",[])), (Var 1)]),(Var 1)))),
 (2,(1,(Term ("*",[(Term ("I",[(Var 1)])), (Var 1)]),(Term ("U",[]))))),
 (3,(3,(Term ("*",[(Term ("*",[(Var 1), (Var 2)])), (Var 3)]),
  (Term ("*",[(Var 1), (Term ("*",[(Var 2), (Var 3)]))]))))),
 (4,(0,(Term ("*",[(Term ("A",[])), (Term ("B",[]))]),
  (Term ("*",[(Term ("B",[])), (Term ("A",[]))]))))),
 (5,(0,(Term ("*",[(Term ("C",[])), (Term ("C",[]))]),(Term ("U",[]))))),
 (6,(0,
  (Term
   ("*",
    [(Term ("C",[])),
     (Term ("*",[(Term ("A",[])), (Term ("I",[(Term ("C",[]))]))]))]),
  (Term ("I",[(Term ("A",[]))]))))),
 (7,(0,
  (Term
   ("*",
    [(Term ("C",[])),
     (Term ("*",[(Term ("B",[])), (Term ("I",[(Term ("C",[]))]))]))]),
  (Term ("B",[])))))
]

fun Group_rank "U" = 0
  | Group_rank "*" = 1
  | Group_rank "I" = 2
  | Group_rank "B" = 3
  | Group_rank "C" = 4
  | Group_rank "A" = 5
  | Group_rank _ = 100  (*added, to avoid non-exhaustive patter (mads) *)

fun Group_precedence op1 op2 =
  let val r1 = Group_rank op1
      val r2 = Group_rank op2
  in
    if r1 = r2 then Equal else
    if r1 > r2 then Greater else NotGE
  end


fun rpo () =
  let fun rporec (M,N) =
    if M = N then Equal else 
      case M of
          Var m => NotGE
        | Term(op1,sons1) =>
            case N of
                Var n =>
                  if occurs n M then Greater else NotGE
              | Term(op2,sons2) =>
                  case (Group_precedence op1 op2) of
                      Greater =>
                        if for_all (fn N' => gt_ord rporec (M,N')) sons2
                        then Greater else NotGE
                    | Equal =>
                        lex_ext rporec (M,N)
                    | NotGE =>
                        if exists (fn M' => ge_ord rporec (M',N)) sons1
                        then Greater else NotGE
  in rporec
  end

fun Group_order x = rpo () x

fun greater pair =
  case Group_order pair of Greater => true | _ => false

(****************** Critical pairs *********************)

(* All (u,sig) such that N/u (&var) unifies with M,
   with principal unifier sig *)

fun super M =
  let fun suprec (N as Term(_,sons)) =  
        let fun collate (pairs,n) son =
                  (pairs @ map (fn (u,sigma) => (n::u,sigma)) (suprec son), n+1)
            val insides  : (int list * (int*term)list)list  =  (*type constraint added (mads)*)
                  fst (it_list collate ([],1) sons)
        in ([], unify(M,N)) ::  insides   handle _ => insides 
        end
    | suprec _ = []
  in suprec 
  end


(********************

Ex :

let (M,_) = <<F(A,B)>> 
and (N,_) = <<H(F(A,x),F(x,y))>> in super M N;;
==> [[1],[2,Term ("B",[])];                      x <- B
     [2],[2,Term ("A",[]); 1,Term ("B",[])]]     x <- A  y <- B
*)

(* All (u,sigma), u&[], such that N/u unifies with M *)
(* super_strict : term -> term -> (num list & subst) list *)

fun super_strict M (Term(_,sons)) =
        let fun collate (pairs,n) son =
          (pairs @ map (fn (u,sigma) => (n::u,sigma)) (super M son), n+1)
        in fst (it_list collate ([],1) sons) end
  | super_strict _ _ = []

(* Critical pairs of L1=R1 with L2=R2 *)
(* critical_pairs : term_pair -> term_pair -> term_pair list *)
fun critical_pairs (L1,R1) (L2,R2) =
  let fun mk_pair (u,sigma) =
     (substitute sigma (replace L2 u R1), substitute sigma R2) in
  map mk_pair (super L1 L2)
  end

(* Strict critical pairs of L1=R1 with L2=R2 *)
(* strict_critical_pairs : term_pair -> term_pair -> term_pair list *)
fun strict_critical_pairs (L1,R1) (L2,R2) =
  let fun mk_pair (u,sigma) =
    (substitute sigma (replace L2 u R1), substitute sigma R2) in
  map mk_pair (super_strict L1 L2)
  end

(* All critical pairs of eq1 with eq2 *)
fun mutual_critical_pairs eq1 eq2 =
  (strict_critical_pairs eq1 eq2) @ (critical_pairs eq2 eq1)

(* Renaming of variables *)

fun rename n (t1,t2) =
  let fun ren_rec (Var k) = Var(k+n)
        | ren_rec (Term(oper,sons)) = Term(oper, map ren_rec sons)
  in (ren_rec t1, ren_rec t2)
  end

(************************ Completion ******************************)

fun deletion_message (k,_) =
  (print_string "Rule ";print_num k; message " deleted")

(* Generate failure message *)
fun non_orientable (M,N) =
  (pretty_term M; print_string " = "; pretty_term N; print_newline())

fun copy_termpairlist [] = []
  | copy_termpairlist ((M,N)::rest) = (copy_term M, copy_term N):: copy_termpairlist rest

fun copy_int_pair(x,y) = (x+0, y+0)
fun copy_int_pair_list l = map copy_int_pair l
fun copy_int (x) = x+0

fun copy_arg(n, rules, failures, p, eps) =
    (n+0, copy_rules rules, copy_termpairlist failures, copy_int_pair p, copy_termpairlist eps)

datatype ('a,'b) intermediate = FOUND of 'a | NOTFOUND of 'b

(* Improved Knuth-Bendix completion procedure *)
(* kb_completion :  num -> rules -> term_pair list -> (num & num) -> term_pair list -> rules *)
fun kb_completion(arg as (n, rules, list, (k,l), eps)) =
  let fun kbrec count n rules =
    let fun normal_form x = mrewrite_all rules x
        fun get_rule k = assoc k rules
        fun process failures =
          let fun processf (k,l) =
            let fun processkl [] =
              if k<l then next_criticals (k+1,l) else
              if l<n then next_criticals (1,l+1) else
               (case failures of
                  [] => FOUND rules (* successful completion *)
                | _  => (message "Non-orientable equations :";
                         app non_orientable failures;
                         raise FailKbComplettion (*failwith "kb_completion"*)  ))
            | processkl ((M,N)::eqs) =
              let val M' = normal_form M
                  val N' = normal_form N
                  fun enter_rule(left,right) =
                    let val new_rule = (n+1, mk_rule left right) in
                     (pretty_rule new_rule;
                      let fun left_reducible (_,(_,(L,_))) = reducible left L
                          val (redl,irredl) = partition left_reducible rules
                      in (app deletion_message redl;
                          let fun right_reduce (m,(_,(L,R))) = 
                              (m,mk_rule L (mrewrite_all (new_rule::rules) R));
                              val irreds = map right_reduce irredl
                              val eqs' = map (fn (_,(_,pair)) => pair) redl
                          in if count>0
                             then (kbrec (count-1) ((n+1)) ((new_rule::irreds)) [] ((k,l))
                                              ((eqs @ eqs' @ failures))
                                       )
                             else NOTFOUND(n+1, new_rule::irreds, [], (k,l), (eqs @ eqs' @ failures))
                          end)
                      end)
                    end
              in if M' = N' then processkl eqs else
                 if greater(M',N') then enter_rule( M', N') 
                 else
                 if greater(N',M') then enter_rule( N', M') 
                 else
                       (process ( ((M', N')::failures)) ( (k,l)) ( eqs))
              end
            in processkl
            end
          and next_criticals (k,l) =
            (let val (v,el) = get_rule l in
               if k = l then
                 processf (k,l) (strict_critical_pairs el (rename v el))
               else
                (let val (_,ek) = get_rule k in 
                    processf (k,l) (mutual_critical_pairs el (rename v ek))
                 end
                 handle FailFind (*rule k deleted*) =>
                   next_criticals (k+1,l))
             end
             handle FailFind (*rule l deleted*) =>
                next_criticals (1,l+1))
          in processf
          end
    in process
    end
(*
    fun kb_outer n rules failures (k,l) other_failures =
      case kbrec 10 (copy_int n) (copy_rules rules) (copy_termpairlist failures) (copy_int_pair (k,l)) 
           (copy_termpairlist other_failures) of
        FOUND rules => copy_rules rules
      | NOTFOUND (n', rules', failures', (k',l'), eqs') =>
          kb_outer n' rules' failures' (k',l') eqs'
*)
     fun kb_outer(arg as (n, rules, failures, (k,l), other_failures)) =
      case kbrec 1 n rules failures (k,l) other_failures of
        FOUND result_rules => if false then arg else (n, result_rules, failures, (k,l), other_failures)
      | NOTFOUND (n', rules', failures', (k',l'), eqs') =>
          kb_outer(copy_arg(copy_arg((n', rules', failures', (k',l'), eqs'))))

  in #2(kb_outer(arg))
  end

fun kb_complete  complete_rules (* the terms in the complete_rules are global *) rules =
    let val n = check_rules complete_rules
        val eqs = map (fn (_,(_,pair)) => pair) rules
        val completed_rules = 
               (* the copying in the line below is to avoid that kb_completion is called with attop modes *)
               kb_completion(n+0, copy_rules complete_rules, [], (n+0,n+0), copy_termpairlist eqs) 
    in (message "Canonical set found :";
        pretty_rules (rev completed_rules);
        ())
    end


fun doit() = kb_complete  [] (* terms in list global *) (Geom_rules())
fun testit _ = ()

(*
  end (* Main *)
*)

in
  doit(); doit(); doit()
end
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*I(C)) *)
(* CHECK-STDOUT-NEXT: 7 : C*(B*I(C)) = B *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 11 : C*(A*(I(C)*A)) = U *)
(* CHECK-STDOUT-NEXT: 12 : C*(B*(I(C)*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 13 : I(U)*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 14 : I(I(v1))*U = v1 *)
(* CHECK-STDOUT-NEXT: 15 : I(v3*v2)*(v3*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 16 : C*(A*(I(C)*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 17 : I(C)*U = C *)
(* CHECK-STDOUT-NEXT: 18 : C*(A*(I(C)*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 19 : I(C)*B = B*I(C) *)
(* CHECK-STDOUT-NEXT: 20 : I(I(v2))*v1 = v2*v1 *)
(* CHECK-STDOUT-NEXT: Rule 14 deleted *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: Rule 17 deleted *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: Rule 19 deleted *)
(* CHECK-STDOUT-NEXT: Rule 18 deleted *)
(* CHECK-STDOUT-NEXT: Rule 16 deleted *)
(* CHECK-STDOUT-NEXT: Rule 12 deleted *)
(* CHECK-STDOUT-NEXT: Rule 11 deleted *)
(* CHECK-STDOUT-NEXT: Rule 7 deleted *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 24 : C*(A*(C*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 25 : C*(A*(C*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 26 : C*(B*(C*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 27 : C*(A*(C*A)) = U *)
(* CHECK-STDOUT-NEXT: 28 : C*(B*C) = B *)
(* CHECK-STDOUT-NEXT: 29 : C*(A*(C*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 30 : I(I(v2*v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 31 : I(v2*I(v1))*v2 = v1 *)
(* CHECK-STDOUT-NEXT: 32 : I(v4*(v3*v2))*(v4*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 33 : I(v1*A)*(v1*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 34 : I(v1*C)*v1 = C *)
(* CHECK-STDOUT-NEXT: 35 : I(v3*I(v2))*(v3*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 36 : I(v2*A)*(v2*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 37 : I(v2*C)*(v2*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 39 : I(C*(A*C))*v1 = A*v1 *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: Rule 13 deleted *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: Rule 20 deleted *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 29 deleted *)
(* CHECK-STDOUT-NEXT: Rule 28 deleted *)
(* CHECK-STDOUT-NEXT: Rule 26 deleted *)
(* CHECK-STDOUT-NEXT: Rule 25 deleted *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: Rule 24 deleted *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: Rule 27 deleted *)
(* CHECK-STDOUT-NEXT: 46 : v2*(I(v1*v2)*v1) = U *)
(* CHECK-STDOUT-NEXT: 47 : I(I(v3*(v2*v1))*(v3*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 48 : I(I(B*A)*A) = B *)
(* CHECK-STDOUT-NEXT: 49 : v3*(I(v2*v3)*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 50 : I(I(v2)*I(v1)) = v1*v2 *)
(* CHECK-STDOUT-NEXT: 51 : I(I(B*(A*v1))*A) = B*v1 *)
(* CHECK-STDOUT-NEXT: 52 : I(I(v1)*C) = C*v1 *)
(* CHECK-STDOUT-NEXT: 53 : I(v2*I(v1*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 54 : I(v3*(v2*I(v1)))*(v3*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 55 : I(v1*(C*(A*C)))*v1 = A *)
(* CHECK-STDOUT-NEXT: 56 : v2*I(I(v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 57 : I(v3*(I(v2*v1)*v2))*v3 = v1 *)
(* CHECK-STDOUT-NEXT: 58 : I(v5*(v4*(v3*v2)))*(v5*(v4*(v3*(v2*v1)))) = v1 *)
(* CHECK-STDOUT-NEXT: 59 : I(v2*(v1*A))*(v2*(v1*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 60 : I(v2*(v1*C))*(v2*v1) = C *)
(* CHECK-STDOUT-NEXT: 61 : I(v4*(v3*I(v2)))*(v4*(v3*v1)) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 62 : I(v3*(v2*A))*(v3*(v2*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 63 : I(v3*(v2*C))*(v3*(v2*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 64 : I(v4*(I(v3*v2)*v3))*(v4*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 65 : v4*(I(v3*(v2*v4))*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 66 : I(I(B)*A)*A = B *)
(* CHECK-STDOUT-NEXT: 67 : I(A*A)*(B*(A*A)) = B *)
(* CHECK-STDOUT-NEXT: 68 : v1*(I(A*v1)*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 69 : I(I(v1*A)*(v1*B))*B = A *)
(* CHECK-STDOUT-NEXT: 70 : v1*I(C*v1) = C *)
(* CHECK-STDOUT-NEXT: 71 : I(A*I(v1))*(B*A) = v1*B *)
(* CHECK-STDOUT-NEXT: 72 : I(C*I(v1)) = v1*C *)
(* CHECK-STDOUT-NEXT: 73 : I(v2*(C*(A*C)))*(v2*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 74 : I(A*I(v2))*(B*(A*v1)) = v2*(B*v1) *)
(* CHECK-STDOUT-NEXT: 75 : v3*(I(I(v2)*v3)*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 76 : I(I(B*I(v1))*A)*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 77 : I(v1*A)*(v1*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 78 : I(I(B)*A)*(A*v1) = B*v1 *)
(* CHECK-STDOUT-NEXT: 79 : I(A*A)*(B*(A*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 80 : I(v2*A)*(v2*(B*(B*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 81 : v2*(I(A*v2)*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 82 : I(I(v2*A)*(v2*B))*(B*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 83 : I(I(B*I(v2))*A)*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 84 : I(A*C)*(B*A) = B*C *)
(* CHECK-STDOUT-NEXT: 85 : I(A*C)*(B*(A*v1)) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 86 : v2*(I(C*v2)*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 87 : I(I(B*C)*A)*(C*A) = B *)
(* CHECK-STDOUT-NEXT: 88 : I(I(B*C)*A)*(C*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 89 : v2*(v1*I(v2*v1)) = U *)
(* CHECK-STDOUT-NEXT: 90 : B*(A*I(B)) = A *)
(* CHECK-STDOUT-NEXT: 91 : I(v2*v1)*v2 = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 64 deleted *)
(* CHECK-STDOUT-NEXT: Rule 57 deleted *)
(* CHECK-STDOUT-NEXT: Rule 55 deleted *)
(* CHECK-STDOUT-NEXT: Rule 46 deleted *)
(* CHECK-STDOUT-NEXT: Rule 34 deleted *)
(* CHECK-STDOUT-NEXT: Rule 31 deleted *)
(* CHECK-STDOUT-NEXT: Rule 30 deleted *)
(* CHECK-STDOUT-NEXT: 92 : I(C*(A*C)) = A *)
(* CHECK-STDOUT-NEXT: Rule 39 deleted *)
(* CHECK-STDOUT-NEXT: 93 : I(v3*(v2*v1))*(v3*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 60 deleted *)
(* CHECK-STDOUT-NEXT: Rule 54 deleted *)
(* CHECK-STDOUT-NEXT: Rule 47 deleted *)
(* CHECK-STDOUT-NEXT: 94 : I(v2*I(v1)) = v1*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 83 deleted *)
(* CHECK-STDOUT-NEXT: Rule 76 deleted *)
(* CHECK-STDOUT-NEXT: Rule 74 deleted *)
(* CHECK-STDOUT-NEXT: Rule 72 deleted *)
(* CHECK-STDOUT-NEXT: Rule 71 deleted *)
(* CHECK-STDOUT-NEXT: Rule 53 deleted *)
(* CHECK-STDOUT-NEXT: Rule 50 deleted *)
(* CHECK-STDOUT-NEXT: Rule 35 deleted *)
(* CHECK-STDOUT-NEXT: 95 : I(v2*(I(B)*A))*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 96 : I(v1*(I(B)*A))*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 97 : I(v1*A)*(v1*B) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 82 deleted *)
(* CHECK-STDOUT-NEXT: Rule 69 deleted *)
(* CHECK-STDOUT-NEXT: 98 : I(v1*C) = C*I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 88 deleted *)
(* CHECK-STDOUT-NEXT: Rule 87 deleted *)
(* CHECK-STDOUT-NEXT: Rule 85 deleted *)
(* CHECK-STDOUT-NEXT: Rule 84 deleted *)
(* CHECK-STDOUT-NEXT: Rule 52 deleted *)
(* CHECK-STDOUT-NEXT: Rule 37 deleted *)
(* CHECK-STDOUT-NEXT: 99 : v3*(v2*(I(v3*v2)*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 100 : B*(A*(I(B)*v1)) = A*v1 *)
(* CHECK-STDOUT-NEXT: 101 : I(v3*v2)*(v3*v1) = I(v2)*v1 *)
(* CHECK-STDOUT-NEXT: Rule 97 deleted *)
(* CHECK-STDOUT-NEXT: Rule 96 deleted *)
(* CHECK-STDOUT-NEXT: Rule 95 deleted *)
(* CHECK-STDOUT-NEXT: Rule 93 deleted *)
(* CHECK-STDOUT-NEXT: Rule 80 deleted *)
(* CHECK-STDOUT-NEXT: Rule 77 deleted *)
(* CHECK-STDOUT-NEXT: Rule 73 deleted *)
(* CHECK-STDOUT-NEXT: Rule 65 deleted *)
(* CHECK-STDOUT-NEXT: Rule 63 deleted *)
(* CHECK-STDOUT-NEXT: Rule 62 deleted *)
(* CHECK-STDOUT-NEXT: Rule 61 deleted *)
(* CHECK-STDOUT-NEXT: Rule 59 deleted *)
(* CHECK-STDOUT-NEXT: Rule 58 deleted *)
(* CHECK-STDOUT-NEXT: Rule 49 deleted *)
(* CHECK-STDOUT-NEXT: Rule 36 deleted *)
(* CHECK-STDOUT-NEXT: Rule 33 deleted *)
(* CHECK-STDOUT-NEXT: Rule 32 deleted *)
(* CHECK-STDOUT-NEXT: Rule 15 deleted *)
(* CHECK-STDOUT-NEXT: 102 : B*(C*I(B)) = C *)
(* CHECK-STDOUT-NEXT: 103 : B*(C*(I(B)*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 104 : B*(I(B*A)*A) = U *)
(* CHECK-STDOUT-NEXT: 105 : B*(I(B*A)*(A*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 106 : I(B*A)*A = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 104 deleted *)
(* CHECK-STDOUT-NEXT: Rule 48 deleted *)
(* CHECK-STDOUT-NEXT: 107 : B*(v1*(I(B*(A*v1))*A)) = U *)
(* CHECK-STDOUT-NEXT: 108 : I(I(B*(B*A))*A) = B*B *)
(* CHECK-STDOUT-NEXT: 109 : B*(v2*(I(B*(A*v2))*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 110 : I(I(B*(B*(A*v1)))*A) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 111 : I(I(B)*A) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 78 deleted *)
(* CHECK-STDOUT-NEXT: Rule 66 deleted *)
(* CHECK-STDOUT-NEXT: 112 : I(I(B*v1)*A) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 110 deleted *)
(* CHECK-STDOUT-NEXT: Rule 108 deleted *)
(* CHECK-STDOUT-NEXT: Rule 51 deleted *)
(* CHECK-STDOUT-NEXT: 113 : v3*(v2*I(I(v1)*(v3*v2))) = v1 *)
(* CHECK-STDOUT-NEXT: 114 : v1*I(C*(A*(C*v1))) = A *)
(* CHECK-STDOUT-NEXT: 115 : I(I(v2)*v1) = I(v1)*v2 *)
(* CHECK-STDOUT-NEXT: Rule 113 deleted *)
(* CHECK-STDOUT-NEXT: Rule 112 deleted *)
(* CHECK-STDOUT-NEXT: Rule 111 deleted *)
(* CHECK-STDOUT-NEXT: Rule 75 deleted *)
(* CHECK-STDOUT-NEXT: Rule 56 deleted *)
(* CHECK-STDOUT-NEXT: 116 : v2*(v1*(I(A*(v2*v1))*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 117 : I(A*v1)*(B*A) = I(v1)*B *)
(* CHECK-STDOUT-NEXT: Rule 116 deleted *)
(* CHECK-STDOUT-NEXT: Rule 68 deleted *)
(* CHECK-STDOUT-NEXT: 118 : v2*(v1*I(C*(v2*v1))) = C *)
(* CHECK-STDOUT-NEXT: 119 : I(C*v1) = I(v1)*C *)
(* CHECK-STDOUT-NEXT: Rule 118 deleted *)
(* CHECK-STDOUT-NEXT: Rule 114 deleted *)
(* CHECK-STDOUT-NEXT: Rule 92 deleted *)
(* CHECK-STDOUT-NEXT: Rule 86 deleted *)
(* CHECK-STDOUT-NEXT: Rule 70 deleted *)
(* CHECK-STDOUT-NEXT: 120 : v1*(I(A*(C*v1))*C) = A *)
(* CHECK-STDOUT-NEXT: 121 : I(A*A)*(B*(B*(A*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 122 : I(A*A)*(B*(B*(A*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 123 : I(A*A)*(B*(A*v1)) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 79 deleted *)
(* CHECK-STDOUT-NEXT: Rule 67 deleted *)
(* CHECK-STDOUT-NEXT: 124 : v3*(v2*(I(A*(v3*v2))*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 125 : v1*(I(A*v1)*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 126 : I(A*v2)*(B*(A*v1)) = I(v2)*(B*v1) *)
(* CHECK-STDOUT-NEXT: Rule 124 deleted *)
(* CHECK-STDOUT-NEXT: Rule 123 deleted *)
(* CHECK-STDOUT-NEXT: Rule 81 deleted *)
(* CHECK-STDOUT-NEXT: 127 : v3*(v2*(v1*I(v3*(v2*v1)))) = U *)
(* CHECK-STDOUT-NEXT: 128 : v2*I(v1*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 89 deleted *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: Rule 90 deleted *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 128 deleted *)
(* CHECK-STDOUT-NEXT: Rule 127 deleted *)
(* CHECK-STDOUT-NEXT: Rule 126 deleted *)
(* CHECK-STDOUT-NEXT: Rule 125 deleted *)
(* CHECK-STDOUT-NEXT: Rule 122 deleted *)
(* CHECK-STDOUT-NEXT: Rule 121 deleted *)
(* CHECK-STDOUT-NEXT: Rule 120 deleted *)
(* CHECK-STDOUT-NEXT: Rule 119 deleted *)
(* CHECK-STDOUT-NEXT: Rule 117 deleted *)
(* CHECK-STDOUT-NEXT: Rule 115 deleted *)
(* CHECK-STDOUT-NEXT: Rule 109 deleted *)
(* CHECK-STDOUT-NEXT: Rule 107 deleted *)
(* CHECK-STDOUT-NEXT: Rule 106 deleted *)
(* CHECK-STDOUT-NEXT: Rule 105 deleted *)
(* CHECK-STDOUT-NEXT: Rule 101 deleted *)
(* CHECK-STDOUT-NEXT: Rule 99 deleted *)
(* CHECK-STDOUT-NEXT: Rule 98 deleted *)
(* CHECK-STDOUT-NEXT: Rule 94 deleted *)
(* CHECK-STDOUT-NEXT: Rule 91 deleted *)
(* CHECK-STDOUT-NEXT: 131 : B*(C*(A*(C*(I(B)*(A*v1))))) = v1 *)
(* CHECK-STDOUT-NEXT: 132 : B*(C*(A*(C*(I(B)*A)))) = U *)
(* CHECK-STDOUT-NEXT: 133 : C*(A*(C*(I(B)*A))) = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 132 deleted *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: Rule 100 deleted *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: Rule 102 deleted *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 133 deleted *)
(* CHECK-STDOUT-NEXT: Rule 131 deleted *)
(* CHECK-STDOUT-NEXT: Rule 103 deleted *)
(* CHECK-STDOUT-NEXT: Canonical set found : *)
(* CHECK-STDOUT-NEXT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*C) *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
(* CHECK-STDOUT-NEXT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*I(C)) *)
(* CHECK-STDOUT-NEXT: 7 : C*(B*I(C)) = B *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 11 : C*(A*(I(C)*A)) = U *)
(* CHECK-STDOUT-NEXT: 12 : C*(B*(I(C)*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 13 : I(U)*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 14 : I(I(v1))*U = v1 *)
(* CHECK-STDOUT-NEXT: 15 : I(v3*v2)*(v3*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 16 : C*(A*(I(C)*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 17 : I(C)*U = C *)
(* CHECK-STDOUT-NEXT: 18 : C*(A*(I(C)*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 19 : I(C)*B = B*I(C) *)
(* CHECK-STDOUT-NEXT: 20 : I(I(v2))*v1 = v2*v1 *)
(* CHECK-STDOUT-NEXT: Rule 14 deleted *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: Rule 17 deleted *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: Rule 19 deleted *)
(* CHECK-STDOUT-NEXT: Rule 18 deleted *)
(* CHECK-STDOUT-NEXT: Rule 16 deleted *)
(* CHECK-STDOUT-NEXT: Rule 12 deleted *)
(* CHECK-STDOUT-NEXT: Rule 11 deleted *)
(* CHECK-STDOUT-NEXT: Rule 7 deleted *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 24 : C*(A*(C*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 25 : C*(A*(C*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 26 : C*(B*(C*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 27 : C*(A*(C*A)) = U *)
(* CHECK-STDOUT-NEXT: 28 : C*(B*C) = B *)
(* CHECK-STDOUT-NEXT: 29 : C*(A*(C*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 30 : I(I(v2*v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 31 : I(v2*I(v1))*v2 = v1 *)
(* CHECK-STDOUT-NEXT: 32 : I(v4*(v3*v2))*(v4*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 33 : I(v1*A)*(v1*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 34 : I(v1*C)*v1 = C *)
(* CHECK-STDOUT-NEXT: 35 : I(v3*I(v2))*(v3*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 36 : I(v2*A)*(v2*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 37 : I(v2*C)*(v2*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 39 : I(C*(A*C))*v1 = A*v1 *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: Rule 13 deleted *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: Rule 20 deleted *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 29 deleted *)
(* CHECK-STDOUT-NEXT: Rule 28 deleted *)
(* CHECK-STDOUT-NEXT: Rule 26 deleted *)
(* CHECK-STDOUT-NEXT: Rule 25 deleted *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: Rule 24 deleted *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: Rule 27 deleted *)
(* CHECK-STDOUT-NEXT: 46 : v2*(I(v1*v2)*v1) = U *)
(* CHECK-STDOUT-NEXT: 47 : I(I(v3*(v2*v1))*(v3*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 48 : I(I(B*A)*A) = B *)
(* CHECK-STDOUT-NEXT: 49 : v3*(I(v2*v3)*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 50 : I(I(v2)*I(v1)) = v1*v2 *)
(* CHECK-STDOUT-NEXT: 51 : I(I(B*(A*v1))*A) = B*v1 *)
(* CHECK-STDOUT-NEXT: 52 : I(I(v1)*C) = C*v1 *)
(* CHECK-STDOUT-NEXT: 53 : I(v2*I(v1*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 54 : I(v3*(v2*I(v1)))*(v3*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 55 : I(v1*(C*(A*C)))*v1 = A *)
(* CHECK-STDOUT-NEXT: 56 : v2*I(I(v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 57 : I(v3*(I(v2*v1)*v2))*v3 = v1 *)
(* CHECK-STDOUT-NEXT: 58 : I(v5*(v4*(v3*v2)))*(v5*(v4*(v3*(v2*v1)))) = v1 *)
(* CHECK-STDOUT-NEXT: 59 : I(v2*(v1*A))*(v2*(v1*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 60 : I(v2*(v1*C))*(v2*v1) = C *)
(* CHECK-STDOUT-NEXT: 61 : I(v4*(v3*I(v2)))*(v4*(v3*v1)) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 62 : I(v3*(v2*A))*(v3*(v2*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 63 : I(v3*(v2*C))*(v3*(v2*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 64 : I(v4*(I(v3*v2)*v3))*(v4*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 65 : v4*(I(v3*(v2*v4))*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 66 : I(I(B)*A)*A = B *)
(* CHECK-STDOUT-NEXT: 67 : I(A*A)*(B*(A*A)) = B *)
(* CHECK-STDOUT-NEXT: 68 : v1*(I(A*v1)*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 69 : I(I(v1*A)*(v1*B))*B = A *)
(* CHECK-STDOUT-NEXT: 70 : v1*I(C*v1) = C *)
(* CHECK-STDOUT-NEXT: 71 : I(A*I(v1))*(B*A) = v1*B *)
(* CHECK-STDOUT-NEXT: 72 : I(C*I(v1)) = v1*C *)
(* CHECK-STDOUT-NEXT: 73 : I(v2*(C*(A*C)))*(v2*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 74 : I(A*I(v2))*(B*(A*v1)) = v2*(B*v1) *)
(* CHECK-STDOUT-NEXT: 75 : v3*(I(I(v2)*v3)*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 76 : I(I(B*I(v1))*A)*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 77 : I(v1*A)*(v1*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 78 : I(I(B)*A)*(A*v1) = B*v1 *)
(* CHECK-STDOUT-NEXT: 79 : I(A*A)*(B*(A*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 80 : I(v2*A)*(v2*(B*(B*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 81 : v2*(I(A*v2)*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 82 : I(I(v2*A)*(v2*B))*(B*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 83 : I(I(B*I(v2))*A)*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 84 : I(A*C)*(B*A) = B*C *)
(* CHECK-STDOUT-NEXT: 85 : I(A*C)*(B*(A*v1)) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 86 : v2*(I(C*v2)*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 87 : I(I(B*C)*A)*(C*A) = B *)
(* CHECK-STDOUT-NEXT: 88 : I(I(B*C)*A)*(C*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 89 : v2*(v1*I(v2*v1)) = U *)
(* CHECK-STDOUT-NEXT: 90 : B*(A*I(B)) = A *)
(* CHECK-STDOUT-NEXT: 91 : I(v2*v1)*v2 = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 64 deleted *)
(* CHECK-STDOUT-NEXT: Rule 57 deleted *)
(* CHECK-STDOUT-NEXT: Rule 55 deleted *)
(* CHECK-STDOUT-NEXT: Rule 46 deleted *)
(* CHECK-STDOUT-NEXT: Rule 34 deleted *)
(* CHECK-STDOUT-NEXT: Rule 31 deleted *)
(* CHECK-STDOUT-NEXT: Rule 30 deleted *)
(* CHECK-STDOUT-NEXT: 92 : I(C*(A*C)) = A *)
(* CHECK-STDOUT-NEXT: Rule 39 deleted *)
(* CHECK-STDOUT-NEXT: 93 : I(v3*(v2*v1))*(v3*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 60 deleted *)
(* CHECK-STDOUT-NEXT: Rule 54 deleted *)
(* CHECK-STDOUT-NEXT: Rule 47 deleted *)
(* CHECK-STDOUT-NEXT: 94 : I(v2*I(v1)) = v1*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 83 deleted *)
(* CHECK-STDOUT-NEXT: Rule 76 deleted *)
(* CHECK-STDOUT-NEXT: Rule 74 deleted *)
(* CHECK-STDOUT-NEXT: Rule 72 deleted *)
(* CHECK-STDOUT-NEXT: Rule 71 deleted *)
(* CHECK-STDOUT-NEXT: Rule 53 deleted *)
(* CHECK-STDOUT-NEXT: Rule 50 deleted *)
(* CHECK-STDOUT-NEXT: Rule 35 deleted *)
(* CHECK-STDOUT-NEXT: 95 : I(v2*(I(B)*A))*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 96 : I(v1*(I(B)*A))*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 97 : I(v1*A)*(v1*B) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 82 deleted *)
(* CHECK-STDOUT-NEXT: Rule 69 deleted *)
(* CHECK-STDOUT-NEXT: 98 : I(v1*C) = C*I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 88 deleted *)
(* CHECK-STDOUT-NEXT: Rule 87 deleted *)
(* CHECK-STDOUT-NEXT: Rule 85 deleted *)
(* CHECK-STDOUT-NEXT: Rule 84 deleted *)
(* CHECK-STDOUT-NEXT: Rule 52 deleted *)
(* CHECK-STDOUT-NEXT: Rule 37 deleted *)
(* CHECK-STDOUT-NEXT: 99 : v3*(v2*(I(v3*v2)*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 100 : B*(A*(I(B)*v1)) = A*v1 *)
(* CHECK-STDOUT-NEXT: 101 : I(v3*v2)*(v3*v1) = I(v2)*v1 *)
(* CHECK-STDOUT-NEXT: Rule 97 deleted *)
(* CHECK-STDOUT-NEXT: Rule 96 deleted *)
(* CHECK-STDOUT-NEXT: Rule 95 deleted *)
(* CHECK-STDOUT-NEXT: Rule 93 deleted *)
(* CHECK-STDOUT-NEXT: Rule 80 deleted *)
(* CHECK-STDOUT-NEXT: Rule 77 deleted *)
(* CHECK-STDOUT-NEXT: Rule 73 deleted *)
(* CHECK-STDOUT-NEXT: Rule 65 deleted *)
(* CHECK-STDOUT-NEXT: Rule 63 deleted *)
(* CHECK-STDOUT-NEXT: Rule 62 deleted *)
(* CHECK-STDOUT-NEXT: Rule 61 deleted *)
(* CHECK-STDOUT-NEXT: Rule 59 deleted *)
(* CHECK-STDOUT-NEXT: Rule 58 deleted *)
(* CHECK-STDOUT-NEXT: Rule 49 deleted *)
(* CHECK-STDOUT-NEXT: Rule 36 deleted *)
(* CHECK-STDOUT-NEXT: Rule 33 deleted *)
(* CHECK-STDOUT-NEXT: Rule 32 deleted *)
(* CHECK-STDOUT-NEXT: Rule 15 deleted *)
(* CHECK-STDOUT-NEXT: 102 : B*(C*I(B)) = C *)
(* CHECK-STDOUT-NEXT: 103 : B*(C*(I(B)*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 104 : B*(I(B*A)*A) = U *)
(* CHECK-STDOUT-NEXT: 105 : B*(I(B*A)*(A*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 106 : I(B*A)*A = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 104 deleted *)
(* CHECK-STDOUT-NEXT: Rule 48 deleted *)
(* CHECK-STDOUT-NEXT: 107 : B*(v1*(I(B*(A*v1))*A)) = U *)
(* CHECK-STDOUT-NEXT: 108 : I(I(B*(B*A))*A) = B*B *)
(* CHECK-STDOUT-NEXT: 109 : B*(v2*(I(B*(A*v2))*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 110 : I(I(B*(B*(A*v1)))*A) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 111 : I(I(B)*A) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 78 deleted *)
(* CHECK-STDOUT-NEXT: Rule 66 deleted *)
(* CHECK-STDOUT-NEXT: 112 : I(I(B*v1)*A) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 110 deleted *)
(* CHECK-STDOUT-NEXT: Rule 108 deleted *)
(* CHECK-STDOUT-NEXT: Rule 51 deleted *)
(* CHECK-STDOUT-NEXT: 113 : v3*(v2*I(I(v1)*(v3*v2))) = v1 *)
(* CHECK-STDOUT-NEXT: 114 : v1*I(C*(A*(C*v1))) = A *)
(* CHECK-STDOUT-NEXT: 115 : I(I(v2)*v1) = I(v1)*v2 *)
(* CHECK-STDOUT-NEXT: Rule 113 deleted *)
(* CHECK-STDOUT-NEXT: Rule 112 deleted *)
(* CHECK-STDOUT-NEXT: Rule 111 deleted *)
(* CHECK-STDOUT-NEXT: Rule 75 deleted *)
(* CHECK-STDOUT-NEXT: Rule 56 deleted *)
(* CHECK-STDOUT-NEXT: 116 : v2*(v1*(I(A*(v2*v1))*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 117 : I(A*v1)*(B*A) = I(v1)*B *)
(* CHECK-STDOUT-NEXT: Rule 116 deleted *)
(* CHECK-STDOUT-NEXT: Rule 68 deleted *)
(* CHECK-STDOUT-NEXT: 118 : v2*(v1*I(C*(v2*v1))) = C *)
(* CHECK-STDOUT-NEXT: 119 : I(C*v1) = I(v1)*C *)
(* CHECK-STDOUT-NEXT: Rule 118 deleted *)
(* CHECK-STDOUT-NEXT: Rule 114 deleted *)
(* CHECK-STDOUT-NEXT: Rule 92 deleted *)
(* CHECK-STDOUT-NEXT: Rule 86 deleted *)
(* CHECK-STDOUT-NEXT: Rule 70 deleted *)
(* CHECK-STDOUT-NEXT: 120 : v1*(I(A*(C*v1))*C) = A *)
(* CHECK-STDOUT-NEXT: 121 : I(A*A)*(B*(B*(A*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 122 : I(A*A)*(B*(B*(A*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 123 : I(A*A)*(B*(A*v1)) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 79 deleted *)
(* CHECK-STDOUT-NEXT: Rule 67 deleted *)
(* CHECK-STDOUT-NEXT: 124 : v3*(v2*(I(A*(v3*v2))*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 125 : v1*(I(A*v1)*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 126 : I(A*v2)*(B*(A*v1)) = I(v2)*(B*v1) *)
(* CHECK-STDOUT-NEXT: Rule 124 deleted *)
(* CHECK-STDOUT-NEXT: Rule 123 deleted *)
(* CHECK-STDOUT-NEXT: Rule 81 deleted *)
(* CHECK-STDOUT-NEXT: 127 : v3*(v2*(v1*I(v3*(v2*v1)))) = U *)
(* CHECK-STDOUT-NEXT: 128 : v2*I(v1*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 89 deleted *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: Rule 90 deleted *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 128 deleted *)
(* CHECK-STDOUT-NEXT: Rule 127 deleted *)
(* CHECK-STDOUT-NEXT: Rule 126 deleted *)
(* CHECK-STDOUT-NEXT: Rule 125 deleted *)
(* CHECK-STDOUT-NEXT: Rule 122 deleted *)
(* CHECK-STDOUT-NEXT: Rule 121 deleted *)
(* CHECK-STDOUT-NEXT: Rule 120 deleted *)
(* CHECK-STDOUT-NEXT: Rule 119 deleted *)
(* CHECK-STDOUT-NEXT: Rule 117 deleted *)
(* CHECK-STDOUT-NEXT: Rule 115 deleted *)
(* CHECK-STDOUT-NEXT: Rule 109 deleted *)
(* CHECK-STDOUT-NEXT: Rule 107 deleted *)
(* CHECK-STDOUT-NEXT: Rule 106 deleted *)
(* CHECK-STDOUT-NEXT: Rule 105 deleted *)
(* CHECK-STDOUT-NEXT: Rule 101 deleted *)
(* CHECK-STDOUT-NEXT: Rule 99 deleted *)
(* CHECK-STDOUT-NEXT: Rule 98 deleted *)
(* CHECK-STDOUT-NEXT: Rule 94 deleted *)
(* CHECK-STDOUT-NEXT: Rule 91 deleted *)
(* CHECK-STDOUT-NEXT: 131 : B*(C*(A*(C*(I(B)*(A*v1))))) = v1 *)
(* CHECK-STDOUT-NEXT: 132 : B*(C*(A*(C*(I(B)*A)))) = U *)
(* CHECK-STDOUT-NEXT: 133 : C*(A*(C*(I(B)*A))) = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 132 deleted *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: Rule 100 deleted *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: Rule 102 deleted *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 133 deleted *)
(* CHECK-STDOUT-NEXT: Rule 131 deleted *)
(* CHECK-STDOUT-NEXT: Rule 103 deleted *)
(* CHECK-STDOUT-NEXT: Canonical set found : *)
(* CHECK-STDOUT-NEXT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*C) *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
(* CHECK-STDOUT-NEXT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*I(C)) *)
(* CHECK-STDOUT-NEXT: 7 : C*(B*I(C)) = B *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 11 : C*(A*(I(C)*A)) = U *)
(* CHECK-STDOUT-NEXT: 12 : C*(B*(I(C)*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 13 : I(U)*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 14 : I(I(v1))*U = v1 *)
(* CHECK-STDOUT-NEXT: 15 : I(v3*v2)*(v3*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 16 : C*(A*(I(C)*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 17 : I(C)*U = C *)
(* CHECK-STDOUT-NEXT: 18 : C*(A*(I(C)*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 19 : I(C)*B = B*I(C) *)
(* CHECK-STDOUT-NEXT: 20 : I(I(v2))*v1 = v2*v1 *)
(* CHECK-STDOUT-NEXT: Rule 14 deleted *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: Rule 17 deleted *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: Rule 19 deleted *)
(* CHECK-STDOUT-NEXT: Rule 18 deleted *)
(* CHECK-STDOUT-NEXT: Rule 16 deleted *)
(* CHECK-STDOUT-NEXT: Rule 12 deleted *)
(* CHECK-STDOUT-NEXT: Rule 11 deleted *)
(* CHECK-STDOUT-NEXT: Rule 7 deleted *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 24 : C*(A*(C*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 25 : C*(A*(C*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 26 : C*(B*(C*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 27 : C*(A*(C*A)) = U *)
(* CHECK-STDOUT-NEXT: 28 : C*(B*C) = B *)
(* CHECK-STDOUT-NEXT: 29 : C*(A*(C*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 30 : I(I(v2*v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 31 : I(v2*I(v1))*v2 = v1 *)
(* CHECK-STDOUT-NEXT: 32 : I(v4*(v3*v2))*(v4*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 33 : I(v1*A)*(v1*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 34 : I(v1*C)*v1 = C *)
(* CHECK-STDOUT-NEXT: 35 : I(v3*I(v2))*(v3*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 36 : I(v2*A)*(v2*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 37 : I(v2*C)*(v2*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 39 : I(C*(A*C))*v1 = A*v1 *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: Rule 13 deleted *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: Rule 20 deleted *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 29 deleted *)
(* CHECK-STDOUT-NEXT: Rule 28 deleted *)
(* CHECK-STDOUT-NEXT: Rule 26 deleted *)
(* CHECK-STDOUT-NEXT: Rule 25 deleted *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: Rule 24 deleted *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: Rule 27 deleted *)
(* CHECK-STDOUT-NEXT: 46 : v2*(I(v1*v2)*v1) = U *)
(* CHECK-STDOUT-NEXT: 47 : I(I(v3*(v2*v1))*(v3*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 48 : I(I(B*A)*A) = B *)
(* CHECK-STDOUT-NEXT: 49 : v3*(I(v2*v3)*(v2*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 50 : I(I(v2)*I(v1)) = v1*v2 *)
(* CHECK-STDOUT-NEXT: 51 : I(I(B*(A*v1))*A) = B*v1 *)
(* CHECK-STDOUT-NEXT: 52 : I(I(v1)*C) = C*v1 *)
(* CHECK-STDOUT-NEXT: 53 : I(v2*I(v1*v2)) = v1 *)
(* CHECK-STDOUT-NEXT: 54 : I(v3*(v2*I(v1)))*(v3*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 55 : I(v1*(C*(A*C)))*v1 = A *)
(* CHECK-STDOUT-NEXT: 56 : v2*I(I(v1)*v2) = v1 *)
(* CHECK-STDOUT-NEXT: 57 : I(v3*(I(v2*v1)*v2))*v3 = v1 *)
(* CHECK-STDOUT-NEXT: 58 : I(v5*(v4*(v3*v2)))*(v5*(v4*(v3*(v2*v1)))) = v1 *)
(* CHECK-STDOUT-NEXT: 59 : I(v2*(v1*A))*(v2*(v1*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 60 : I(v2*(v1*C))*(v2*v1) = C *)
(* CHECK-STDOUT-NEXT: 61 : I(v4*(v3*I(v2)))*(v4*(v3*v1)) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 62 : I(v3*(v2*A))*(v3*(v2*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 63 : I(v3*(v2*C))*(v3*(v2*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 64 : I(v4*(I(v3*v2)*v3))*(v4*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 65 : v4*(I(v3*(v2*v4))*(v3*(v2*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 66 : I(I(B)*A)*A = B *)
(* CHECK-STDOUT-NEXT: 67 : I(A*A)*(B*(A*A)) = B *)
(* CHECK-STDOUT-NEXT: 68 : v1*(I(A*v1)*(B*A)) = B *)
(* CHECK-STDOUT-NEXT: 69 : I(I(v1*A)*(v1*B))*B = A *)
(* CHECK-STDOUT-NEXT: 70 : v1*I(C*v1) = C *)
(* CHECK-STDOUT-NEXT: 71 : I(A*I(v1))*(B*A) = v1*B *)
(* CHECK-STDOUT-NEXT: 72 : I(C*I(v1)) = v1*C *)
(* CHECK-STDOUT-NEXT: 73 : I(v2*(C*(A*C)))*(v2*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 74 : I(A*I(v2))*(B*(A*v1)) = v2*(B*v1) *)
(* CHECK-STDOUT-NEXT: 75 : v3*(I(I(v2)*v3)*v1) = v2*v1 *)
(* CHECK-STDOUT-NEXT: 76 : I(I(B*I(v1))*A)*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 77 : I(v1*A)*(v1*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 78 : I(I(B)*A)*(A*v1) = B*v1 *)
(* CHECK-STDOUT-NEXT: 79 : I(A*A)*(B*(A*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 80 : I(v2*A)*(v2*(B*(B*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 81 : v2*(I(A*v2)*(B*(A*v1))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 82 : I(I(v2*A)*(v2*B))*(B*v1) = A*v1 *)
(* CHECK-STDOUT-NEXT: 83 : I(I(B*I(v2))*A)*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 84 : I(A*C)*(B*A) = B*C *)
(* CHECK-STDOUT-NEXT: 85 : I(A*C)*(B*(A*v1)) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 86 : v2*(I(C*v2)*v1) = C*v1 *)
(* CHECK-STDOUT-NEXT: 87 : I(I(B*C)*A)*(C*A) = B *)
(* CHECK-STDOUT-NEXT: 88 : I(I(B*C)*A)*(C*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 89 : v2*(v1*I(v2*v1)) = U *)
(* CHECK-STDOUT-NEXT: 90 : B*(A*I(B)) = A *)
(* CHECK-STDOUT-NEXT: 91 : I(v2*v1)*v2 = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 64 deleted *)
(* CHECK-STDOUT-NEXT: Rule 57 deleted *)
(* CHECK-STDOUT-NEXT: Rule 55 deleted *)
(* CHECK-STDOUT-NEXT: Rule 46 deleted *)
(* CHECK-STDOUT-NEXT: Rule 34 deleted *)
(* CHECK-STDOUT-NEXT: Rule 31 deleted *)
(* CHECK-STDOUT-NEXT: Rule 30 deleted *)
(* CHECK-STDOUT-NEXT: 92 : I(C*(A*C)) = A *)
(* CHECK-STDOUT-NEXT: Rule 39 deleted *)
(* CHECK-STDOUT-NEXT: 93 : I(v3*(v2*v1))*(v3*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 60 deleted *)
(* CHECK-STDOUT-NEXT: Rule 54 deleted *)
(* CHECK-STDOUT-NEXT: Rule 47 deleted *)
(* CHECK-STDOUT-NEXT: 94 : I(v2*I(v1)) = v1*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 83 deleted *)
(* CHECK-STDOUT-NEXT: Rule 76 deleted *)
(* CHECK-STDOUT-NEXT: Rule 74 deleted *)
(* CHECK-STDOUT-NEXT: Rule 72 deleted *)
(* CHECK-STDOUT-NEXT: Rule 71 deleted *)
(* CHECK-STDOUT-NEXT: Rule 53 deleted *)
(* CHECK-STDOUT-NEXT: Rule 50 deleted *)
(* CHECK-STDOUT-NEXT: Rule 35 deleted *)
(* CHECK-STDOUT-NEXT: 95 : I(v2*(I(B)*A))*(v2*(A*v1)) = B*v1 *)
(* CHECK-STDOUT-NEXT: 96 : I(v1*(I(B)*A))*(v1*A) = B *)
(* CHECK-STDOUT-NEXT: 97 : I(v1*A)*(v1*B) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 82 deleted *)
(* CHECK-STDOUT-NEXT: Rule 69 deleted *)
(* CHECK-STDOUT-NEXT: 98 : I(v1*C) = C*I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 88 deleted *)
(* CHECK-STDOUT-NEXT: Rule 87 deleted *)
(* CHECK-STDOUT-NEXT: Rule 85 deleted *)
(* CHECK-STDOUT-NEXT: Rule 84 deleted *)
(* CHECK-STDOUT-NEXT: Rule 52 deleted *)
(* CHECK-STDOUT-NEXT: Rule 37 deleted *)
(* CHECK-STDOUT-NEXT: 99 : v3*(v2*(I(v3*v2)*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 100 : B*(A*(I(B)*v1)) = A*v1 *)
(* CHECK-STDOUT-NEXT: 101 : I(v3*v2)*(v3*v1) = I(v2)*v1 *)
(* CHECK-STDOUT-NEXT: Rule 97 deleted *)
(* CHECK-STDOUT-NEXT: Rule 96 deleted *)
(* CHECK-STDOUT-NEXT: Rule 95 deleted *)
(* CHECK-STDOUT-NEXT: Rule 93 deleted *)
(* CHECK-STDOUT-NEXT: Rule 80 deleted *)
(* CHECK-STDOUT-NEXT: Rule 77 deleted *)
(* CHECK-STDOUT-NEXT: Rule 73 deleted *)
(* CHECK-STDOUT-NEXT: Rule 65 deleted *)
(* CHECK-STDOUT-NEXT: Rule 63 deleted *)
(* CHECK-STDOUT-NEXT: Rule 62 deleted *)
(* CHECK-STDOUT-NEXT: Rule 61 deleted *)
(* CHECK-STDOUT-NEXT: Rule 59 deleted *)
(* CHECK-STDOUT-NEXT: Rule 58 deleted *)
(* CHECK-STDOUT-NEXT: Rule 49 deleted *)
(* CHECK-STDOUT-NEXT: Rule 36 deleted *)
(* CHECK-STDOUT-NEXT: Rule 33 deleted *)
(* CHECK-STDOUT-NEXT: Rule 32 deleted *)
(* CHECK-STDOUT-NEXT: Rule 15 deleted *)
(* CHECK-STDOUT-NEXT: 102 : B*(C*I(B)) = C *)
(* CHECK-STDOUT-NEXT: 103 : B*(C*(I(B)*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 104 : B*(I(B*A)*A) = U *)
(* CHECK-STDOUT-NEXT: 105 : B*(I(B*A)*(A*v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 106 : I(B*A)*A = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 104 deleted *)
(* CHECK-STDOUT-NEXT: Rule 48 deleted *)
(* CHECK-STDOUT-NEXT: 107 : B*(v1*(I(B*(A*v1))*A)) = U *)
(* CHECK-STDOUT-NEXT: 108 : I(I(B*(B*A))*A) = B*B *)
(* CHECK-STDOUT-NEXT: 109 : B*(v2*(I(B*(A*v2))*(A*v1))) = v1 *)
(* CHECK-STDOUT-NEXT: 110 : I(I(B*(B*(A*v1)))*A) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 111 : I(I(B)*A) = B*(C*(A*C)) *)
(* CHECK-STDOUT-NEXT: Rule 78 deleted *)
(* CHECK-STDOUT-NEXT: Rule 66 deleted *)
(* CHECK-STDOUT-NEXT: 112 : I(I(B*v1)*A) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 110 deleted *)
(* CHECK-STDOUT-NEXT: Rule 108 deleted *)
(* CHECK-STDOUT-NEXT: Rule 51 deleted *)
(* CHECK-STDOUT-NEXT: 113 : v3*(v2*I(I(v1)*(v3*v2))) = v1 *)
(* CHECK-STDOUT-NEXT: 114 : v1*I(C*(A*(C*v1))) = A *)
(* CHECK-STDOUT-NEXT: 115 : I(I(v2)*v1) = I(v1)*v2 *)
(* CHECK-STDOUT-NEXT: Rule 113 deleted *)
(* CHECK-STDOUT-NEXT: Rule 112 deleted *)
(* CHECK-STDOUT-NEXT: Rule 111 deleted *)
(* CHECK-STDOUT-NEXT: Rule 75 deleted *)
(* CHECK-STDOUT-NEXT: Rule 56 deleted *)
(* CHECK-STDOUT-NEXT: 116 : v2*(v1*(I(A*(v2*v1))*(B*A))) = B *)
(* CHECK-STDOUT-NEXT: 117 : I(A*v1)*(B*A) = I(v1)*B *)
(* CHECK-STDOUT-NEXT: Rule 116 deleted *)
(* CHECK-STDOUT-NEXT: Rule 68 deleted *)
(* CHECK-STDOUT-NEXT: 118 : v2*(v1*I(C*(v2*v1))) = C *)
(* CHECK-STDOUT-NEXT: 119 : I(C*v1) = I(v1)*C *)
(* CHECK-STDOUT-NEXT: Rule 118 deleted *)
(* CHECK-STDOUT-NEXT: Rule 114 deleted *)
(* CHECK-STDOUT-NEXT: Rule 92 deleted *)
(* CHECK-STDOUT-NEXT: Rule 86 deleted *)
(* CHECK-STDOUT-NEXT: Rule 70 deleted *)
(* CHECK-STDOUT-NEXT: 120 : v1*(I(A*(C*v1))*C) = A *)
(* CHECK-STDOUT-NEXT: 121 : I(A*A)*(B*(B*(A*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 122 : I(A*A)*(B*(B*(A*(A*v1)))) = B*(B*v1) *)
(* CHECK-STDOUT-NEXT: 123 : I(A*A)*(B*(A*v1)) = B*(C*(A*(C*v1))) *)
(* CHECK-STDOUT-NEXT: Rule 79 deleted *)
(* CHECK-STDOUT-NEXT: Rule 67 deleted *)
(* CHECK-STDOUT-NEXT: 124 : v3*(v2*(I(A*(v3*v2))*(B*(A*v1)))) = B*v1 *)
(* CHECK-STDOUT-NEXT: 125 : v1*(I(A*v1)*(B*(B*A))) = B*B *)
(* CHECK-STDOUT-NEXT: 126 : I(A*v2)*(B*(A*v1)) = I(v2)*(B*v1) *)
(* CHECK-STDOUT-NEXT: Rule 124 deleted *)
(* CHECK-STDOUT-NEXT: Rule 123 deleted *)
(* CHECK-STDOUT-NEXT: Rule 81 deleted *)
(* CHECK-STDOUT-NEXT: 127 : v3*(v2*(v1*I(v3*(v2*v1)))) = U *)
(* CHECK-STDOUT-NEXT: 128 : v2*I(v1*v2) = I(v1) *)
(* CHECK-STDOUT-NEXT: Rule 89 deleted *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: Rule 90 deleted *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: Rule 128 deleted *)
(* CHECK-STDOUT-NEXT: Rule 127 deleted *)
(* CHECK-STDOUT-NEXT: Rule 126 deleted *)
(* CHECK-STDOUT-NEXT: Rule 125 deleted *)
(* CHECK-STDOUT-NEXT: Rule 122 deleted *)
(* CHECK-STDOUT-NEXT: Rule 121 deleted *)
(* CHECK-STDOUT-NEXT: Rule 120 deleted *)
(* CHECK-STDOUT-NEXT: Rule 119 deleted *)
(* CHECK-STDOUT-NEXT: Rule 117 deleted *)
(* CHECK-STDOUT-NEXT: Rule 115 deleted *)
(* CHECK-STDOUT-NEXT: Rule 109 deleted *)
(* CHECK-STDOUT-NEXT: Rule 107 deleted *)
(* CHECK-STDOUT-NEXT: Rule 106 deleted *)
(* CHECK-STDOUT-NEXT: Rule 105 deleted *)
(* CHECK-STDOUT-NEXT: Rule 101 deleted *)
(* CHECK-STDOUT-NEXT: Rule 99 deleted *)
(* CHECK-STDOUT-NEXT: Rule 98 deleted *)
(* CHECK-STDOUT-NEXT: Rule 94 deleted *)
(* CHECK-STDOUT-NEXT: Rule 91 deleted *)
(* CHECK-STDOUT-NEXT: 131 : B*(C*(A*(C*(I(B)*(A*v1))))) = v1 *)
(* CHECK-STDOUT-NEXT: 132 : B*(C*(A*(C*(I(B)*A)))) = U *)
(* CHECK-STDOUT-NEXT: 133 : C*(A*(C*(I(B)*A))) = I(B) *)
(* CHECK-STDOUT-NEXT: Rule 132 deleted *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: Rule 100 deleted *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: Rule 102 deleted *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
(* CHECK-STDOUT-NEXT: Rule 133 deleted *)
(* CHECK-STDOUT-NEXT: Rule 131 deleted *)
(* CHECK-STDOUT-NEXT: Rule 103 deleted *)
(* CHECK-STDOUT-NEXT: Canonical set found : *)
(* CHECK-STDOUT-NEXT: 1 : U*v1 = v1 *)
(* CHECK-STDOUT-NEXT: 2 : I(v1)*v1 = U *)
(* CHECK-STDOUT-NEXT: 3 : (v3*v2)*v1 = v3*(v2*v1) *)
(* CHECK-STDOUT-NEXT: 4 : A*B = B*A *)
(* CHECK-STDOUT-NEXT: 5 : C*C = U *)
(* CHECK-STDOUT-NEXT: 6 : I(A) = C*(A*C) *)
(* CHECK-STDOUT-NEXT: 8 : I(v2)*(v2*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 9 : A*(B*v1) = B*(A*v1) *)
(* CHECK-STDOUT-NEXT: 10 : C*(C*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 21 : v1*U = v1 *)
(* CHECK-STDOUT-NEXT: 22 : I(C) = C *)
(* CHECK-STDOUT-NEXT: 23 : C*B = B*C *)
(* CHECK-STDOUT-NEXT: 38 : v1*I(v1) = U *)
(* CHECK-STDOUT-NEXT: 40 : v2*(I(v2)*v1) = v1 *)
(* CHECK-STDOUT-NEXT: 41 : I(U) = U *)
(* CHECK-STDOUT-NEXT: 42 : I(I(v1)) = v1 *)
(* CHECK-STDOUT-NEXT: 43 : C*(B*v1) = B*(C*v1) *)
(* CHECK-STDOUT-NEXT: 44 : A*(C*(A*v1)) = C*v1 *)
(* CHECK-STDOUT-NEXT: 45 : A*(C*A) = C *)
(* CHECK-STDOUT-NEXT: 129 : A*I(B) = I(B)*A *)
(* CHECK-STDOUT-NEXT: 130 : I(v2*v1) = I(v1)*I(v2) *)
(* CHECK-STDOUT-NEXT: 134 : A*(I(B)*v1) = I(B)*(A*v1) *)
(* CHECK-STDOUT-NEXT: 135 : C*I(B) = I(B)*C *)
(* CHECK-STDOUT-NEXT: 136 : C*(I(B)*v1) = I(B)*(C*v1) *)
