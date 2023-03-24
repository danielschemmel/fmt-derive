use syn::parse::{Parse, ParseStream};

#[derive(Clone, Debug, Default)]
pub struct VariantAttribute {
	pub format: Option<proc_macro2::TokenStream>,
}

impl VariantAttribute {
	pub fn update(&mut self, other: Self) {
		if other.format.is_some() {
			self.format = other.format;
		}
	}
}

impl Parse for VariantAttribute {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut result = Self::default();
		let lookahead = input.lookahead1();

		if !input.is_empty() {
			if lookahead.peek(syn::LitStr) {
				result.format = Some(input.parse()?);
			} else {
				return Err(lookahead.error());
			}
		}

		Ok(result)
	}
}
