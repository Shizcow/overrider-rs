use overrider::*;
#[allow(unused_imports)]
use secondary_lib::*;

#[default]
pub fn foo() {
    println!("Default");
}

fn main() {
    foo();
}
