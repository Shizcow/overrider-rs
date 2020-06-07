//! `overrider` is a crate for making dynamic compilation easier.
//! 
//! ## About
//! `overrider` aims to bring the `override` keyword of some other programming languages,
//! such as Java and Python, to Rust. With this crate, a base implimentation of a function,
//! method, or other item can be defined and then later overriden. All of this happens at
//! compilation time.  
//! 
//! `overrider` also allows for defining `flags`. By parsing flags with `clap` in a
//! `lazy_static`, highly efficient switching of functionality due to input flags
//! can be achieved.
//! 
//! 
//! ## Quick Example
//! The following code shows how `overrider` is used in `src` file:
//! ```
//! use overrider::*;
//! 
//! #[default]
//! fn main() {
//!     println!("This is the base implimentation");
//! }
//! 
//! #[override_default]
//! fn main() {
//!     println!("This is the overriden implimentation");
//! }
//! ```
//! Easy as that. If the second implimentation is included, the output changes.  
//! 
//! How about with flags?
//! ```
//! use overrider::*;
//! use clap::{Arg, ArgMatches, App};
//! use lazy_static::lazy_static;
//! 
//! lazy_static! {
//!     static ref CLAP_FLAGS: ArgMatches<'static> = {
//! 	App::new("Overrider example - flag")
//!             .arg(Arg::with_name("change").long("change"))
//!             .get_matches()
//!     };
//! }
//! 
//! #[default]
//! fn main() {
//!     println!("Nothing to see here");
//! }
//!
//! #[override_flag(flag = change)]
//! fn main() {
//!     println!("I'll be printed if you call this program with --change");
//! }
//! ```
//! 
//! ## Why not traits?
//! Rust has a powerful trait system which allows somewhat similar functionality.
//! However, it does not allow multiple, concurrent definitions without conflict.  
//! Additionally, traits do not have built in support for flags.
//!
//! 
//! ## Impl
//! Due to limitations of `proc_macro`, all `overrider` flags __must__ be attached to
//! the ouside of an `impl` block, __not__ the inside.  
//! The following is correct:
//! ```
//! #[default]
//! impl Foo {
//!     fn bar(){}
//! }
//! ```
//! The following is __not__ correct;
//! ```
//! impl Foo {
//!     #[default]
//!     fn bar(){}
//! }
//! ```
//! Currently, `overrider` allows for the following items inside an `impl` block to
//! be manipulated:
//! - `fn` (methods)
//! - `const`ants
//! 
//! 
//! ## Building
//! Because of limitations in `proc_macro`, `overrider` **will not work** without
//! it's sister crate `overrider_build`. This is because `overrider_build` parses
//! Rust files, supplying the `rustc` configuration flags nessicary for conditional
//! compilation. For the above files, placing this code in `build.rs` will do the trick:
//! ```
//! fn main() {
//!     overrider_build::watch_files(vec!["src/main.rs"]);
//! }
//! ```
//! For more information, see the `overrider_build` documentation.
//! 
//! 
//! ## More examples
//! Additional examples, verified to work, can be seen
//! [online](https://github.com/Shizcow/overrider-rs/tree/master/examples).  
//! Try cloning the repository and running examples with `cargo run -p EXAMPLE_NAME`


use syn::{parse::Nothing, spanned::Spanned, ImplItem::{Method, Const}, Type::Path,
	  ItemFn, ItemImpl, DeriveInput, Ident, Attribute};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

/// Throw a compiler error to help ensure this item gets compiled
///
/// Because `overrider` is all about modifying functionality, **this will not compile**.
/// Instead, A compiler error will emit the priority required to ensure this item is
/// included in the final compilation.
///
/// ## Syntax
/// Simply add `#[override_final]` to a `fn` or `impl` block.
#[proc_macro_attribute]
pub fn override_final(attr: TokenStream, input: TokenStream)-> TokenStream {
    syn::parse_macro_input!(attr as Nothing); // I take no args
    if let Ok(impl_block) = syn::parse::<ItemImpl>(input.clone()) {
	let self_type = match impl_block.self_ty.as_ref() { // TODO function
	    Path(path) => path,
	    _ => return quick_error(format!("Could not get Path for impl \
					     (should never see this)")),
	}.path.segments[0].ident.to_string();
	match impl_block.items.into_iter().fold(None, |acc, item| {
	    let new_error = match item {
		Method(method) => {
		    let priority_lesser = 
			std::env::var(format!("__override_final_method_{}_{}", self_type,
					      &method.sig.ident.to_string()))
			.expect("Failed covering final. \
				 Did you configure your build script to watch this file?");
		    syn::Error::new(
			method.sig.ident.span(),
			match priority_lesser.as_str() {
			    "0" => 
				format!("Method requested final. \
					 Replace #[override_final] with #[default] or higher \
					 on a (seperate if required) impl block to make top \
					 level."),
			    "1" => 
				format!("Method requested final. \
					 Replace #[override_final] with #[override_default] \
					 or higher on a (seperate if required) impl block to \
make top level."),
			    priority_lesser => 
				format!("Method requested final. \
					 Replace #[override_final] with \
					 #[override_default(priority = {})] or higher on a \
					 (seperate if required) impl block to make top level.",
					priority_lesser),
			}
		    )
		},
		Const(constant) => {
		    let priority_lesser = 
			std::env::var(format!("__override_final_implconst_{}_{}",
					      self_type, &constant.ident.to_string()))
			.expect("Failed covering final. \
				 Did you configure your build script to watch this file?");
		    syn::Error::new(
			constant.ident.span(),
			match priority_lesser.as_str() {
			    "0" => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with #[default] or higher \
					 on a (seperate if required) impl block to make top \
					 level."),
			    "1" => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with #[override_default] \
					 or higher on a (seperate if required) impl block \
					 to make top level."),
			    priority_lesser => 
				format!("Impl constant requested final. \
					 Replace #[override_final] with \
					 #[override_default(priority = {})] or higher on a \
					 (seperate if required) impl block to make top level.",
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
	    None => input, // will only happen if user tries to finalize an empty impl block
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
			     Replace #[override_final] with #[default] or higher \
			     to make top level."),
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

/// Marks an item as the base implimentation
///
/// Attaching this attribute to a `fn` or `impl` block enables it to be overriden.
///
/// `#[default]` is short hand for `#[override_default(priority = 0)]`
///
/// ### Syntax
/// Here's an example showing how to flag a function as default:
/// ```
/// #[default]
/// fn main() {
///     println!("Default");
/// }
/// ```
/// It's that easy.
#[proc_macro_attribute]
pub fn default(attr: TokenStream, input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(attr as Nothing); // I take no args
    attach(input, 0)
}

/// Replaces (overrides) base implimentation
///
/// Attaching this attribute to `fn` or `impl` block overrides the implimentation
/// defined with `#[default]`. `overrider` will intelligently determine which implimentation
/// should be compiled in.
///
/// `#[override_default]` accepts a single, optinal arguement: `priority`. By setting
/// the priority of a particular implimentation higher, `overrider` will prefer it over
/// any other implimentation, even other `#[override_default]` implientations, so long
/// as it holds the highest priority.
///
/// ### Syntax
/// Here's an example showing how to override a function, and then override it a second
/// time with a higher priority.
/// ```
/// #[override_default]
/// fn main() {
///     println!("I won't run");
/// }
/// 
/// #[override_default(priority = 2)]
/// fn main() {
///     println!("I will run");
/// }
/// ```
/// `priority` can be any positive integer. Having two implimentations with the same
/// priority invokes undefined behavior. To help avoid this, see
/// [`#[override_final]`](attr.override_default.html)
#[proc_macro_attribute]
pub fn override_default(attr: TokenStream, input: TokenStream) -> TokenStream {
    let priority = {
	if let Ok(_) = syn::parse::<Nothing>(attr.clone()) {
	    1
	} else if let Ok(syn::Expr::Assign(assign)) = syn::parse::<syn::Expr>(attr.clone()) {
	    if let (syn::Expr::Path(left), syn::Expr::Lit(right)) = (*assign.left, *assign.right) {
		if left.path.segments[0].ident.to_string() == "priority" {
		    if let syn::Lit::Int(lit) = right.lit {
			if let Ok(i) = lit.base10_parse::<u32>() {
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

fn attach(input: TokenStream, priority: u32) -> TokenStream { // TODO: do this with traits
    if let Ok(item) = syn::parse::<ItemImpl>(input.clone()) {
	attach_impl(item, priority)
    } else if let Ok(item) = syn::parse::<ItemFn>(input) {
	attach_function(item, priority)
    } else {
	quick_error(format!("I can't parse this yet"))
    }
}

fn attach_function(mut input: ItemFn, priority: u32) -> TokenStream {
    attr_add(&mut input.attrs,
	     format!("__override_priority_{}_func_{}", priority, &input.sig.ident));
    match std::env::var(format!("__override_acceptflags_func_{}", &input.sig.ident)) {
	Err(_) => { // no flags to worry about
	    TokenStream::from(quote! {
		#input
	    })
	},
	Ok(flags) => {
	    let old_attrs = input.attrs.clone();
	    let old_ident = &input.sig.ident;
	    let old_sig = input.sig.clone();

	    let mut args = Vec::new();
	    for input in &input.sig.inputs {
		match input {
		    syn::FnArg::Typed(t) => {
			match t.pat.as_ref() {
			    syn::Pat::Ident(p) => args.push(&p.ident),
			    _ => return syn::Error::new(
				t.span(),
				format!("I do not know what this is and \
					 it can't be overriden"))
				.to_compile_error().into(),
			}
		    },
		    arg => return syn::Error::new(
				arg.span(),
				format!("I can only override typed arguments"))
				.to_compile_error().into(),
		}
	    };
	    
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

	    attr_inline(&mut input.attrs);
	    
	    TokenStream::from(quote! {
		#(#old_attrs)*
		#old_sig {
		    #(#if_branches else )* {
			#sigentry (#(#args),*);
		    }
		}
		
		#input
	    })
	}
    }
}

fn attach_impl(mut input: ItemImpl, priority: u32) -> TokenStream {
    // First, grab the struct name
    let self_type = match input.self_ty.as_ref() {
	Path(path) => path,
	item => return syn::Error::new(
	    item.span(),
	    format!("Could not get Path for impl (should never see this)"))
	    .to_compile_error().into(),
    }.path.segments[0].ident.to_string();

    let mut additional_items: Vec::<syn::ImplItem> = Vec::new();

    // then step over each method, appending override flag to each
    for item in &mut input.items {
	match item {
	    Method(method) => {
		attr_add(&mut method.attrs, format!("__override_priority_{}_method_{}_{}",
							    priority,
						    self_type,
						    &method.sig.ident));
		if let Ok(flags) = std::env::var(format!("__override_acceptflags_method_{}_{}",
							 self_type, &method.sig.ident)) {
		    let old_attrs = method.attrs.clone();
		    let old_ident = &method.sig.ident;
		    let old_sig = method.sig.clone();

		    let mut args = Vec::new();
		    for input in &method.sig.inputs {
			match input {
			    syn::FnArg::Typed(t) => {
				match t.pat.as_ref() {
				    syn::Pat::Ident(p) => args.push(&p.ident),
				    _ => return syn::Error::new(
					t.span(),
					format!("I do not know what this is and \
						 it can't be overriden"))
				.to_compile_error().into(),
				}
			    },
			    arg => return syn::Error::new(
				arg.span(),
				format!("I can only override typed arguments"))
				.to_compile_error().into(),
			}
		    };
			
		    let if_branches = flags.split(" ").map(|flagstr| {
			    let flagext = Ident::new(&format!("__override_flagext_{}_{}",
							      flagstr, old_ident),
						     Span::call_site());
			    quote! {
				if CLAP_FLAGS.occurrences_of(#flagstr) > 0 {
				    Self::#flagext (#(#args),*);
				}
			    }
			}).collect::<Vec<proc_macro2::TokenStream>>();

			method.sig.ident = Ident::new(&format!("__override_flagentry_{}",
							       old_ident),
						      Span::call_site());
			let sigentry = &method.sig.ident;

			attr_inline(&mut method.attrs);

			additional_items.push(syn::parse2::<syn::ImplItem>(quote! {
			    #(#old_attrs)*
			    #old_sig {
				#(#if_branches else )* {
				    Self::#sigentry (#(#args),*);
				}
			    }
			}).unwrap());
		    }
		},
	    Const(constant) =>
		match std::env::var(format!("__override_acceptflags_method_{}", self_type)) {
		    Err(_) => // no flags to worry about
			attr_add(&mut constant.attrs,
				 format!("__override_priority_{}_implconst_{}_{}",
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
		format!("I can't overload anything other than methods/consts \
			 in an impl block yet"))
		.to_compile_error().into(),
	}
    }
    input.items.append(&mut additional_items);
    
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

/// Override a base implimentation, but only when runtime is called with certain flags
///
/// Attaching this attribute to a `fn` or `impl` block enables it to be overriden at runtime
/// depending on what flags are passed to the executable. Flags require `lazy_static` and
/// `clap` to work properly. The expected format is as follows:
/// ```
/// lazy_static! {
///     static ref CLAP_FLAGS: ArgMatches<'static> = {
/// 	App::new("Overrider example - flag")
///             .arg(Arg::with_name("change").long("change"))
///             .get_matches()
///     };
/// }
/// ```
/// `overrider` expects this format. `CLAP_FLAGS` must be a `clap::ArgMatches` object,
/// and it must be done through `lazy_static`. This object must be referenced consistantly,
/// so if work is spread across multiple files, the single `CLAP_FLAGS` instant must be
/// imported. `overrider` expects `CLAP_FLAGS` in the local namespace.
/// It's a bit annoying, but it offers unparalleled performance and ease of use
/// after the setup stage.
///
/// ## Syntax
/// After the `CLAP_FLAGS` definition mentioned above, the `#[override_flag]` attribute can
/// be attached to an item. **A `#[default]` implimentation is required**. This is so the item
/// is not left undefined if no flags are passed.
///
/// `override_flag` takes two arguements:
/// - flag
/// - priority  
/// Priority allows for overriding a previous flag definition. The full syntax is as follows:  
/// `#[override_flag(flag = FLAGNAME, priority = n)]`, where `FLAGNAME` is a UTF8 string
/// containing no spaces, and `n` is a positive integer.
#[proc_macro_attribute]
pub fn override_flag(attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse 2 arguements (flag = x, priority = y)
    // TODO check =, +=, -= etc chars
    let (flag, priority) = 
	if let Ok(expr) = syn::parse::<syn::Expr>(attr.clone()) {
	    if let syn::Expr::Assign(assign) = expr {
		if let (syn::Expr::Path(left), syn::Expr::Path(right)) = (*assign.left, *assign.right) {
		    if left.path.segments[0].ident.to_string() == "flag" {
			(right.path.segments[0].ident.to_string(), 1)
		    } else {
			return quick_error(format!("mandatory arguement 'flag' ommited"));
		    }
		} else {
		    return quick_error(format!("Bad arguement form: left side must be \
						'flag', right side must be an identifier"));
		}
	    } else {
		return quick_error(format!("Malformed arguement"));
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
				return quick_error(format!("Invalid arguement list"));
			    };
			if let (syn::Expr::Path(right1), syn::Expr::Lit(right2)) = (flagarg.1, parg.1) {
			    if let syn::Lit::Int(right2int) = right2.lit {
				if let Ok(right2val) = right2int.base10_parse::<u32>() { // TODO u32
				    (right1.path.segments[0].ident.to_string(), right2val)
				} else {
				    return quick_error(format!("Priority must be a positive \
								integer"));
				}
			    } else {
				return quick_error(format!("Priority must be a positive \
							    integer"));
			    }
			} else {
			    return quick_error(format!("Flag must be an identifier"));
			}
		    } else {
			return quick_error(format!("Flag requires an identifier"));
		    }
		} else {
		    return quick_error(format!("Malformed arguement"));
		}
	    } else {
		return quick_error(format!("Incomplete/badly malformed arguement \
					    or too many arguements"));
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


fn flag_function(mut item: ItemFn, priority: u32, flag: String) -> TokenStream {
    attr_add(&mut item.attrs,
	     format!("__override_priority_{}_flag_{}_func_{}",
		     priority, flag, item.sig.ident));
    attr_inline(&mut item.attrs);
    item.sig.ident = Ident::new(&format!("__override_flagext_{}_{}",
					 flag, item.sig.ident),
				Span::call_site());
    return TokenStream::from(quote! {
	#item
    });
}

fn flag_impl(mut impl_block: ItemImpl, priority: u32, flag: String) -> TokenStream {
    let self_type = match impl_block.self_ty.as_ref() {
	Path(path) => path,
	_ => return quick_error(format!("Could not get Path for impl (should never see this)")),
    }.path.segments[0].ident.to_string();
    for item in &mut impl_block.items {
	match item {
	    Method(method) => {
		attr_add(&mut method.attrs,
			 format!("__override_priority_{}_flag_{}_method_{}_{}",
				 priority, flag, self_type, method.sig.ident));
		attr_inline(&mut method.attrs);
		method.sig.ident = Ident::new(&format!("__override_flagext_{}_{}",
						       flag, method.sig.ident),
					      Span::call_site());
	    },
	    Const(_constant) => {
		return quick_error(format!("flagging a constant currently envokes undefined \
					    behavior"));
	    },
	    item => return syn::Error::new(
		item.span(),
		format!("I can't overload anything other than methods/consts in an impl \
			 block yet"))
		.to_compile_error().into(),
	}
    }
    TokenStream::from(quote! {
	#impl_block
    })
}
