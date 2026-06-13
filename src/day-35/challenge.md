# Day 35 — Strings: Splitting a `host:port` Address

> **XP:** base 100  ·  **Chapter:** Ch 8.2  ·  **Type:** 🧩 concept
> **needs:** slices, ownership  ·  **unlocks:** parsing, I/O, HashMap keys
> **Today's theme:** networking — taking an address like `"example.com:8080"` apart and putting it back together

---

## 📖 Read first
Open `The Rust Programming Language.pdf` and read:
- **Storing UTF-8 Encoded Text with Strings** — PDF pages **193–203**

Read it once for the big picture. Then come back here; this file is your only reference while you code.

> Work top to bottom. Run `cargo run` after **each** step so you see it working before moving on.

---

## 🎯 Goal
Start with an address string like `"example.com:8080"`, find the `:` that separates the host from the
port, and slice it into the two pieces. Then rebuild a clean, normalized version of it. All text — no number
parsing today.

---

## ✅ Requirements

### Step 1 — Build the address as an owned `String`
A `&str` (a *string slice* — a borrowed view into text) is fixed. A `String` is the **owned, growable**
kind. Make an empty one and grow it with `.push_str()`.
```rust
let mut addr = String::new();   // owned, growable, starts empty
addr.push_str("example.com");   // append more text onto it
addr.push(':');                 // .push() adds a single char
addr.push_str("8080");
println!("{addr}");
```
- `String::new()` makes an empty owned string.
- `.push_str(&str)` appends a slice; `.push(char)` appends one character.

### Step 2 — Find the separator
`.find()` looks for a character and hands back an `Option` (a "maybe" — `Some(index)` or `None`), so it
never crashes if there's no `:`. The index it gives you is a **byte position** in the string.
```rust
match addr.find(':') {
    Some(i) => println!("colon is at byte {i}"),
    None => println!("no port given"),
}
```

### Step 3 — Slice it into host and port
Inside the `Some(i)` branch, use **slicing** to take the two pieces. Slicing borrows part of the string as a
`&str` — `&addr[..i]` is "from the start up to (not including) `i`".
```rust
let host = &addr[..i];      // before the colon
let port = &addr[i + 1..];  // after the colon  (i+1 skips the ':' itself)
println!("host = {host}, port = {port}");
```
> ⚠️ Slicing works on **byte** indices, not characters. That's fine for `:` (1 byte). For multi-byte
> characters (like emoji or accents) slicing mid-character would panic — note *why* in your reflection.

### Step 4 — Rebuild a normalized address
Use `format!` (builds a new `String` from pieces, like `println!` but returns the text instead of printing)
to assemble a clean version — e.g. force the host to lowercase.
```rust
let normalized = format!("{}:{}", host.to_lowercase(), port);
println!("normalized = {normalized}");
```

### Step 5 — Do it for several addresses *(spaced repetition: `Vec` + `for &` from day-34)*
Put a few addresses in a `Vec`, then loop over them **by reference** and run your splitting logic on each.
```rust
let addresses = vec!["example.com:8080", "LOCALHOST:443", "no-port-here"];
for addr in &addresses {        // &addresses borrows the list (you still own it)
    // find ':' and handle Some / None for each one
}
```
The `"no-port-here"` entry should hit your `None` branch — prove it doesn't crash.

---

## 🌟 Stretch (optional — only if Steps 1–5 felt easy)
**Count the host labels.** A host like `api.example.com` has 3 labels split by `.`. Use `.split('.')` (which
hands you an iterator of pieces) and count them — `.split('.').count()`. Print "host has N labels".
```rust
let labels = host.split('.').count();
```

---

## 💻 Terminal commands (your memory aids)
```bash
cd src/day-35
cargo check        # does it compile? (no program made)
cargo run          # compile + run it
cargo fmt          # auto-format (build the habit!)
```

---

## 🧠 Reflect (you'll answer these at /end)
- What's the difference between a `String` and a `&str`, and why did Step 1 need the owned `String`?
- Slicing uses **byte** indices. Why is `&addr[..i]` safe for a `:` but risky in general for arbitrary text?

---

## 🆘 Stuck?
- `/teacher <your question>` — I find the missing piece behind it.
- `/hint` — smallest next nudge.
- `/harder` or `/easier` — re-tune this challenge.
