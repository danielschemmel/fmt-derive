use syn::parenthesized;
use syn::parse::{Parse, ParseStream};

#[derive(Clone, Debug, Default)]
pub struct FieldAttribute {
	pub ignore: bool,
	pub format: Option<proc_macro2::TokenStream>,
}

impl FieldAttribute {
	pub fn update(&mut self, other: Self) {
		if other.ignore {
			self.ignore = true;
		}
		if other.format.is_some() {
			self.format = other.format;
		}
	}
}

impl Parse for FieldAttribute {
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
					if lookahead.peek(super::kw::ignore) {
						let _kw: super::kw::ignore = args.parse()?;
						result.ignore = true;
						if !args.is_empty() {
							let lookahead = args.lookahead1();
							return Err(lookahead.error());
						}
					} else if lookahead.peek(syn::LitStr) {
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
