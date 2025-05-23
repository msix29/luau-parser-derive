//! `#[Derive(Range)]` implementation for structs.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed};

use super::utils::get_fallback;

/// An error saying that this enum variant must have at least one item.
macro_rules! must_have_one_item {
    () => {
        return error!("Enum variants passed to `#[Derive(Range)]` must have at least one item.")
    };
}

/// Generate the code for an enum.
#[inline]
pub fn generate(data: &DataEnum) -> TokenStream {
    let mut match_arms = Vec::new();

    for variant in data.variants.iter() {
        let name = &variant.ident;
        let arm = match &variant.fields {
            Fields::Named(fields) => named(name, fields),
            Fields::Unnamed(fields) => unnamed(name, fields),
            Fields::Unit => {
                quote! { Self::#name => Err(crate::types::GetRangeError::ErrorVariant), }
            }
        };

        match_arms.push(arm);
    }

    quote! {
        match self {
            #(#match_arms)*
        }
    }
}

/// Generate the code which accounts for the fallback field.
fn generate_for_fallback(name: &Ident, fallback: &Option<Ident>) -> TokenStream {
    if fallback.is_some() {
        quote! {{
            if let Some(item) = #name {
                item.get_range()
            } else {
                #fallback.get_range()
            }
        }}
    } else {
        quote! { #name.get_range() }
    }
}

/// Generate the code for a named enum variant.
fn named(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let Some(first) = fields.named.first() else {
        must_have_one_item!();
    };
    let Some(first_name) = first.ident.as_ref() else {
        must_have_one_item!();
    };
    let (found, first_fallback) = get_fallback(first, first_name);
    if found && first_fallback.is_none() {
        return error!("`range_or` field must be a string literal.");
    }

    let first_body = generate_for_fallback(first_name, &first_fallback);

    let mut referenced_fields = first_fallback.map_or_else(
        || vec![first_name.clone()],
        |first_fallback| vec![first_name.clone(), first_fallback],
    );

    if fields.named.len() == 1 {
        quote! {
            Self::#name { #(#referenced_fields,)* .. } => #first_body,
        }
    } else {
        let Some(last) = fields.named.last() else {
            must_have_one_item!();
        };
        let Some(last_name) = last.ident.as_ref() else {
            must_have_one_item!();
        };
        let (found, last_fallback) = get_fallback(last, last_name);
        if found && last_fallback.is_none() {
            return error!("`range_or` field must be a string literal.");
        }

        let last_body = generate_for_fallback(last_name, &last_fallback);

        referenced_fields.push(last_name.clone());

        if Some(first_name) != last_fallback.as_ref() {
            if let Some(last_fallback) = last_fallback {
                referenced_fields.push(last_fallback);
            }
        }

        quote! {
            Self::#name { #(#referenced_fields,)* .. } => Ok(lsp_types::Range::new(
                #first_body?.start,
                #last_body?.end,
            )),
        }
    }
}

/// Generate the code for an unnamed enum variant.
fn unnamed(name: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    if fields.unnamed.len() == 1 {
        quote! {
            Self::#name(item) => item.get_range(),
        }
    } else {
        quote! {
            Self::#name(item1, .., item2) => lsp_types::Range::new(
                item1.get_range()?.start,
                item2.get_range()?.end,
            ),
        }
    }
}
