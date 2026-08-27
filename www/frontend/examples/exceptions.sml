exception TooSmall of int;

fun check n = if n < 10 then raise TooSmall n else n;

check 3 handle TooSmall n => n + 10;
check 2;
check 42;
