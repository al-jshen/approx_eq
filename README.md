# approx_eq

This crate provides a macro to check whether two numbers are approximately equal. It does so by checking that the relative difference between the two numbers is less than some upper limit. 

To use this in your Rust program, add the following to your `Cargo.toml` file:

```rust
// Cargo.toml
[dependencies]
approx_eq = { git = "https://github.com/al-jshen/approx_eq" }
```

Using this macro is easy!

```rust
// main.rs
use approx_eq::assert_approx_eq;

fn main() {
  assert_approx_eq!(1., 0.99999999999); // should pass
  assert_approx_eq!(1., 0.99999999999, 1e-5); // should pass
  assert_approx_eq!(1., 0.99999999999, 1e-20); // should fail
  assert_approx_eq!(1., 2.) // should fail
}
```
