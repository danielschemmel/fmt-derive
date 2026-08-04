use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod debug;
mod display;
mod generics;
mod syntax;

#[manyhow::manyhow]
#[proc_macro_derive(Debug, attributes(fmt, debug))]
pub fn debug(item: proc_macro::TokenStream) -> manyhow::Result<TokenStream2> {
	debug::debug(item, &use_rt()?)
}

#[manyhow::manyhow]
#[proc_macro_derive(Display, attributes(fmt, display))]
pub fn display(item: proc_macro::TokenStream) -> manyhow::Result<TokenStream2> {
	display::display(item, &use_rt()?)
}

#[manyhow::manyhow]
#[proc_macro_derive(Fmt, attributes(fmt, debug, display))]
pub fn fmt(item: proc_macro::TokenStream) -> manyhow::Result<TokenStream2> {
	let use_rt = use_rt()?;
	let mut stream = debug::debug(item.clone(), &use_rt)?;
	stream.extend(display::display(item, &use_rt)?);
	Ok(stream)
}

fn use_rt() -> manyhow::Result<TokenStream2> {
	match proc_macro_crate::crate_name("fmt-derive").map_err(|err| {
		manyhow::error_message!(
			"{}", err;
			help = "The `fmt-derive` crate must be used directly.";
			note = "Did you accidentally import `fmt-derive-proc` instead?"
		)
	})? {
		proc_macro_crate::FoundCrate::Itself => Ok(quote!(
			use crate::_rt;
		)),
		proc_macro_crate::FoundCrate::Name(name) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			Ok(quote!( use ::#ident::_rt; ))
		}
	}
}
