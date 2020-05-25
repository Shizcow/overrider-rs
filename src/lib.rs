#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};

fn override_function(mut input: syn::ItemFn) -> TokenStream {
    // build.rs does all the rest
    TokenStream::from(quote! {
	#input
    })
}

fn override_method(mut input: syn::ItemImpl) -> TokenStream {
    // step over the methods
    for mut item in &mut input.items {
	match item {
	    syn::ImplItem::Method(method) => {
		// edit the method names
		method.sig.ident = 
		    Ident::new(&format!("__{}_default", &method.sig.ident), Span::call_site());
		// and add the #[inline] attribute
		method.attrs.push(
		    syn::parse2::<syn::DeriveInput>(
			quote! {
			    #[inline]
			    struct Dummy;
			}
		    ).unwrap().attrs.swap_remove(0));
	    },
	    _ => panic!("I can't overload anything other than methods yet"),
	}
    }
    TokenStream::from(quote! {
	#input
    })
}

#[proc_macro_attribute]
pub fn override_default(attr: TokenStream, input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(attr as syn::parse::Nothing); // I take no args

    if let Ok(item) = syn::parse::<syn::ItemImpl>(input.clone()) {
	override_method(item)
    } else if let Ok(item) = syn::parse::<syn::ItemFn>(input) {
	override_function(item)
    } else {
	panic!("I can't parse this yet");
    }
}



fn default_function(mut input: syn::ItemFn) -> TokenStream {
    let override_flag = syn::LitStr::new(&format!("func_{}", &input.sig.ident), Span::call_site());
    
    input.attrs.push(
	syn::parse2::<syn::DeriveInput>(
	    quote! {
		#[cfg(not(overriden = #override_flag))]
		struct Dummy;
	    }
	).unwrap().attrs.swap_remove(0));
    TokenStream::from(quote! {
	#input
    })
}

#[proc_macro_attribute]
pub fn default(attr: TokenStream, input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(attr as syn::parse::Nothing); // I take no args

    if let Ok(item) = syn::parse::<syn::ItemImpl>(input.clone()) {
	panic!("TODO"); //default_method(item)
    } else if let Ok(item) = syn::parse::<syn::ItemFn>(input) {
	default_function(item)
    } else {
	panic!("I can't parse this yet");
    }
}

