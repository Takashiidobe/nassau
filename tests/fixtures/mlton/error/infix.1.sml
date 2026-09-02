(* mlton regression/fail/infix.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* error *)
local
   fun _ f _ = ()
in
end

(* error *)
local
   fun (_ f _) = ()
in
end

(* error *)
local
   fun (_ f _) _ = ()
in
end

infix && ||

(* error *)
local
   fun && x = ()
in
end

(* error *)
local
   fun x && = ()
in
end

(* error *)
local
   fun && || = ()
in
end

(* error *)
local
   fun || && = ()
in
end
(* CHECK-ERR: × expected a function name *)
(* CHECK-ERR: :4:10] *)
