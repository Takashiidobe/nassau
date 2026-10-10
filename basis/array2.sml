structure Array2 =
struct
  datatype traversal = RowMajor | ColMajor
  type 'a array = {rows : int, cols : int, data : 'a Array.array} ref
  type 'a region = {base : 'a array, row : int, col : int, nrows : int option, ncols : int option}

  fun array (rows, cols, init) =
    if rows < 0 orelse cols < 0 orelse (rows <> 0 andalso cols > Array.maxLen div rows)
    then raise Size
    else ref {rows = rows, cols = cols, data = Array.array (rows * cols, init)}

  fun dimensions a = let val {rows, cols, ...} = !a in (rows, cols) end
  fun nRows a = #1 (dimensions a)
  fun nCols a = #2 (dimensions a)
  fun index (a, row, col) =
    let val {rows, cols, data} = !a
    in if row < 0 orelse row >= rows orelse col < 0 orelse col >= cols then raise Subscript
       else (data, row * cols + col) end
  fun sub (a, row, col) = let val (data, i) = index (a, row, col) in Array.sub (data, i) end
  fun update (a, row, col, value) = let val (data, i) = index (a, row, col) in Array.update (data, i, value) end

  fun fromList rows =
    case rows of
      [] => ref {rows = 0, cols = 0, data = Array.fromList []}
    | first :: _ =>
        let val cols = List.length first
            val _ = if List.exists (fn row => List.length row <> cols) rows then raise Size else ()
            val flat = List.concat rows
            val values = Array.fromList flat
            val rowsN = List.length rows
        in ref {rows = rowsN, cols = cols, data = values} end

  fun tabulate trv (rows, cols, f) =
    let val result = array (rows, cols, ())
        fun rowMajor i j = if i = rows then () else if j = cols then rowMajor (i + 1) 0
          else (update (result, i, j, f (i, j)); rowMajor i (j + 1))
        fun colMajor i j = if j = cols then () else if i = rows then colMajor 0 (j + 1)
          else (update (result, i, j, f (i, j)); colMajor (i + 1) j)
    in (case trv of RowMajor => rowMajor 0 0 | ColMajor => colMajor 0 0); result end

  fun row (a, i) = if i < 0 orelse i >= nRows a then raise Subscript
    else Vector.tabulate (nCols a, fn j => sub (a, i, j))
  fun column (a, j) = if j < 0 orelse j >= nCols a then raise Subscript
    else Vector.tabulate (nRows a, fn i => sub (a, i, j))

  fun bounds {base, row, col, nrows, ncols} =
    let val (rows, cols) = dimensions base
        val nr = case nrows of NONE => rows - row | SOME n => n
        val nc = case ncols of NONE => cols - col | SOME n => n
    in if row < 0 orelse col < 0 orelse nr < 0 orelse nc < 0 orelse row > rows orelse col > cols
         orelse nr > rows - row orelse nc > cols - col then raise Subscript
       else (nr, nc) end
  fun coordinates trv (r, c, f) =
    let fun rm i j = if i = r then () else if j = c then rm (i + 1) 0 else (f (i,j); rm i (j+1))
        fun cm i j = if j = c then () else if i = r then cm 0 (j+1) else (f (i,j); cm (i+1) j)
    in case trv of RowMajor => rm 0 0 | ColMajor => cm 0 0 end
  fun appi trv f reg =
    let val {base, row, col, ...} = reg val (nr,nc) = bounds reg
    in coordinates trv (nr,nc,fn (i,j) => f (row+i,col+j,sub (base,row+i,col+j))) end
  fun app trv f a = appi trv (fn (_,_,x) => f x) {base=a,row=0,col=0,nrows=NONE,ncols=NONE}
  fun foldi trv f init reg =
    let val {base,row,col,...} = reg val (nr,nc) = bounds reg
        val coords = ref []
        val _ = coordinates trv (nr,nc,fn (i,j) => coords := (i,j)::(!coords))
    in List.foldl (fn ((i,j),acc) => f (row+i,col+j,sub (base,row+i,col+j),acc)) init (List.rev (!coords)) end
  fun fold trv f init a = foldi trv (fn (_,_,x,acc) => f (x,acc)) init {base=a,row=0,col=0,nrows=NONE,ncols=NONE}
  fun modifyi trv f reg = appi trv (fn (i,j,x) => update (#base reg,i,j,f (i,j,x))) reg
  fun modify trv f a = modifyi trv (fn (_,_,x) => f x) {base=a,row=0,col=0,nrows=NONE,ncols=NONE}

  fun copy {src, dst, dst_row, dst_col} =
    let val {base,row,col,...} = src val (nr,nc) = bounds src
        val rows = nRows dst val cols = nCols dst
        val _ = if dst_row < 0 orelse dst_col < 0 orelse dst_row > rows orelse dst_col > cols
          orelse nr > rows - dst_row orelse nc > cols - dst_col then raise Subscript else ()
        val snapshot = Array.tabulate (nr * nc, fn k => sub (base,row+k div (if nc=0 then 1 else nc),col+k mod (if nc=0 then 1 else nc)))
    in coordinates RowMajor (nr,nc,fn (i,j) => update (dst,dst_row+i,dst_col+j,Array.sub(snapshot,i*nc+j))) end
end
