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
		let mut result = Self::default();
		let lookahead = input.lookahead1();

		if !input.is_empty() {
			if lookahead.peek(syn::LitStr) {
				result.format = Some(input.parse()?);
			} else if lookahead.peek(super::kw::ignore) {
				let _kw: super::kw::ignore = input.parse()?;
				result.ignore = true;
				if !input.is_empty() {
					let lookahead = input.lookahead1();
					return Err(lookahead.error());
				}
			} else {
				return Err(lookahead.error());
			}
		}

		Ok(result)
	}
}
