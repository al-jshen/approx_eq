# approx_eq

[![Build Status](https://travis-ci.org/al-jshen/approx_eq.svg?branch=master)](https://travis-ci.org/al-jshen/approx_eq)
[![Crates.io](https://img.shields.io/crates/v/approx_eq)](https://crates.io/crates/approx_eq)
[![Documentation](https://docs.rs/approx_eq/badge.svg)](https://docs.rs/approx_eq)

This crate provides a macro to check whether two numbers are approximately equal. It does so by checking that the relative difference between the two numbers is less than some upper limit. 

To use this in your Rust program, add the following to your `Cargo.toml` file:

```rust
// Cargo.toml
[dependencies]
approx_eq = "0.1.0"
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
