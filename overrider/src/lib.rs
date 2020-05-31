use syn::{parse::Nothing, spanned::Spanned, ImplItem::{Method, Const}, Type::Path, ItemFn, ItemImpl,
	  DeriveInput, Ident, Attribute};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

#[proc_macro_attribute]
pub fn override_final(attr: TokenStream, input: TokenStream)-> TokenStream {
    syn::parse_macro_input!(attr as Nothing); // I take no args
    if let Ok(impl_block) = syn::parse::<ItemImpl>(input.clone()) {
	let self_type = match impl_block.self_ty.as_ref() {
	    Path(path) => path,
	    _ => return quick_error(format!("Could not get Path for impl (should never see this)")),
	}.path.segments[0].ident.to_string();
	match impl_block.items.into_iter().fold(None, |acc, item| {
	    let new_error = match item {
		Method(method) => {
		    let priority_lesser = 
			std::env::var(format!("__override_final_method_{}_{}", self_type, &method.sig.ident.to_string()))
			.expect("Failed covering final. \
				 Did you configure your build script to watch this file?");
		    syn::Error::new(
			method.sig.ident.span(),
			match priority_lesser.as_str() {
			    "0" => 
				format!("Method requested final. \
					 Replace #[override_final] with #[default] or higher \
					 on a (seperate if required) impl block to make top level."),
			    "1" => 
				format!("Method requested final. \
					 Replace #[override_final] with #[override_default] or higher  \
					 on a (seperate if required) impl block to make top level."),
			    priority_lesser => 
				format!("Method requested final. \
					 Replace #[override_final] with #[override_default(priority = {})] \
					 or higher on a (seperate if required) impl block to make top level.",
					priority_lesser),
			}
		    )
		},
		Const(constant) => {
		    let priority_lesser = 
			std::env::var(format!("__override_final_implconst_{}_{}", self_type, &constant.ident.to_string()))
			.expect("Failed covering final. \
				 Did you configure your build script to watch this file?");
		    syn::Error::new(
			constant.ident.span(),
			match priority_lesser.as_str() {
			    "0" => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with #[default] or higher  \
					 on a (seperate if required) impl block to make top level."),
			    "1" => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with #[override_default] or higher  \
					 on a (seperate if required) impl block to make top level."),
			    priority_lesser => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with #[override_default(priority = {})] \
					 or higher on a (seperate if required) impl block to make top level.",
					priority_lesser),
			}
		    )
		}
		item => syn::Error::new(item.span(),
					format!("I can't finalize this yet")),
	    };
	    match acc {
		None => Some(new_error),
		Some(mut errors) => {
		    errors.combine(new_error);
		    Some(errors)
		},
	    }
	}) {
	    Some(errors) => errors.to_compile_error().into(),
	    None => input, // this will only happen if user tries to finalize an empty impl block
	}
    } else if let Ok(item) = syn::parse::<ItemFn>(input) {
	let priority_lesser = 
	    std::env::var(format!("__override_final_func_{}", &item.sig.ident.to_string()))
	    .expect("Failed covering final. \
		     Did you configure your build script to watch this file?");
	return syn::Error::new(
	    item.sig.ident.span(),
	    match priority_lesser.as_str() {
		"0" => 
		    format!("Function requested final. \
			     Replace #[override_final] with #[default] or higher to make top level."),
		"1" => 
		    format!("Function requested final. \
			     Replace #[override_final] with #[override_default] \
			     or higher to make top level."),
		priority_lesser => 
		    format!("Function requested final. \
			     Replace #[override_final] with #[override_default(priority = {})] \
			     or higher  to make top level.",
			    priority_lesser),
	    }
	).to_compile_error().into();
    } else {
	quick_error(format!("I can't finalize whatever this is attached to yet"))
    }
}

#[proc_macro_attribute] // shorthand for #[override_default(priority = 0)]
pub fn default(attr: TokenStream, input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(attr as Nothing); // I take no args
    attach(input, 0)
}

#[proc_macro_attribute]
pub fn override_default(attr: TokenStream, input: TokenStream) -> TokenStream {
    let priority = {
	if let Ok(_) = syn::parse::<Nothing>(attr.clone()) {
	    1
	} else if let Ok(syn::Expr::Assign(assign)) = syn::parse::<syn::Expr>(attr.clone()) {
	    if let (syn::Expr::Path(left), syn::Expr::Lit(right)) = (*assign.left, *assign.right) {
		if left.path.segments[0].ident.to_string() == "priority" {
		    if let syn::Lit::Int(lit) = right.lit {
			if let Ok(i) = lit.base10_parse::<i32>() {
			    i
			} else {
			    return quick_error(format!("Could not parse literal"));
			}                                     
		    } else {
			return quick_error(format!("Expected integer literal"));
		    }
		} else {
		    return quick_error(format!("Unexpected arguement name"));
		}
	    } else {
		return quick_error(format!("Incorrect arguement format / expected positive integer"));
	    }
	} else {
	    return quick_error(format!("Unexpected arguement {} (expected priority)", attr.to_string()));
	}
    };

    attach(input, priority)
}

fn quick_error(message: String) -> TokenStream {
    syn::Error::new(
	Span::call_site(),
	message
    ).to_compile_error().into()
}

fn attach(input: TokenStream, priority: i32) -> TokenStream { // TODO: do this with traits
    if let Ok(item) = syn::parse::<ItemImpl>(input.clone()) {
	attach_impl(item, priority)
    } else if let Ok(item) = syn::parse::<ItemFn>(input) {
	attach_function(item, priority)
    } else {
	quick_error(format!("I can't parse this yet"))
    }
}

fn attach_function(mut input: ItemFn, priority: i32) -> TokenStream {
    match std::env::var(format!("__override_acceptflags_func_{}", &input.sig.ident)) {
	Err(_) => { // no flags to worry about
	    attr_add(&mut input.attrs,
		     format!("__override_priority_{}_func_{}", priority, &input.sig.ident));
	    
	    TokenStream::from(quote! {
		#input
	    })
	},
	Ok(flags) => {
	    let old_attrs = &input.attrs;
	    let old_ident = &input.sig.ident;
	    let old_sig = input.sig.clone();
	    
	    let args = input.sig.inputs.iter().map(|input| {
		match input {
		    syn::FnArg::Typed(t) => {
			match &*t.pat {
			    syn::Pat::Ident(p) => &p.ident,
			    _ => panic!("I don't know what this is"),
			}
		    },
		    _ => panic!("Can't take untyped args"), // TODO: compiler error
		}
	    }).collect::<Vec<&Ident>>();
	    
	    let if_branches = flags.split(" ").map(|flagstr| {
		let flagext = Ident::new(&format!("__override_flagext_{}_{}",
						  flagstr, old_ident),
					 Span::call_site());
		quote! {
		    if CLAP_FLAGS.occurrences_of(#flagstr) > 0 {
			#flagext (#(#args),*);
		    }
		}
	    }).collect::<Vec<proc_macro2::TokenStream>>();

	    input.sig.ident = Ident::new(&format!("__override_flagentry_{}",
						  old_ident),
					 Span::call_site());
	    let sigentry = &input.sig.ident;
	    
	    TokenStream::from(quote! {
		#(#old_attrs)*
		#old_sig {
		    #(#if_branches else )* {
			#sigentry (#(#args),*);
		    }
		}
		
		#[override_default(priority = #priority)]
		#[inline(always)]
		#input
	    })
	}
    }
}

fn attach_impl(mut input: ItemImpl, priority: i32) -> TokenStream {
    // First, grab the struct name
    let self_type = match input.self_ty.as_ref() {
	Path(path) => path,
	item => return syn::Error::new(
	    item.span(),
	    format!("Could not get Path for impl (should never see this)"))
	    .to_compile_error().into(),
    }.path.segments[0].ident.to_string();

    // then step over each method, appending override flag to each
    for item in &mut input.items {
	match item {
	    Method(method) =>
		match std::env::var(format!("__override_acceptflags_method_{}_{}", self_type, &method.sig.ident)) {
		    Err(_) => // no flags to worry about
			attr_add(&mut method.attrs, format!("__override_priority_{}_method_{}_{}",
							    priority,
							    self_type,
							    &method.sig.ident)),
		    Ok(flags) => {
			panic!("found flags!")
		    }
		},
	    Const(constant) =>
		match std::env::var(format!("__override_acceptflags_method_{}", self_type)) {
		    Err(_) => // no flags to worry about
			attr_add(&mut constant.attrs, format!("__override_priority_{}_implconst_{}_{}",
							      priority,
							      self_type,
							      &constant.ident)),
		    Ok(_) => return syn::Error::new(
			item.span(),
			format!("Laying flags on const currently envokes undefined behavior"))
			.to_compile_error().into(),
		},
	    item => return syn::Error::new(
		item.span(),
		format!("I can't overload anything other than methods/consts in an impl block yet"))
		.to_compile_error().into(),
	}
    }
    TokenStream::from(quote! {
	#input
    })
}

fn attr_add(attrs: &mut Vec<Attribute>, flag: String) {
    let override_flag = Ident::new(&flag, Span::call_site());
    attrs.push(
	syn::parse2::<DeriveInput>(
	    quote! {
		#[cfg(not(#override_flag))]
		struct Dummy;
	    }
	).unwrap().attrs.swap_remove(0));
}

fn attr_inline(attrs: &mut Vec<Attribute>) {
    attrs.push(
	syn::parse2::<DeriveInput>(
	    quote! {
		#[inline(always)]
		struct Dummy;
	    }
	).unwrap().attrs.swap_remove(0));
}

#[proc_macro_attribute]
pub fn override_flag(attr: TokenStream, input: TokenStream) -> TokenStream { // TODO: default_flag
    // parse 2 arguements (flag = x, priority = y)
    // TODO check =, +=, -= etc chars
    let (flag, priority) = 
	if let Ok(expr) = syn::parse::<syn::Expr>(attr.clone()) {
	    if let syn::Expr::Assign(assign) = expr {
		if let (syn::Expr::Path(left), syn::Expr::Path(right)) = (*assign.left, *assign.right) {
		    if left.path.segments[0].ident.to_string() == "flag" {
			(right.path.segments[0].ident.to_string(), 1)
		    } else {
			panic!("arg must be flag");
		    }
		} else {
		    panic!("Bad arg form / need flag");
		}
	    } else {
		panic!("Bad arg");
	    }
	} else { // try to parse manually
	    let attrstr = attr.to_string();
	    if attrstr.matches(",").count() == 1 {
		let args = attrstr.split(",").collect::<Vec<&str>>();
		if let (Ok(syn::Expr::Assign(arg1)), Ok(syn::Expr::Assign(arg2))) = (syn::parse_str::<syn::Expr>(args[0]), syn::parse_str::<syn::Expr>(args[1])) {
		    if let (syn::Expr::Path(left1), syn::Expr::Path(left2)) = (*arg1.left, *arg2.left) {
			let (flagarg, parg) = 
			    if left1.path.segments[0].ident.to_string() == "flag" && left2.path.segments[0].ident.to_string() == "priority" {
				((left1, *arg1.right), (left2, *arg2.right))
			    } else if left1.path.segments[0].ident.to_string() == "priority" && left2.path.segments[0].ident.to_string() == "flag" {
				((left2, *arg2.right), (left1, *arg1.right))
			    } else {
				panic!("invalid arg 3");
			    };
			if let (syn::Expr::Path(right1), syn::Expr::Lit(right2)) = (flagarg.1, parg.1) {
			    if let syn::Lit::Int(right2int) = right2.lit {
				if let Ok(right2val) = right2int.base10_parse::<i32>() { // TODO u32
				    (right1.path.segments[0].ident.to_string(), right2val)
				} else {
				    panic!("wrong type for lit");
				}
			    } else {
				panic!("invalid lit");
			    }
			} else {
			    panic!("invalid arg 1");
			}
		    } else {
			panic!("nopath");
		    }
		} else {
		    panic!("Invalid arg form");
		}
	    } else {
		panic!("Invalid arg list");
	    }
	};
    
    if let Ok(item) = syn::parse::<ItemImpl>(input.clone()) {
	flag_impl(item, priority, flag)
    } else if let Ok(item) = syn::parse::<ItemFn>(input) {
	flag_function(item, priority, flag)
    } else {
	quick_error(format!("I can't parse this yet"))
    }
}


fn flag_function(mut item: ItemFn, priority: i32, flag: String) -> TokenStream {
    attr_add(&mut item.attrs,
	     format!("__override_priority_{}_flag_{}_func_{}",
		     priority, flag, item.sig.ident));
    attr_inline(&mut item.attrs);
    //__override_priority_1_flag_change_func_main
    item.sig.ident = Ident::new(&format!("__override_flagext_{}_{}",
					 flag, item.sig.ident),
				Span::call_site());
    return TokenStream::from(quote! {
	#item
    });
}

fn flag_impl(mut item: ItemImpl, priority: i32, flag: String) -> TokenStream {
    for item in &mut item.items {
	match item {
	    Method(method) => {
		attr_add(&mut method.attrs,
			 format!("__override_priority_{}_flag_{}_method_{}",
				 priority, flag, method.sig.ident));
		attr_inline(&mut method.attrs);
		method.sig.ident = Ident::new(&format!("__override_flagext_{}_{}",
						       flag, method.sig.ident),
					      Span::call_site());
	    },
	    Const(_constant) => {
		panic!("flagging a constant currently envokes undefined behavior");
		/*
		attr_add(&mut constant.attrs,
		format!("__override_priority_{}_flag_{}_implconst_{}",
		priority, flag, &constant.ident.to_string()));
		constant.ident = Ident::new(&format!("__override_flagext_{}_{}",
		flag, &constant.ident.to_string()),
		Span::call_site());*/
	    },
	    item => return syn::Error::new(
		item.span(),
		format!("I can't overload anything other than methods/consts in an impl block yet"))
		.to_compile_error().into(),
	}
    }
    TokenStream::from(quote! {
	#item
    })
}
