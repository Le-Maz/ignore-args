# ignore-args 🦀

A small utility crate that makes it easy to **wrap closures or functions
that take no arguments** (`Fn`, `FnMut`, or `FnOnce`) into callables
that can accept (and then ignore) _any_ arguments.
This is especially handy when dealing with APIs that expect a callback
with a specific argument list, but your logic doesn’t actually need them.

## ✨ Features

- Supports `Fn`, `FnMut`, and `FnOnce` closures.
- Discards all arguments, always invoking the underlying function with `()`.
- Fully inlined, zero-cost in release builds.
- Simple, minimal API (`ignore_args`, `ignore_args_mut`, `ignore_args_once`).

## 🚀 Example

```rust
use ignore_args::{ignore_args, ignore_args_mut, ignore_args_once};
fn say_hello() {
    println!("Hello, world!");
}
fn main() {
    // Wrap a plain function
    let f = ignore_args(say_hello);
    f(1, 2, 3); // arguments ignored
    // Wrap a mutable closure
    let mut counter = 0;
    let mut g = ignore_args_mut(|| { counter += 1; });
    g("anything");
    g();
    // Wrap a once-only closure
    let data = "once".to_string();
    let h = ignore_args_once(|| println!("runs {}", data));
    h("unused args");
}
```

## ⚠️ Notes

- Requires **nightly Rust** (uses `unboxed_closures`, `fn_traits`, `tuple_trait`).
- This crate is minimal and intended for adapter-style use cases
  (e.g., working with iterator combinators or callback-based APIs).
