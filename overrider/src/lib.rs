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
			format!("Method requested final. \
				 Replace #[override_final] with #[override(priority = {})] \
				 on a (seperate if required) impl block to make top level.",
				priority_lesser))
		},
		Const(constant) => {
		    let priority_lesser = 
			std::env::var(format!("__override_final_implconst_{}_{}", self_type, &constant.ident.to_string()))
			.expect("Failed covering final. \
				 Did you configure your build script to watch this file?");
		    syn::Error::new(
			constant.ident.span(),
			format!("Impl constant requested final. \
				 Replace #[override_final] with #[override(priority = {})] \
				 on a (seperate if required) impl block to make top level.",
				priority_lesser))
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
	    format!("Function requested final. \
		     Replace #[override_final] with #[override(priority = {})] to make top level.",
		    priority_lesser)
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
    attr_flag(&mut input.attrs,
	      format!("__override_priority_{}_func_{}", priority, &input.sig.ident));
    
    TokenStream::from(quote! {
	#input
    })
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
		attr_flag(&mut method.attrs, format!("__override_priority_{}_method_{}_{}",
			     priority,
			     self_type,
						     &method.sig.ident)),
	    Const(constant) =>
		attr_flag(&mut constant.attrs, format!("__override_priority_{}_implconst_{}_{}",
						       priority,
						       self_type,
						       &constant.ident)),
	    item => return syn::Error::new(
		item.span(),
		format!("I can't overload anything other than methods in an impl block yet"))
		.to_compile_error().into(),
	}
    }
    TokenStream::from(quote! {
	#input
    })
}

fn attr_flag(attrs: &mut Vec<Attribute>, flag: String) {
    let override_flag = Ident::new(&flag, Span::call_site());
    attrs.push(
	syn::parse2::<DeriveInput>(
	    quote! {
		#[cfg(not(#override_flag))]
		struct Dummy;
	    }
	).unwrap().attrs.swap_remove(0));
}
