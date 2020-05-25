#![allow(unused)]

extern crate overrider;
use overrider::*;

struct Flagger {}

impl Flagger {
    pub fn new() -> Self {
	Self{}
    }
}

// Example: Basic function
// comment out the following definition and output changes
#[override_default]
fn func_foo() -> &'static str {
    "overriden function"
}

#[default]
fn func_foo() -> &'static str {
    "default function"
}

// Example: Method
// comment out the following definition and output changes
#[override_default]
impl Flagger {
    fn method_foo(&self) -> &'static str {
	"overriden method"
    }
}

#[default]
impl Flagger {
    fn method_foo(&self) -> &'static str {
	"default method"
    }
}

fn main() {
    let flagger = Flagger::new();
    println!("{}", flagger.method_foo());

    println!("{}", func_foo());
}
