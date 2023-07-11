use quote::quote;

#[derive(Debug, Clone)]
pub struct GenericVariants {
	pub params_bare: proc_macro2::TokenStream,
	pub params_no_defaults: proc_macro2::TokenStream,
	pub where_clause: proc_macro2::TokenStream,
}

impl GenericVariants {
	pub fn new(generics: &syn::Generics) -> GenericVariants {
		let params_bare = generics
			.params
			.pairs()
			.map(|param| {
				let comma = param.punct();
				match param.value() {
					syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
						quote!(#ident #comma)
					}
					syn::GenericParam::Lifetime(syn::LifetimeParam { lifetime, .. }) => {
						quote!(#lifetime #comma)
					}
					syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
						quote!(#ident #comma)
					}
				}
			})
			.collect::<proc_macro2::TokenStream>();

		let params_no_defaults = generics
			.params
			.pairs()
			.map(|param| {
				let comma = param.punct();
				match param.value() {
					syn::GenericParam::Type(syn::TypeParam {
						ident,
						colon_token,
						bounds,
						..
					}) => {
						quote!(#ident #colon_token #bounds #comma)
					}
					syn::GenericParam::Lifetime(syn::LifetimeParam {
						lifetime,
						colon_token,
						bounds,
						..
					}) => {
						quote!(#lifetime #colon_token #bounds #comma)
					}
					syn::GenericParam::Const(syn::ConstParam {
						const_token,
						ident,
						colon_token,
						ty,
						..
					}) => {
						quote!(#const_token #ident #colon_token #ty #comma)
					}
				}
			})
			.collect::<proc_macro2::TokenStream>();

		let where_clause = &generics.where_clause;
		let where_clause = quote!(#where_clause);

		GenericVariants {
			params_bare,
			params_no_defaults,
			where_clause,
		}
	}
}
