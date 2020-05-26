use overrider::*;

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

struct Dummy {}

// Example: Method
// comment out the following definition and output changes
#[override_default]
impl Dummy {
    fn method_foo(&self) -> &'static str {
	"overriden method"
    }
}

#[default]
impl Dummy {
    fn method_foo(&self) -> &'static str {
	"default method"
    }
}

fn main() {
    let dummy = Dummy{};
    println!("{}", dummy.method_foo());

    println!("{}", func_foo());
}
