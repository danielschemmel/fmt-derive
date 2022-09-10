use proc_macro_error::proc_macro_error;
use quote::quote;

mod debug;
mod display;

#[proc_macro_error]
#[proc_macro_derive(Debug, attributes(debug))]
pub fn debug(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	debug::debug(item, &lib())
}

#[proc_macro_error]
#[proc_macro_derive(Display, attributes(display))]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	display::display(item, &lib())
}

#[proc_macro_error]
#[proc_macro_derive(Fmt, attributes(debug, display))]
pub fn fmt(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let lib = lib();
	let mut stream = debug::debug(item.clone(), &lib);
	stream.extend(display::display(item, &lib));
	stream
}

fn lib() -> proc_macro2::TokenStream {
	match proc_macro_crate::crate_name("fmt-derive").unwrap_or_else(|err| {
		proc_macro_error::abort_call_site!(
			"{}",
			err; help = "Did you accidentally import `fmt-derive-proc` directly?"
		)
	}) {
		proc_macro_crate::FoundCrate::Itself => quote!(crate),
		proc_macro_crate::FoundCrate::Name(name) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			quote!( ::#ident )
		}
	}
}
