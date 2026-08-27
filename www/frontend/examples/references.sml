fun counter start =
  let val current = ref start
  in fn () => (current := !current + 1; !current)
  end;

val next = counter 39;
next ();
next ();
next ();
