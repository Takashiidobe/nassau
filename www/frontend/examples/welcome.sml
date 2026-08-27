fun map f [] = []
  | map f (x :: xs) = f x :: map f xs;

val squares = map (fn n => n * n) [1, 2, 3, 4, 5];

squares;
