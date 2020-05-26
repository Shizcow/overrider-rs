// overrider works on entire impl blocks and supports partial overriding

use overrider::*;

struct Dummy {}

#[default]
impl Dummy {
    fn foo(&self) -> &'static str {
	"default foo"
    }
    fn bar(&self) -> &'static str {
	"default bar"
    }
    fn baz(&self) -> &'static str {
	"default baz"
    }
}


// You can comment out the whole block, or only single functions
#[override_default]
impl Dummy {
    fn foo(&self) -> &'static str {
	"overriden foo"
    }
    fn bar(&self) -> &'static str {
	"overriden bar"
    }
    /* See, not everything needs to be overriden at once
    fn baz(&self) -> &'static str {
	"overriden baz"
    }
     */
}

fn main() {
    let dummy = Dummy{};
    println!("{}", dummy.foo());
    println!("{}", dummy.bar());
    println!("{}", dummy.baz());
}
