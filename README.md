# overrider-rs

`overrider` is a Rust crate that provides overloading of functions, methods, and more. For example:

```rust
#[default]
fn foo() {
    println!("Hello World");
}
```

Calling `foo()` will print `Hello World`. However, if an `override_default` version
of `foo` is defined:

```rust
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
Due to limitations in `proc_macro`, `overrider` requires use of it's sister crate [`overrider-build`](https://github.com/Shizcow/overrider-build-rs)
in a build script. However, it's pretty easy to use:
```rust
fn main() { // build.rs
    overrider_build::watch_files!("src/main.rs");
}

```

