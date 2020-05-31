use syn::{Type::Path, ImplItem::{Method, Const}};
use std::fs::File;
use std::io::Read;

enum Status {Norm(i32), Flag(String, i32), Final, Empty}
use Status::*;
fn get_priority(attrs: &Vec<syn::Attribute>) -> Status {
    for attr in attrs { // there's no error checking; overrider main can give richer error messages
	if attr.path.segments[0].ident.to_string() == "override_default" {
	    if let Ok(syn::Expr::Paren(expr)) = syn::parse2::<syn::Expr>(attr.tokens.clone()) {
		if let syn::Expr::Assign(assign) = *expr.expr {
		    if let syn::Expr::Path(left) = *assign.left {
			if left.path.segments.len() == 1
			    && left.path.segments[0].ident.to_string() == "priority" {
				if let syn::Expr::Lit(lit) = *assign.right {
				    if let syn::Lit::Int(i) = lit.lit {
					if let Ok(priority) = i.base10_parse::<i32>() {
					    return Norm(priority);
					}
				    }
				}
			    }
		    }
		}
	    } else { // might be default
		if attr.tokens.is_empty() {
		    return Norm(1);
		}
	    }
	} else if attr.path.segments[0].ident.to_string() == "override_flag" {
	    if !attr.tokens.is_empty() { // TODDO;
		match syn::parse2::<syn::Expr>(attr.tokens.clone()) {
		    Ok(syn::Expr::Paren(expr)) => 
			if let syn::Expr::Assign(assign) = *expr.expr {
			    if let (syn::Expr::Path(left), syn::Expr::Path(right)) = (*assign.left, *assign.right) {
				if left.path.segments[0].ident.to_string() == "flag" {
				    return Flag(right.path.segments[0].ident.to_string(), 1);
				}
			    }
			},
		    Ok(syn::Expr::Tuple(tuple)) => {
			let mut elems = tuple.elems.into_iter();
			if let (Some(syn::Expr::Assign(assign1)), Some(syn::Expr::Assign(assign2))) = (elems.next(), elems.next())
			{
			    if let (syn::Expr::Path(left1), syn::Expr::Path(left2)) = (*assign1.left, *assign2.left) {
				let (flagarg, parg) = 
				    if left1.path.segments[0].ident.to_string() == "flag" && left2.path.segments[0].ident.to_string() == "priority" {
					((left1, *assign1.right), (left2, *assign2.right))
				    } else if left1.path.segments[0].ident.to_string() == "priority" && left2.path.segments[0].ident.to_string() == "flag" {
					((left2, *assign2.right), (left1, *assign1.right))
				    } else {
					return Empty;
				    };
				if let (syn::Expr::Path(right1), syn::Expr::Lit(right2)) = (flagarg.1, parg.1) {
				    if let syn::Lit::Int(right2int) = right2.lit {
					if let Ok(right2val) = right2int.base10_parse::<i32>() { // TODO u32
					    return Flag(right1.path.segments[0].ident.to_string(), right2val);
					}
				    }
				}
			    }
			}
		    },
		    _ => {},
		}
	    }
	} else if attr.path.segments[0].ident.to_string() == "default" {
	    if attr.tokens.is_empty() {
		return Norm(0);
	    }
	} else if attr.path.segments[0].ident.to_string() == "override_final" {
	    if attr.tokens.is_empty() {
		return Final;
	    }
	}
    }
    Empty
}


#[derive(Debug)]
struct Override {
    pub sig: String,
    pub priority: i32,
}

#[derive(Debug)]
struct Flagger {
    pub sig: String,
    pub flag: String,
    pub priority: i32,
}

pub fn watch_files(file_names: Vec<&str>) {

    // find all overrides in files
    let mut overrides: Vec<Override> = Vec::new();
    let mut finals:    Vec<String>   = Vec::new();
    let mut flags:     Vec<Flagger>  = Vec::new();
    for file_name in file_names {
	let mut file = File::open(file_name).expect(&format!("Unable to open file '{}'", file_name));
	let mut src = String::new(); 
	file.read_to_string(&mut src).expect(&format!("Unable to read file '{}'", file_name));

	let parsed = match syn::parse_file(&src) {
	    Ok(items) => items,
	    Err(_) => return, // There's a compiler error. Let rustc take care of it
	};
	
	for item in parsed.items {
	    match item { // step over everything in the file
		syn::Item::Fn(func) => {
		    match get_priority(&func.attrs) {
			Norm(priority) =>
			    overrides.push(Override{
				sig: format!("func_{}",func.sig.ident),
				priority,
			    }),
			Flag(flag, priority) => {// TODO: add meta-overload of flags vie priorities
			    flags.push(Flagger{
				sig: format!("func_{}",func.sig.ident),
				flag,
				priority,
			    })}
			,
			Final => finals.push(format!("func_{}", func.sig.ident)),
			Empty => {},
		    }
		},
		syn::Item::Impl(impl_block) => {
		    match get_priority(&impl_block.attrs) {
			Flag(flag, priority) => {
			    let self_type = match impl_block.self_ty.as_ref() { // The `Dummy` in `impl Dummy {}`
				Path(path) => path,
				_ => continue,
			    }.path.segments[0].ident.to_string();
			    
			    for item in impl_block.items {
				match item {
				    Method(method) =>
					flags.push(Flagger{
					    sig: format!("method_{}_{}",
							  self_type,
							 &method.sig.ident),
					    flag: flag.clone(),
					    priority,
					}),
				    Const(constant) =>
					flags.push(Flagger{
					    sig: format!("implconst_{}_{}",
							  self_type,
							 &constant.ident),
					    flag: flag.clone(),
					    priority,
					}),
				    _ => continue,
				}
			    }
			},
			Norm(priority) => {
			    let self_type = match impl_block.self_ty.as_ref() { // The `Dummy` in `impl Dummy {}`
				Path(path) => path,
				_ => continue,
			    }.path.segments[0].ident.to_string();
			    
			    for item in impl_block.items {
				match item {
				    Method(method) =>
					overrides.push(Override{
					    sig: format!("method_{}_{}",
							  self_type,
							  &method.sig.ident),
					    priority,
					}),
				    Const(constant) =>
					overrides.push(Override{
					    sig: format!("implconst_{}_{}",
							  self_type,
							  &constant.ident),
					    priority,
					}),
				    _ => continue,
				}
			    }
			},
			Final => {
			    let self_type = match impl_block.self_ty.as_ref() { // The `Dummy` in `impl Dummy {}`
				Path(path) => path,
				_ => continue,
			    }.path.segments[0].ident.to_string();
			    
			    for item in impl_block.items {
				match item {
				    Method(method) => 
					finals.push(format!("method_{}_{}",
							    self_type,
							    &method.sig.ident)),
				    Const(constant) =>
					finals.push(format!("implconst_{}_{}",
							    self_type,
							    &constant.ident)),
				    _ => continue,
				}
			    }
			},
			Empty => {},
		    }
		},
		_ => {} // can't parse everything yet
	    }
	}
    }

    // group them into like targets
    let mut override_chains: Vec<Vec<Override>> = Vec::new();
    // [[for each priority] for each item]
    for overrider in overrides.into_iter() {
	if let Some(position) = override_chains.iter().position(|chain| chain[0].sig == overrider.sig) {
	    override_chains[position].push(overrider);
	} else {
	    override_chains.push(vec![overrider]);
	}
    }

    // print cfgs
    for chain in override_chains.iter() {
	let (i_of_max, _) = chain.iter().enumerate().max_by_key(|x| x.1.priority.abs()).unwrap();
	for fin in &finals {
	    if fin == &chain[i_of_max].sig {
		println!("cargo:rustc-env=__override_final_{}={}", fin, chain[i_of_max].priority+1);
	    }
	}
	for (i, overrider) in chain.iter().enumerate(){
	    if i_of_max != i {
		println!("cargo:rustc-cfg=__override_priority_{}_{}", overrider.priority, overrider.sig);
	    }
	};
    }
    
    // sometimes there's something in fin that's not in override_chains. If so, priority = 0
    for fin in finals.into_iter() {
	if !override_chains.iter().any(|chain| chain[0].sig == fin) {
	    println!("cargo:rustc-env=__override_final_{}={}", fin, 0);
	}
    }

    // now for flags. This will look familiar
    let mut flag_chains: Vec<Vec<Vec<Flagger>>> = Vec::new();
    // [[[for each priority] for each --flag] for each item]
    for flag in flags {
	if let Some(item_found) = flag_chains.iter().position(|chain| chain[0][0].sig == flag.sig) {
	    if let Some(flag_found) = flag_chains[item_found].iter().position(|flag_pack| flag_pack[0].flag == flag.flag) {
		flag_chains[item_found][flag_found].push(flag);
	    } else {
		flag_chains[item_found].push(vec![flag]);
	    }
	} else {
	    flag_chains.push(vec![vec![flag]]);
	}
    }
    
    for flag_chain in flag_chains.into_iter() {
	let cargoflag = format!("__override_acceptflags_{}", flag_chain[0][0].sig);
	let item_flags = flag_chain.iter().map(|e| e[0].flag.clone())
	    .collect::<Vec<String>>().join(" "); // TODO: error check for spaces in flag
	println!("cargo:rustc-env={}={}", cargoflag, item_flags);
	
	for flag in flag_chain.into_iter() { // TODO: combine with iter above
	    let (i_of_max, _) = flag.iter().enumerate()
		.max_by_key(|x| x.1.priority.abs()).unwrap();
	    for (i, p) in flag.into_iter().enumerate() {
		if i_of_max != i {
		    println!("cargo:rustc-cfg=__override_priority_{}_flag_{}_{}", p.priority, p.flag, p.sig);
		}
	    }
	}
    }
}
