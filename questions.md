"Act as my Rust programming tutor. Ask me one question at a time covering core Rust concepts like ownership, borrowing, the borrow checker, enums, pattern matching, and lifetimes. Wait for my answer before moving on, then give me detailed feedback explaining what I got right, what I missed, and the full concept with code examples. Adjust difficulty based on my answers."


# 🦀 Rust Tutoring

---

## Question 1 — Ownership

**What happens when you run this code?**
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1); // ❌ Compiler error!
}
```

**Key Concept: Move Semantics**

When you assign `s1` to `s2`, Rust **moves** ownership — `s1` is no longer valid. This prevents double-free memory errors without needing a garbage collector.

**How to fix it:**
```rust
// Option 1: Clone the data
let s2 = s1.clone();

// Option 2: Borrow instead of move
let s2 = &s1;
```

**Remember:**
- **Move** = ownership transfers, original variable is gone
- **Borrow** = you lend a reference, original variable stays valid

---

## Question 2 — Borrowing

**What's the difference between `&s` and `&mut s`?**

| | `&s` (Immutable Reference) | `&mut s` (Mutable Reference) |
|---|---|---|
| Read | ✅ | ✅ |
| Modify | ❌ | ✅ |
| How many at once | Unlimited | Only one |

```rust
// ✅ Many immutable refs — fine
let r1 = &s;
let r2 = &s;

// ✅ One mutable ref — fine
let r3 = &mut s;

// ❌ Mutable + immutable at the same time — compiler error!
let r1 = &s;
let r3 = &mut s; // not allowed while r1 is still in use
```

---

## Question 3 — The Borrow Checker

**Why can't you have a mutable and immutable reference at the same time?**

Because if the mutable reference changes the value, the immutable reference would see unexpected/stale data — this is called a **data race**.

In other languages this causes:
- Undefined behavior
- Hard-to-track bugs that only appear sometimes
- Crashes in multithreaded code

**Rust catches all of this at compile time** — these bugs literally cannot exist in safe Rust.

---

## Question 4 — Enums & Option\<T\>

**What is `Option<T>` and why does Rust use it instead of `null`?**

```rust
enum Option<T> {
    Some(T),  // there IS a value
    None,     // there is NO value
}
```

`null` is dangerous because any variable can secretly be `null` — forget to check and your program crashes (null pointer exception). Tony Hoare, the inventor of `null`, called it his *"billion dollar mistake."*

`Option<T>` forces you to explicitly handle both cases:
```rust
let name: Option<String> = Some(String::from("Alice"));

match name {
    Some(n) => println!("Hello, {}", n),
    None => println!("No name found"),
}
```

If you try to use the value without checking — **the compiler refuses to compile.** Bug caught before runtime. ✅

---

## Question 5 — Lifetimes

**What is a lifetime and why do we sometimes annotate them explicitly?**

A lifetime is how long a reference is guaranteed to point to valid data. Rust tracks these automatically most of the time, but needs help when it can't figure out how long a returned reference will live.

**Without annotation — compiler confused:**
```rust
// ❌ Is the output tied to x or y?
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**With lifetime annotation — compiler happy:**
```rust
// ✅ Output lives as long as the shortest input
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**The core problem lifetimes prevent — dangling references:**
```rust
let r;
{
    let x = 5;
    r = &x; // x dies here...
}
println!("{}", r); // ❌ r points to dead memory!
```

Rust catches this at compile time. You can never accidentally point to freed memory.

---

## Summary

| Concept | Key Takeaway |
|---|---|
| **Ownership** | Only one owner at a time; assigning moves ownership |
| **Borrowing** | `&s` = read-only, many allowed; `&mut s` = read-write, only one allowed |
| **Borrow Checker** | Prevents data races by enforcing borrowing rules at compile time |
| **Option\<T\>** | Safe alternative to null; forces you to handle the "no value" case |
| **Lifetimes** | Ensures references always point to valid data; annotate when compiler can't infer |
