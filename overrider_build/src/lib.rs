use syn::{Type::Path, ImplItem::Method};
use std::fs::File;
use std::io::Read;

#[macro_export]
macro_rules! watch_files {
    ($arg:literal) => { // I accept a single string literal...
	$crate::watch_file($arg);
    };
    ($args:expr) => { // ...an iteratable (think vector)...
	for arg in $args.iter() {
	    $crate::watch_file(arg);
	}
    };
    ($arg:literal, $($args:literal),*) => { // ...or a list of the above (prioritize literals)
	watch_files!($arg);
	watch_files!($($args),*);
    };
    ($arg:literal, $($args:expr),*) => {
	watch_files!($arg);
	watch_files!($($args),*);
    };
    ($arg:expr, $($args:literal),*) => {
	watch_files!($arg);
	watch_files!($($args),*);
    };
    ($arg:expr, $($args:expr),*) => {
	watch_files!($arg);
	watch_files!($($args),*);
    };
}

pub fn watch_file(file_name: &str) {
    let mut file = File::open(file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut src = String::new(); 
    file.read_to_string(&mut src).expect(&format!("Unable to read file '{}'", file_name));

    for item in syn::parse_file(&src).expect(&format!("Unable to parse file '{}'", file_name)).items {
	match item {
	    syn::Item::Fn(func) => { // a function is overriden
		if func.attrs.iter().any(|attr| attr.path.segments[0].ident.to_string() == "override_default") {
		    println!("cargo:rustc-cfg=__override_func_{}", func.sig.ident.to_string());
		}
	    },
	    syn::Item::Impl(impl_block) => { // an impl block is overriden -- check each item within (methods, consts, etc)
		if impl_block.attrs.iter().any(|attr| attr.path.segments[0].ident.to_string() == "override_default") {
		    let self_type = match impl_block.self_ty.as_ref() { // The `Dummy` in `impl Dummy {}`
			Path(path) => path,
			_ => panic!("Could not get Path for impl (should never see this)"),
		    }.path.segments[0].ident.to_string();
		     
		    for item in impl_block.items {
			match item {
			    Method(method) =>
				println!("cargo:rustc-cfg=__override_method_{}_{}",
					 self_type,
					 &method.sig.ident),
			_ => panic!("I can't overload anything other than methods in an impl block yet"),
			}
		    }
		}
	    },
	    _ => {} // Ignore everything else. The eventual goal is to be able to override everything, but that's not done yet.
	}
    }
}
