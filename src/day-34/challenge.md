# Day 34 — Vectors: a Router's Packet Buffer

> **XP:** base 100  ·  **Chapter:** Ch 8.1  ·  **Type:** 🧩 concept
> **needs:** ownership, generics-intuition  ·  **unlocks:** HashMap, iterators, byte buffers
> **Today's theme:** networking — a router collecting incoming packets

---

## 📖 Read first
Open `The Rust Programming Language.pdf` and read:
- **Storing Lists of Values with Vectors** — PDF pages **187–193**

Read it once for the big picture. Then come back here; this file is your only reference while you code.

> Work top to bottom. Run `cargo run` after **each** step so you see it working before moving on.

---

## 🎯 Goal
Model a router that receives packets into a growing list (`Vec`), reads one safely, and adds up the total
bytes received. Small steps — one idea at a time.

---

## ✅ Requirements

### Step 1 — Make an empty list of packet sizes
A `Vec` (a growable list, all items the same type) of packet sizes in bytes (`u32`).
```rust
let mut packets: Vec<u32> = Vec::new();
```
- `mut` because it will grow.
- `<u32>` tells Rust what kind of values go in (unsigned 32-bit numbers).

### Step 2 — Add three packets
Use `.push()` to add three arriving packet sizes (pick any numbers, e.g. 64, 512, 1500).
```rust
packets.push(64);
// ...add two more
```
Then print the whole list to check it:
```rust
println!("{:?}", packets);   // {:?} prints the "debug" view of the vector
```
Run it. You should see all three numbers.

### Step 3 — Read one packet, the safe way
Print packet number 2 using `.get()`. `.get()` returns an `Option` (a "maybe" — either `Some(value)` or
`None`), so it never crashes if the index is missing.
```rust
match packets.get(1) {        // index 1 = the 2nd packet (counting starts at 0)
    Some(size) => println!("Packet 2 is {size} bytes"),
    None => println!("No packet there"),
}
```
Now try `packets.get(99)` and run again — see how you get the `None` branch instead of a crash.

### Step 4 — Add up the total bytes (throughput)
Loop over the list with a reference (`&`) and sum the sizes into a `mut` total.
```rust
let mut total: u32 = 0;
for size in &packets {        // &packets = "borrow the list, don't take it"
    total += size;
}
println!("Total received: {total} bytes");
```

---

## 🌟 Stretch (optional — only if Steps 1–4 felt easy)
**Clamp to the MTU.** The MTU (Maximum Transmission Unit — the biggest packet a link allows) is 1500. Loop
again, this time with `&mut`, and shrink any packet bigger than 1500 down to 1500.
```rust
for size in &mut packets {
    *size = (*size).min(1500);   // the * reaches through the reference to the real value
}
```

---

## 💻 Terminal commands (your memory aids)
```bash
cd src/day-34
cargo check        # does it compile? (no program made)
cargo run          # compile + run it
cargo fmt          # auto-format (build the habit!)
```

---

## 🧠 Reflect (you'll answer these at /end)
- What does `.get()` hand you back, and why is that safer than `packets[i]`?
- In Step 4, what does the `&` in `for size in &packets` do — and what would happen without it?

---

## 🆘 Stuck?
- `/teacher <your question>` — I find the missing piece behind it.
- `/hint` — smallest next nudge.
- `/harder` or `/easier` — re-tune this challenge.
