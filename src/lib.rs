use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::Span;

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
    for item in &mut input.items {
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
