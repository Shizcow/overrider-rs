use overrider::*;

// What about overriding overrides?
// overrider allows setting priorities (default is zero)

// Here is the default/base implimentation
#[default]
fn main() {
    println!("Default");
}

// This is overriden
#[override_default]
fn main() {
    println!("Overriden");
}

// This is overriden again, with a higher priority
#[override_default(priority = 2)]
fn main() {
    println!("Overriden with a higher priority");
}
