use overrider::*;
use clap::{Arg, ArgMatches, App};

// clap parsing is up to you
lazy_static::lazy_static! {
    static ref CLAP_FLAGS: ArgMatches<'static> = {
	App::new("Overrider example - flag")
        .version(env!("CARGO_PKG_VERSION"))
        .about("An example showing overriding based on command line args")
        .arg(Arg::with_name("switch")
                 .short("s")
                 .long("switch")
                 .help("A switch to change the output of foo (try it)"))
        .get_matches()
    };
}


#[default]
fn main() {
    println!("Default");
}

#[override_flag(flag = switch)]
fn main() {
    println!("Changed by flag --switch");
}
