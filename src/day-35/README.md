What i learned / did:

The difference between &str and String: &str is borrowed and fixed, String is owned and can grow.
That's why building the address up with push_str needed a String.

Slicing uses byte positions, not characters. I think it's something to do with the bytes changing as
the char changes - ':' is one byte so splitting on it is safe, but a bigger char would be more bytes and
slicing in the middle of it would break.

Did:
challenge day-35 - split a host:port address into host and port.
