# overrider-rs

`overrider` is a set of Rust crates that provide overloading of functions, methods, and more. For example:

```rust
// main.rs
use overrider::*;
#[default]
fn foo() {
    println!("Hello World");
}
```

Calling `foo()` will print `Hello World`. However, if an `override_default` version
of `foo` is defined:

```rust
// main.rs
use overrider::*;
#[default]
fn foo() {
    println!("Hello World");
}

#[override_default]
fn foo() {
    println!("Hello Za Warudo");
}
```

Calling `foo()` will now print `Hello Za Warudo`. The first function definition may remain.


## Using
Due to limitations in `proc_macro`, `overrider` requires the use of two crates:
- [`overrider`](https://crates.io/crates/overrider) for code in `src`
- [`overrider_build`](https://crates.io/crates/overrider_build) for a build script such as `build.rs`
The code from above shows how to use the `overrider` crate.
Below is how to use the build portion:
```rust
// build.rs
use overrider_build::watch_files;
fn main() {
    watch_files!("src/main.rs");
}

```

For examples, see [the git repo](https://github.com/Shizcow/overrider-rs/tree/master/examples).
