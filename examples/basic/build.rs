use std::fs::File;
use std::io::Read;

fn main() {
    // TODO: crawl whole project
    let mut file = File::open("src/main.rs").expect("Unable to open file");
    let mut src = String::new(); 
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    for item in syntax.items {
	match item {
	    syn::Item::Fn(func) => { // a function is overriden
		if func.attrs.iter().any(|attr| attr.path.segments[0].ident.to_string() == "override_default") {
		    println!("cargo:rustc-cfg=__override_func_{}", func.sig.ident.to_string());
		}
	    },
	    syn::Item::Impl(impl_block) => { // an impl block is overriden
		if impl_block.attrs.iter().any(|attr| attr.path.segments[0].ident.to_string() == "override_default") {
		    let self_type = match impl_block.self_ty.as_ref() {
			syn::Type::Path(path) => path,
			_ => panic!("Could not get Path for impl (should never see this)"),
		    }.path.segments[0].ident.to_string();
		     
		    for item in impl_block.items {
			match item {
			    syn::ImplItem::Method(method) => {
				println!("cargo:rustc-cfg=__override_method_{}_{}",
					 self_type,
					 &method.sig.ident); 
			},
			_ => panic!("I can't overload anything other than methods in an impl block yet"),
			}
		    }
		}
	    },
	    _ => {}
	}
    }
}
