use syn::parenthesized;
use syn::parse::{Parse, ParseStream};

#[derive(Clone, Debug)]
pub struct ItemAttribute {
	pub format: Option<proc_macro2::TokenStream>,
}

impl Default for ItemAttribute {
	fn default() -> Self {
		Self { format: None }
	}
}

impl ItemAttribute {
	pub fn update(&mut self, other: Self) {
		if other.format.is_some() {
			self.format = other.format;
		}
	}
}

impl Parse for ItemAttribute {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut result = Default::default();
		if input.is_empty() {
			Ok(result)
		} else {
			let lookahead = input.lookahead1();
			if lookahead.peek(syn::token::Paren) {
				let args;
				parenthesized!(args in input);
				let lookahead = args.lookahead1();

				if !args.is_empty() {
					if lookahead.peek(syn::LitStr) {
						result.format = Some(args.parse()?);
					} else {
						return Err(lookahead.error());
					}
				}

				Ok(result)
			} else {
				Err(lookahead.error())
			}
		}
	}
}
