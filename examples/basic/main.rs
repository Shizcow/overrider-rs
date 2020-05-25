#![allow(unused)]

extern crate liboverloader;
use liboverloader::*;

struct Flagger {
    flag: String,
}

impl Flagger {
    pub fn new(flag: String) -> Self {
	Self{flag}
    }
}

// Example: Basic function
#[override_default]
impl Flagger {
    #[inline]
    pub fn do_foo(&self) -> &'static str {
	"overriden default"
    }
}


#[allow(non_snake_case)]
impl Flagger {
    pub fn do_foo(&self) -> &'static str {
	match self.flag {
	    _ => self.__do_foo_default(),
	}
    }
}

/*
#[cfg(not(func_overriden = "__Flagger_do_foo_default"))]
#[inline]
fn ___do_foo_default(&self) -> &str {
    "univeral default"
}
*/

/*
// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"
*/

fn main() {
    let flagger = Flagger::new("a".to_string());
    println!("{}", flagger.do_foo());
}
