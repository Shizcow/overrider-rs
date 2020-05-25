#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};


#[allow(unused)] // TODO
fn override_method_for_flags(mut input: syn::ItemImpl) -> TokenStream {
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
    input // build.rs will take care of this
}

fn default_impl(mut input: syn::ItemImpl) -> TokenStream {
    // First, grab the struct name
    let self_type = match input.self_ty.as_ref() {
	syn::Type::Path(path) => path,
	_ => panic!("Could not get Path for impl (should never see this)"),
    }.path.segments[0].ident.to_string();

    // then step over each method, appending override flag to each
    for mut item in &mut input.items {
	match item {
	    syn::ImplItem::Method(method) => {
		let override_flag = syn::Ident::new(
		    &format!("__override_method_{}_{}",
			     self_type,
			     &method.sig.ident), Span::call_site());
		method.attrs.push(
		    syn::parse2::<syn::DeriveInput>(
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

fn default_function(mut input: syn::ItemFn) -> TokenStream {
    let override_flag = proc_macro2::Ident::new(&format!("__override_func_{}", &input.sig.ident), Span::call_site());
    
    input.attrs.push(
	syn::parse2::<syn::DeriveInput>(
	    quote! {
		#[cfg(not(#override_flag))]
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
	default_impl(item)
    } else if let Ok(item) = syn::parse::<syn::ItemFn>(input) {
	default_function(item)
    } else {
	panic!("I can't parse this yet");
    }
}

fn parse_args_strings(args: TokenStream) -> Vec<String> {
    let mut strings = Vec::new();
    for (i, arg) in args.into_iter().enumerate() {
	if let proc_macro::TokenTree::Literal(lit) = arg { // is it a literal?
	    let lit_str = lit.to_string();
	    if i % 2 == 1 || // is it a _string_ literal?
	    {
		let mut chars = lit_str.chars();
		chars.next().unwrap() != '"' || chars.last().unwrap() != '"'
	    }{
		panic!("Unexpected token '{}' (expected string literal)", lit.to_string());
	    }
	    strings.push(lit_str.chars() // great, add to list
			    .skip(1).take(lit_str.len()-2)
			    .collect::<String>());
	    continue;
	} else if let proc_macro::TokenTree::Punct(punct) = arg { // is it punctuation?
	    if punct.to_string() != "," { // is it a comma? (for list)
		panic!("Unexpected punctuation {} (expected comma)", punct.to_string());
	    } else if i % 2 == 0 { // is it in the right place?
		panic!("Expected list element");
	    }
	} else {
	    panic!("Unexpected token: {}", arg.to_string());
	}
    }
    strings
}

#[proc_macro]
pub fn watch_files(args: TokenStream) -> TokenStream {
    // first, parse the arguement list
    let file_names = parse_args_strings(args);
    // Now file_names is populated with Strings. Time to read the files
    panic!("{:?}", file_names);
    TokenStream::new()
}
