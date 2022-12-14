use proc_macro_error::{abort_if_dirty, emit_error};
use quote::quote;
use syn::parse_macro_input;

mod field_attribute;
mod item_attribute;
mod variant_attribute;

pub fn display(item: proc_macro::TokenStream, use_rt: &proc_macro2::TokenStream) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as syn::DeriveInput);
	let item_name = &item.ident;
	let generics_params = &item.generics.params;
	let generics_params_bare = generics_params
		.pairs()
		.map(|param| {
			let comma = param.punct();
			match param.value() {
				syn::GenericParam::Type(ty) => {
					let id = &ty.ident;
					quote!(#id #comma)
				}
				syn::GenericParam::Lifetime(lifetime) => {
					let id = &lifetime.lifetime;
					quote!(#id #comma)
				}
				syn::GenericParam::Const(val) => {
					let id = &val.ident;
					quote!(#id #comma)
				}
			}
		})
		.collect::<proc_macro2::TokenStream>();
	let generics_where = &item.generics.where_clause;

	let mut item_config = item_attribute::ItemAttribute::default();
	for attribute in &item.attrs {
		if attribute.path.is_ident("fmt") || attribute.path.is_ident("display") {
			match syn::parse2(attribute.tokens.clone()) {
				Ok(value) => item_config.update(value),
				Err(err) => emit_error!(err),
			}
		}
	}
	abort_if_dirty();

	let display = match item_config.format {
		Some(format) => {
			quote!(::core::write!(f, #format))
		}
		None => match item.data {
			syn::Data::Struct(item_struct) => {
				let item_name_str = item_name.to_string();
				match item_struct.fields {
					syn::Fields::Unit => process_unit(&item_name_str),
					syn::Fields::Unnamed(fields) => {
						let (destructure, implementation) = process_tuple(&item_name_str, &fields);
						quote!(let #item_name #destructure = self; #implementation)
					}
					syn::Fields::Named(fields) => {
						let (destructure, implementation) = process_struct(&item_name_str, &fields);
						quote!(let #item_name #destructure = self; #implementation)
					}
				}
			}
			syn::Data::Union(_) => {
				let name = format!("<{}>", item.ident);
				quote!(::core::write!(f, #name))
			}
			syn::Data::Enum(item_enum) => {
				if item_enum.variants.is_empty() {
					quote!(::core::unreachable!())
				} else {
					let mut stream = proc_macro2::TokenStream::new();

					for variant in item_enum.variants {
						let variant_name = variant.ident;
						let mut variant_config = variant_attribute::VariantAttribute::default();
						for attribute in &variant.attrs {
							if attribute.path.is_ident("fmt") || attribute.path.is_ident("display") {
								match syn::parse2(attribute.tokens.clone()) {
									Ok(value) => variant_config.update(value),
									Err(err) => emit_error!(err),
								}
							}
						}
						if let Some(format) = variant_config.format {
							match variant.fields {
								syn::Fields::Unit => {
									stream.extend(quote!(Self::#variant_name => { ::core::write!(f, #format) }));
								}
								syn::Fields::Unnamed(fields) => {
									let mut destructure = quote!();
									for (field_number, _field) in fields.unnamed.iter().enumerate() {
										let var_name = proc_macro2::Ident::new(&format!("t{}", field_number), proc_macro2::Span::call_site());
										destructure.extend(quote!(#var_name, ))
									}
									stream.extend(quote!(Self::#variant_name(#destructure) => { ::core::write!(f, #format) }));
								}
								syn::Fields::Named(fields) => {
									let mut destructure = quote!();
									for field in fields.named {
										let var_name = field.ident.unwrap();
										destructure.extend(quote!(#var_name, ))
									}
									stream.extend(quote!(Self::#variant_name{#destructure} => { ::core::write!(f, #format) }));
								}
							}
						} else {
							let variant_name_str = variant_name.to_string();

							match variant.fields {
								syn::Fields::Unit => {
									let implementation = process_unit(&variant_name_str);
									stream.extend(quote!(Self::#variant_name => { #implementation }));
								}
								syn::Fields::Unnamed(fields) => {
									let (destructure, implementation) = process_tuple(&variant_name_str, &fields);
									stream.extend(quote!(Self::#variant_name #destructure => { #implementation }));
								}
								syn::Fields::Named(fields) => {
									let (destructure, implementation) = process_struct(&variant_name_str, &fields);
									stream.extend(quote!(Self::#variant_name #destructure => { #implementation }));
								}
							}
						}
					}

					quote!(match self { #stream })
				}
			}
		},
	};

	let result = quote!(
		impl<#generics_params> ::core::fmt::Display for #item_name<#generics_params_bare> #generics_where {
			fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
				#use_rt
				#display
			}
		}
	)
	.into();
	// println!("{}", result);

	abort_if_dirty();
	result
}

fn process_unit(name: &str) -> proc_macro2::TokenStream {
	quote!(f.debug_struct(#name).finish())
}

fn process_tuple(name: &str, fields: &syn::FieldsUnnamed) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
	let mut destructure = proc_macro2::TokenStream::new();
	let mut chain = quote!(use _rt::Replacement; let mut w = f.debug_tuple(#name););

	for (field_number, field) in fields.unnamed.iter().enumerate() {
		let mut config = field_attribute::FieldAttribute::default();
		for attribute in &field.attrs {
			if attribute.path.is_ident("fmt") || attribute.path.is_ident("display") {
				match syn::parse2(attribute.tokens.clone()) {
					Ok(value) => config.update(value),
					Err(err) => emit_error!(err),
				}
			}
		}

		if config.ignore {
			destructure.extend(quote!(_,));
		} else if let Some(format) = config.format {
			destructure.extend(quote!(_,));

			chain.extend(quote!(w.field(&_rt::DebugDisplay(&::core::format_args!(#format)));));
		} else {
			let var_name = proc_macro2::Ident::new(&format!("x{}", field_number), proc_macro2::Span::call_site());
			destructure.extend(quote!(#var_name,));

			let field_type = &field.ty;
			let opaque = opaque_object_string(field_type);
			chain.extend(quote!(_rt::DisplayOrReplacement::<#field_type>(&#var_name).tuple_field(#opaque, &mut w);));
		}
	}

	(quote!((#destructure)), quote!(#chain w.finish()))
}

fn process_struct(name: &str, fields: &syn::FieldsNamed) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
	let mut destructure = proc_macro2::TokenStream::new();
	let mut chain = quote!(use _rt::Replacement; let mut w = f.debug_struct(#name););

	for field in &fields.named {
		let mut config = field_attribute::FieldAttribute::default();
		for attribute in &field.attrs {
			if attribute.path.is_ident("fmt") || attribute.path.is_ident("display") {
				match syn::parse2(attribute.tokens.clone()) {
					Ok(value) => config.update(value),
					Err(err) => emit_error!(err),
				}
			}
		}

		if config.ignore {
			let field_name = field.ident.as_ref().expect("a named field should always have a name");
			destructure.extend(quote!(#field_name: _,));
		} else if let Some(format) = config.format {
			let field_name = field.ident.as_ref().expect("a named field should always have a name");
			destructure.extend(quote!(#field_name: _,));

			let field_name_str = field_name.to_string();
			chain.extend(quote!(w.field(#field_name_str, &_rt::DebugDisplay(&::core::format_args!(#format)));));
		} else {
			let field_name = field.ident.as_ref().expect("a named field should always have a name");
			destructure.extend(quote!(#field_name,));

			let field_name_str = field_name.to_string();
			let field_type = &field.ty;
			let opaque = opaque_object_string(field_type);
			chain.extend(
				quote!(_rt::DisplayOrReplacement::<#field_type>(&#field_name).struct_field(#field_name_str, #opaque, &mut w);),
			);
		}
	}

	(quote!({#destructure}), quote!(#chain w.finish()))
}

fn opaque_object_string(ty: &syn::Type) -> String {
	format!("<{}>", quote!(#ty))
}
