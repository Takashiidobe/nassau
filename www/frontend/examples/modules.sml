signature NUMBER = sig
  val value : int
end;

functor Double (N : NUMBER) = struct
  val value = N.value * 2
end;

structure TwentyOne = struct
  val value = 21
end;

structure Answer = Double (TwentyOne);
Answer.value;
