(* mlton regression/fail/eqtype.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* This should fail because v is an eqtype and s does not admit equality.
 * Hence, the side condition on rule 64 fails.
 *)
signature T =   
   sig
      type s
      structure V:
         sig
            datatype v = V
         end where type v = s
   end 
(* CHECK-ERR: × cannot refine v: it is not an abstract type of the signature *)
(* CHECK-ERR: :9:10] *)
