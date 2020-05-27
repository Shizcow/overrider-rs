use overrider::*;

// Here, we define a few different main overrides

#[default]
fn main() {
    println!("Default");
}

#[override_default]
fn main() {
    println!("Overriden");
}

#[override_default(priority = 2)]
fn main() {
    println!("Overriden more");
}

// Now comes final. This intentionally throws a compiler error
//   with the required priority to make this overload final
#[finals]
fn main() {
    println!("Ok this is the last one");
}
