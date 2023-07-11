use proc_macro_error::{abort_if_dirty, emit_error};
use quote::quote;
use syn::parse_macro_input;

use crate::generics::GenericVariants;
use crate::syntax::{field_attribute, item_attribute, variant_attribute};

pub fn display(item: proc_macro::TokenStream, use_rt: &proc_macro2::TokenStream) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as syn::DeriveInput);
	let item_name = &item.ident;

	let mut item_config = item_attribute::ItemAttribute::default();
	for attribute in &item.attrs {
		if attribute.path().is_ident("fmt") || attribute.path().is_ident("display") {
			match attribute.parse_args() {
				Ok(value) => item_config.update(value),
				Err(err) => emit_error!(attribute, err),
			}
		}
	}
	abort_if_dirty();

	let display = match item_config.format {
		Some(format) => {
			let mut result = match item.data {
				syn::Data::Struct(item_struct) => match item_struct.fields {
					syn::Fields::Unit => quote!(),
					syn::Fields::Unnamed(fields) => {
						let mut destructure = quote!();
						for (field_number, _field) in fields.unnamed.iter().enumerate() {
							let var_name = proc_macro2::Ident::new(&format!("_{}", field_number), proc_macro2::Span::call_site());
							destructure.extend(quote!(#var_name, ))
						}
						quote!(#[allow(unused_variables)] let #item_name(#destructure) = self;)
					}
					syn::Fields::Named(fields) => {
						let mut destructure = quote!();
						for field in fields.named {
							let var_name = field.ident.expect("a named field should always have a name");
							destructure.extend(quote!(#var_name, ))
						}
						quote!(#[allow(unused_variables)] let #item_name{#destructure} = self;)
					}
				},
				syn::Data::Enum(_) => quote!(),
				syn::Data::Union(_) => quote!(),
			};
			result.extend(quote!(::core::write!(fmt_derive_formatter_variable, #format)));
			result
		}
		None => match item.data {
			syn::Data::Struct(item_struct) => {
				let item_name_str = item_name.to_string();
				match item_struct.fields {
					syn::Fields::Unit => process_unit(&item_name_str),
					syn::Fields::Unnamed(fields) => {
						let (destructure, implementation) = process_tuple(&item_name_str, &fields);
						quote!(#[allow(unused_variables)] let #item_name #destructure = self; #implementation)
					}
					syn::Fields::Named(fields) => {
						let (destructure, implementation) = process_struct(&item_name_str, &fields);
						quote!(#[allow(unused_variables)] let #item_name #destructure = self; #implementation)
					}
				}
			}
			syn::Data::Union(_) => {
				let name = format!("<{}>", item.ident);
				quote!(::core::write!(fmt_derive_formatter_variable, #name))
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
							if attribute.path().is_ident("fmt") || attribute.path().is_ident("display") {
								match attribute.parse_args() {
									Ok(value) => variant_config.update(value),
									Err(err) => emit_error!(attribute, err),
								}
							}
						}
						if let Some(format) = variant_config.format {
							match variant.fields {
								syn::Fields::Unit => {
									stream
										.extend(quote!(Self::#variant_name => { ::core::write!(fmt_derive_formatter_variable, #format) }));
								}
								syn::Fields::Unnamed(fields) => {
									let mut destructure = quote!();
									for (field_number, _field) in fields.unnamed.iter().enumerate() {
										let var_name =
											proc_macro2::Ident::new(&format!("_{}", field_number), proc_macro2::Span::call_site());
										destructure.extend(quote!(#var_name, ))
									}
									stream.extend(
										quote!(Self::#variant_name(#destructure) => { ::core::write!(fmt_derive_formatter_variable, #format) }),
									);
								}
								syn::Fields::Named(fields) => {
									let mut destructure = quote!();
									for field in fields.named {
										let var_name = field.ident.expect("a named field should always have a name");
										destructure.extend(quote!(#var_name, ))
									}
									stream.extend(
										quote!(Self::#variant_name{#destructure} => { ::core::write!(fmt_derive_formatter_variable, #format) }),
									);
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

	let GenericVariants {
		params_bare: generics_params_bare,
		params_no_defaults: generics_params_no_defaults,
		where_clause: generics_where,
	} = GenericVariants::new(&item.generics);
	let result = quote!(
		impl<#generics_params_no_defaults> ::core::fmt::Display for #item_name<#generics_params_bare> #generics_where {
			fn fmt(&self, fmt_derive_formatter_variable: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
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
	quote!(fmt_derive_formatter_variable.debug_struct(#name).finish())
}

fn process_tuple(name: &str, fields: &syn::FieldsUnnamed) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
	let mut destructure = proc_macro2::TokenStream::new();
	let mut chain = quote!(use _rt::Replacement; let mut w = fmt_derive_formatter_variable.debug_tuple(#name););

	for (field_number, field) in fields.unnamed.iter().enumerate() {
		let mut config = field_attribute::FieldAttribute::default();
		for attribute in &field.attrs {
			if attribute.path().is_ident("fmt") || attribute.path().is_ident("display") {
				match attribute.parse_args() {
					Ok(value) => config.update(value),
					Err(err) => emit_error!(attribute, err),
				}
			}
		}

		let var_name = proc_macro2::Ident::new(&format!("_{}", field_number), proc_macro2::Span::call_site());
		destructure.extend(quote!(#var_name,));

		if config.ignore {
			// nop
		} else if let Some(format) = config.format {
			chain.extend(quote!(w.field(&_rt::DebugDisplay(&::core::format_args!(#format)));));
		} else {
			let field_type = &field.ty;
			let opaque = opaque_object_string(field_type);
			chain.extend(quote!(_rt::DisplayOrReplacement::<#field_type>(&#var_name).tuple_field(#opaque, &mut w);));
		}
	}

	(quote!((#destructure)), quote!(#chain w.finish()))
}

fn process_struct(name: &str, fields: &syn::FieldsNamed) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
	let mut destructure = proc_macro2::TokenStream::new();
	let mut chain = quote!(use _rt::Replacement; let mut w = fmt_derive_formatter_variable.debug_struct(#name););

	for field in &fields.named {
		let mut config = field_attribute::FieldAttribute::default();
		for attribute in &field.attrs {
			if attribute.path().is_ident("fmt") || attribute.path().is_ident("display") {
				match attribute.parse_args() {
					Ok(value) => config.update(value),
					Err(err) => emit_error!(attribute, err),
				}
			}
		}

		let field_name = field.ident.as_ref().expect("a named field should always have a name");
		destructure.extend(quote!(#field_name,));

		if config.ignore {
			// nop
		} else if let Some(format) = config.format {
			let field_name_str = field_name.to_string();
			chain.extend(quote!(w.field(#field_name_str, &_rt::DebugDisplay(&::core::format_args!(#format)));));
		} else {
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
