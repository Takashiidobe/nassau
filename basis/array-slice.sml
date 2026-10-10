structure ArraySlice =
struct
  type 'a slice = 'a Array.array * int * int
  fun length (_,_,n) = n
  fun sub ((a,i,n),j) = if j < 0 orelse j >= n then raise Subscript else Array.sub (a,i+j)
  fun update ((a,i,n),j,x) = if j < 0 orelse j >= n then raise Subscript else Array.update (a,i+j,x)
  fun slice (a,i,size) = let val n=Array.length a val k=case size of NONE => n-i | SOME x => x
    in if i < 0 orelse i > n orelse k < 0 orelse k > n-i then raise Subscript else (a,i,k) end
  fun full a = slice (a,0,NONE)
  fun subslice ((a,i,n),j,size) = let val m=case size of NONE => n-j | SOME x => x
    in if j < 0 orelse j > n orelse m < 0 orelse m > n-j then raise Subscript else (a,i+j,m) end
  fun base s = s
  fun vector s = Vector.tabulate (length s, fn i => sub (s,i))
  fun copy {src,dst,di} = let val n=length src val d=Array.length dst
    in if di < 0 orelse di > d orelse n > d-di then raise Subscript else
      let val snapshot=vector src in Array.copyVec {src=snapshot,dst=dst,di=di} end end
  fun copyVec {src,dst,di} = let val n=VectorSlice.length src val d=Array.length dst
    in if di < 0 orelse di > d orelse n > d-di then raise Subscript else
      let val snapshot=VectorSlice.vector src in Array.copyVec {src=snapshot,dst=dst,di=di} end end
  fun isEmpty s = length s = 0
  fun getItem s = if isEmpty s then NONE else SOME (sub(s,0),subslice(s,1,NONE))
  fun appi f s = let fun loop i = if i=length s then () else (f(i,sub(s,i));loop(i+1)) in loop 0 end
  fun app f s = appi (fn (_,x)=>f x) s
  fun modifyi f s = appi (fn(i,x)=>update(s,i,f(i,x))) s
  fun modify f s = modifyi (fn(_,x)=>f x) s
  fun foldli f init s = let fun loop(i,a)=if i=length s then a else loop(i+1,f(i,sub(s,i),a)) in loop(0,init) end
  fun foldri f init s = let fun loop(i,a)=if i<0 then a else loop(i-1,f(i,sub(s,i),a)) in loop(length s-1,init) end
  fun foldl f init s = foldli(fn(_,x,a)=>f(x,a)) init s
  fun foldr f init s = foldri(fn(_,x,a)=>f(x,a)) init s
  fun findi p s = let fun loop i=if i=length s then NONE else let val x=sub(s,i) in if p(i,x) then SOME(i,x) else loop(i+1) end in loop 0 end
  fun find p s = case findi(fn(_,x)=>p x)s of NONE=>NONE | SOME(_,x)=>SOME x
  fun exists p s = case find p s of NONE=>false | SOME _=>true
  fun all p s = not(exists(not o p)s)
  fun collate cmp (a,b) = let val n=Int.min(length a,length b)
    fun loop i=if i=n then Int.compare(length a,length b) else case cmp(sub(a,i),sub(b,i)) of EQUAL=>loop(i+1)|x=>x in loop 0 end
end
