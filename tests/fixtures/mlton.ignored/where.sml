(* mlton regression/where.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML rejects this program *)
(* where.sml *)

(* Checks treatment of type realisations. *)

signature S =
sig
    type t
    type s = t
end where type s = int;

(* Due to Martin Elsman, also see SML/NJ bug 1330. *)
signature T =   
   sig
      type s
      structure U :
         sig
            type 'a t
            type u = (int * real) t
         end where type 'a t = s
   end where type U.u = int;
