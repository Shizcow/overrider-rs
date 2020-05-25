use std::fs::File;
use std::io::Read;

fn main() {
    // TODO: crawl whole project
    let mut file = File::open("src/main.rs").expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut overrides = Vec::new();

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    for item in syntax.items {
	match item {
	    syn::Item::Fn(func) => {
		if func.attrs.iter().any(|attr| attr.path.segments[0].ident.to_string() == "override_default") {
		    overrides.push(format!("func_{}", func.sig.ident.to_string()));
		}
	    },
	    _ => {}
	}
    }

    println!("cargo:rustc-cfg=overriden=\"{}\"", overrides.join(" "));
}
