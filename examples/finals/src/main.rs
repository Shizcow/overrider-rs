use overrider::*;

// Here, we define a few different function overrides

#[default]
fn foo() {
    println!("Default function");
}

#[override_default]
fn foo() {
    println!("Overriden function");
}

#[override_default(priority = 2)]
fn foo() {
    println!("Overriden function more");
}

// Now comes final. This intentionally throws a compiler error
//   with the required priority to make this overload final
// Uncomment to see this error
/*
#[override_final]
fn foo() {
    println!("Ok this is the last one");
}
*/


// impls also support final. Here's the setup:
struct Dummy{}

#[default]
impl Dummy {
    fn foo(&self) {
	println!("Default method");
    }
}

#[override_default(priority = 5)] // skip to high numbers (because we can)
impl Dummy {
    fn foo(&self) {
	println!("Overriden method");
    }
}

// Again, uncomment to show error
/*
#[override_final]
impl Dummy {
    fn foo(&self) {
	println!("Final method");
    }
}
*/


// And here's main, just for show
fn main() {
    foo();
    let dummy = Dummy{};
    dummy.foo();
}
