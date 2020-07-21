use overrider::*;
use clap::{Arg, ArgMatches, App};

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
    println!("This is the old default");
}

#[override_flag(flag = disable, invert = true)]
fn foo() {
    println!("This is the new default, pass --disable to turn off");
}

fn main() {
    foo();
}
