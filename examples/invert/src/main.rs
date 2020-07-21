use overrider::*;
use clap::{Arg, ArgMatches, App};

// clap parsing is up to you
lazy_static::lazy_static! {
    static ref CLAP_FLAGS: ArgMatches<'static> = {
	App::new("Overrider example - flag")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(Arg::with_name("disable")
                 .long("disable"))
            .get_matches()
    };
}

#[default]
fn foo() {
    println!("Default");
}

#[override_flag(flag = disable, invert = true)]
fn foo() {
    println!("Overriden default");
}

fn main() {
    foo();
}
