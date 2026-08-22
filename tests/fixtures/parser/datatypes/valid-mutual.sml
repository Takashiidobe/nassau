datatype tree = Tree of forest
and forest = Empty | Trees of tree * forest
datatype 'a stream = Stream of 'a * 'a stream option
withtype 'a pending = unit -> 'a stream
datatype expr = Num of int | Add of pair
withtype pair = expr * expr
(* CHECK-STDOUT: (datatype (tree () (Tree forest)) (forest () (Empty) (Trees {{[(]}}* tree forest)))) *)
(* CHECK-STDOUT-NEXT: (datatype (stream ('a) (Stream {{[(]}}* 'a (tycon option (tycon stream 'a))))) (withtype (pending ('a) (-> unit (tycon stream 'a))))) *)
(* CHECK-STDOUT-NEXT: (datatype (expr () (Num int) (Add pair)) (withtype (pair () {{[(]}}* expr expr)))) *)
(* CHECK-RUN-EXIT: 0 *)
