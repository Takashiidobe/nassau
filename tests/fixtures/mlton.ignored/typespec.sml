(* mlton regression/typespec.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML rejects this program *)
(* typespec.sml *)

(* Checks scoping of definitional type specifications. *)

type t = int

signature S =
sig
    type t = bool
    and  u = t
end

structure X : S =
struct
    type t = bool
    and  u = bool
end;
