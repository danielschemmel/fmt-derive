use proc_macro_error::proc_macro_error;
use quote::quote;

mod debug;
mod display;

#[proc_macro_error]
#[proc_macro_derive(Debug, attributes(fmt, debug))]
pub fn debug(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	debug::debug(item, &use_rt())
}

#[proc_macro_error]
#[proc_macro_derive(Display, attributes(fmt, display))]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	display::display(item, &use_rt())
}

#[proc_macro_error]
#[proc_macro_derive(Fmt, attributes(fmt, debug, display))]
pub fn fmt(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let use_rt = use_rt();
	let mut stream = debug::debug(item.clone(), &use_rt);
	stream.extend(display::display(item, &use_rt));
	stream
}

fn use_rt() -> proc_macro2::TokenStream {
	match proc_macro_crate::crate_name("fmt-derive").unwrap_or_else(|err| {
		proc_macro_error::abort_call_site!(
			"{}", err;
			help = "The `fmt-derive` must be used directly.";
			note = "Did you accidentally import `fmt-derive-proc` instead?"
		)
	}) {
		proc_macro_crate::FoundCrate::Itself => quote!(
			use crate::_rt;
		),
		proc_macro_crate::FoundCrate::Name(name) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			quote!( use ::#ident::_rt; )
		}
	}
}
