structure VectorSlice =
struct
  type 'a slice = 'a Vector.vector * int * int
  fun length (_,_,n)=n
  fun sub ((v,i,n),j)=if j<0 orelse j>=n then raise Subscript else Vector.sub(v,i+j)
  fun slice(v,i,size)=let val n=Vector.length v val k=case size of NONE=>n-i | SOME x=>x
    in if i<0 orelse i>n orelse k<0 orelse k>n-i then raise Subscript else (v,i,k) end
  fun full v=slice(v,0,NONE)
  fun subslice (s as (v,i,n),j,size)=let val k=case size of NONE=>n-j | SOME x=>x
    in if j<0 orelse j>n orelse k<0 orelse k>n-j then raise Subscript else (v,i+j,k) end
  fun base s=s
  fun vector s=Vector.tabulate(length s,fn i=>sub(s,i))
  fun concat slices=Vector.concat(List.map vector slices)
  fun isEmpty s=length s=0
  fun getItem s=if isEmpty s then NONE else SOME(sub(s,0),subslice(s,1,NONE))
  fun appi f s=let fun loop i=if i=length s then () else (f(i,sub(s,i));loop(i+1)) in loop 0 end
  fun app f s=appi(fn(_,x)=>f x)s
  fun mapi f s=Vector.tabulate(length s,fn i=>f(i,sub(s,i)))
  fun map f s=mapi(fn(_,x)=>f x)s
  fun foldli f init s=let fun loop(i,a)=if i=length s then a else loop(i+1,f(i,sub(s,i),a)) in loop(0,init) end
  fun foldri f init s=let fun loop(i,a)=if i<0 then a else loop(i-1,f(i,sub(s,i),a)) in loop(length s-1,init) end
  fun foldl f init s=foldli(fn(_,x,a)=>f(x,a))init s
  fun foldr f init s=foldri(fn(_,x,a)=>f(x,a))init s
  fun findi p s=let fun loop i=if i=length s then NONE else let val x=sub(s,i) in if p(i,x) then SOME(i,x) else loop(i+1) end in loop 0 end
  fun find p s=case findi(fn(_,x)=>p x)s of NONE=>NONE|SOME(_,x)=>SOME x
  fun exists p s=case find p s of NONE=>false|SOME _=>true
  fun all p s=not(exists(not o p)s)
  fun collate cmp(a,b)=let val n=Int.min(length a,length b)
    fun loop i=if i=n then Int.compare(length a,length b) else case cmp(sub(a,i),sub(b,i)) of EQUAL=>loop(i+1)|x=>x in loop 0 end
end
