What i learned / did
Learned:
String can grow and shrink in size, like appending .push_str()
for this reason the String cannot be allocated a fixed slot in memory and goes on the heap
because string literals are hard coded their size is known at compile so they receive a slot.
unique variables get dropped from memory after the scope ends which is why you'll get a compile error

Did:
Challenge day-08
