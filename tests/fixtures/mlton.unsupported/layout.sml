(* mlton regression/layout.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)

fun layout (cbs : (string * real) list list) : string =
  let 
    val layoutcb =
      map (fn (con,_) => con)

    fun layoutdb cb = "{" ^ concat(layoutcb cb) ^ "}"
  in concat(map layoutdb cbs)
  end
(* CHECK-EXIT: 0 *)
