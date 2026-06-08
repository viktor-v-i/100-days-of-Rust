What i learned / did:
Learned:
- A Vec<T> is a growable list where every item is the same type (used it as a router's packet buffer).
- Vec::new() makes an empty one, .push() adds to it, {:?} prints the whole list.
- Reading: packets[i] crashes if the index isn't there, but .get(i) returns an Option (Some/None)-
  so i can handle a missing packet with match instead of panicking.
- Looping with &packets borrows the list (i still own it after). Without the &, a for loop-
  takes ownership and moves the vec, so i couldn't use it again (same idea as day-10 moves).
- Stretch: &mut packets lets me change each value in place, and *size reaches through the-
  reference to the real value to overwrite it (clamped packets to the MTU of 1500).

Did:
challenge day-34.
