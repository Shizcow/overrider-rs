#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn override_default(attr: TokenStream, input: TokenStream) -> TokenStream {
    
    let input: syn::ItemImpl = syn::parse(input).unwrap();

    let struct_name = match input.self_ty.as_ref() {
	syn::Type::Path(impl_path) => &impl_path.path.segments[0].ident,
	_ => panic!("TODO"),
    };

    let method_name = match &input.items[0] {
	syn::ImplItem::Method(method) => Ident::new(&format!("__{}_default", &method.sig.ident), Span::call_site()),
	_ => panic!("TODO"),
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
	impl #struct_name {
	    fn #method_name (&self) -> &'static str {
		"overriden default"
	    }
	}
    };


    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
