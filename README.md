# overrider-rs

`overrider` is a Rust crate that provides function overloading. For example:

```rust
#[default]
fn foo() {
    println!("Hello World");
}
```

Calling `foo()` will print `Hello World`. However, if a `override_default` version
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
