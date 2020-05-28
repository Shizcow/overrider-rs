use syn::{Type::Path, ImplItem::Method};
use std::fs::File;
use std::io::Read;

enum Status {Final, Empty}
use Status::*;
fn get_priority(attrs: &Vec<syn::Attribute>) -> Result<i32, Status> {
    for attr in attrs {
	if attr.path.segments[0].ident.to_string() == "override_default" {
	    if let Ok(syn::Expr::Paren(expr)) = syn::parse2::<syn::Expr>(attr.tokens.clone()) {
		if let syn::Expr::Assign(assign) = *expr.expr {
		    if let syn::Expr::Path(left) = *assign.left {
			if left.path.segments.len() == 1
			    && left.path.segments[0].ident.to_string() == "priority" {
				if let syn::Expr::Lit(lit) = *assign.right {
				    if let syn::Lit::Int(i) = lit.lit {
					if let Ok(priority) = i.base10_parse::<i32>() {
					    return Ok(priority);
					} else {
					    panic!("Invalid positive integer rvalue in macro invocation");
					}
				    } else {
					panic!("Expected integer rvalue in macro invocation");
				    }
				} else {
				    panic!("Expected rvalue literal in macro invocation");
				}
			    } else {
				panic!("Invalid lvalue in macro invocation");
			    }
		    } else {
			panic!("Unparsable lvalue in macro invocation");
		    }
		} else {
		    panic!("Invalid expression in macro invocation");
		}
	    } else { // might be default
		if attr.tokens.is_empty() {
		    return Ok(1);
		} else {
		    panic!("Invalid macro invocation");
		}
	    }
	} else if attr.path.segments[0].ident.to_string() == "default" {
	    if attr.tokens.is_empty() {
		return Ok(0);
	    } else {
		panic!("Unexpected arguement in macro invocation");
	    }
	} else if attr.path.segments[0].ident.to_string() == "finals" {
	    if attr.tokens.is_empty() {
		return Err(Final);
	    } else {
		panic!("Unexpected arguement in macro invocation");
	    }
	}
    }
    Err(Empty)
}


#[derive(Debug)]
struct Override { // TODO: more debug info
    pub flag: String,
    pub priority: i32,
}

pub fn watch_files(file_names: Vec<&str>) {

    // find all overrides in files
    let mut overrides: Vec<Override> = Vec::new();
    let mut finals:    Vec<String>   = Vec::new();
    for file_name in file_names {
	let mut file = File::open(file_name).expect(&format!("Unable to open file '{}'", file_name));
	let mut src = String::new(); 
	file.read_to_string(&mut src).expect(&format!("Unable to read file '{}'", file_name));

	for item in syn::parse_file(&src).expect(&format!("Unable to parse file '{}'", file_name)).items {
	    match item { // step over everything in the file
		syn::Item::Fn(func) => {
		    match get_priority(&func.attrs) {
			Ok(priority) =>
			    overrides.push(Override{
				flag: format!("func_{}",
					      func.sig.ident.to_string()),
				priority,
			    }),
			Err(Final) => finals.push(format!("func_{}",
							  func.sig.ident.to_string())),
			Err(Empty) => {},
		    }
		},
		syn::Item::Impl(impl_block) => {
		    match get_priority(&impl_block.attrs) {
			Ok(priority) => {
			    let self_type = match impl_block.self_ty.as_ref() { // The `Dummy` in `impl Dummy {}`
				Path(path) => path,
				_ => panic!("Could not get Path for impl (should never see this)"),
			    }.path.segments[0].ident.to_string();
			    
			    for item in impl_block.items {
				match item {
				    Method(method) =>
					overrides.push(Override{
					    flag: format!("method_{}_{}",
							  self_type,
							  &method.sig.ident),
					    priority,
					}),
				    _ => panic!("I can't overload anything other than methods in an impl block yet"),
				}
			    }
			},
			Err(Final) => {
			    panic!("Can't finalize methods yet");
			},
			Err(Empty) => {},
		    }
		},
		_ => {} // can't parse everything yet
	    }
	}
    }

    // group them into like targets
    let mut override_chains: Vec<Vec<Override>> = Vec::new();
    for overrider in overrides.into_iter() {
	if let Some(position) = override_chains.iter().position(|chain| chain[0].flag == overrider.flag) {
	    override_chains[position].push(overrider);
	} else {
	    override_chains.push(vec![overrider]);
	}
    }

    // print cfgs
    for chain in override_chains.into_iter() {
	let (i_of_max, _) = chain.iter().enumerate().max_by_key(|x| x.1.priority.abs()).unwrap();
	for fin in &finals {
	    if fin == &chain[i_of_max].flag {
		println!("cargo:rustc-env=__override_final_{}={}", fin, chain[i_of_max].priority+1);
	    }
	}
	for (i, overrider) in chain.into_iter().enumerate(){
	    if i_of_max != i {
		println!("cargo:rustc-cfg=__override_priority_{}_{}", overrider.priority, overrider.flag);
	    }
	};
    }
}
