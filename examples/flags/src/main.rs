use overrider::*;
use clap::{Arg, ArgMatches, App};

// clap parsing is up to you
lazy_static::lazy_static! {
    static ref CLAP_FLAGS: ArgMatches<'static> = {
	App::new("Overrider example - flag")
            .version(env!("CARGO_PKG_VERSION"))
            .about("An example showing overriding based on command line args")
            .arg(Arg::with_name("a")
                 .short("a")
		 .help("A switch to change the output of foo (try it)")
		 .conflicts_with("b")) // NOTE: defining both is currently undefined behavior
            .arg(Arg::with_name("b")
                 .short("b")
		 .help("Another switch to change the output of foo (try it)"))
            .get_matches()
    };
}
/*
// Must provide a default case
#[default]
fn foo() {
    println!("Default fn");
}

#[override_flag(flag = a)]
fn foo() {
    println!("fn changed by a flag");
}

#[override_flag(flag = b)]
fn foo() {
    println!("fn changed by a different flag");
}
*/
// syntax for impls is similar
struct Dummy{}

#[default]
impl Dummy {
    pub fn foo() {
	println!("Default impl");
    }
}

#[override_flag(flag = a)]
impl Dummy {
    pub fn foo() {
	println!("Flag overriden impl");
    }
}


fn main() {
    foo();
    Dummy::__override_flagext_a_foo();
}
