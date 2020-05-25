#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn override_default(attr: TokenStream, input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(attr as syn::parse::Nothing); // I take no args

    // open the impl block
    let mut input: syn::ItemImpl = syn::parse(input).unwrap();

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
	    _ => panic!("TODO"),
	}
    }

    // zip back up and return
    TokenStream::from(quote! {
	#input
    })
}
