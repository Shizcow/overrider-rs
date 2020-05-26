use overrider::*;

// Here is the default implimentation. Order does not matter.
#[default]
fn foo() {
    println!("Hello World");
}

// Try commenting this definition out. You'll see the output change
#[override_default]
fn foo() {
    println!("Hello Za Warudo");
}

fn main() {
    foo();
}
