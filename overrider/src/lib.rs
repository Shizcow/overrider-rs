use syn::{parse::Nothing, ImplItem::Method, Type::Path, ItemFn, ItemImpl, DeriveInput, Ident};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

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
			    panic!("Could not parse literal");
			}
		    } else {
			panic!("Expected integer literal");
		    }
		} else {
		    panic!("Unexpected arguement name");
		}
	    } else {
		panic!("Incorrect arguement format / expected positive integer");
	    }
	} else {
	    panic!("Unexpected arguement {}", attr.to_string());
	}
    };

    attach(input, priority)
}

fn attach(input: TokenStream, priority: i32) -> TokenStream { // TODO: do this with traits
    if let Ok(item) = syn::parse::<ItemImpl>(input.clone()) {
	attach_impl(item, priority)
    } else if let Ok(item) = syn::parse::<ItemFn>(input) {
	attach_function(item, priority)
    } else {
	panic!("I can't parse this yet")
    }
}


fn attach_function(mut input: ItemFn, priority: i32) -> TokenStream {
    let override_flag = Ident::new(&format!("__override_priority_{}_func_{}", priority, &input.sig.ident), Span::call_site());
    
    input.attrs.push(
	syn::parse2::<DeriveInput>(
	    quote! {
		#[cfg(not(#override_flag))]
		struct Dummy;
	    }
	).unwrap().attrs.swap_remove(0));
    TokenStream::from(quote! {
	#input
    })
}

fn attach_impl(mut input: ItemImpl, priority: i32) -> TokenStream {
    // First, grab the struct name
    let self_type = match input.self_ty.as_ref() {
	Path(path) => path,
	_ => panic!("Could not get Path for impl (should never see this)"),
    }.path.segments[0].ident.to_string();

    // then step over each method, appending override flag to each
    for item in &mut input.items {
	match item {
	    Method(method) => {
		let override_flag = Ident::new(
		    &format!("__override_priority_{}_method_{}_{}",
			     priority,
			     self_type,
			     &method.sig.ident), Span::call_site());
		method.attrs.push(
		    syn::parse2::<DeriveInput>(
			quote! {
			    #[cfg(not(#override_flag))]
			    struct Dummy;
			}
		    ).unwrap().attrs.swap_remove(0));
	    },
	    _ => panic!("I can't overload anything other than methods in an impl block yet"),
	}
    }
    TokenStream::from(quote! {
	#input
    })
}
