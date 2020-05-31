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
		 .conflicts_with("b")) // NOTE: not defining conflictions envokes undefined behavior
            .arg(Arg::with_name("b")
                 .short("b")
		 .help("Another switch to change the output of foo (try it)")
		 .conflicts_with("a"))
            .get_matches()
    };
}


#[default]
fn main() {
    println!("Default");
}

#[override_flag(flag = a)]
fn main() {
    println!("Changed by a flag");
}

#[override_flag(flag = b)]
fn main() {
    println!("Changed by a different flag");
}
