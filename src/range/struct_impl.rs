use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed};

use super::utils::get_fallback;

macro_rules! must_have_one_item {
    () => {
        return error!("Structs passed to `#[Derive(Range)]` must have at least one item.")
    };
}

#[inline]
pub fn generate(data: &DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(fields) => named(fields),
        Fields::Unnamed(fields) => unnamed(fields),
        Fields::Unit => error!("`#[Derive(Range)]` can't be implemented on unit structs."),
    }
}

fn generate_for_fallback(name: &Ident, fallback: Option<Ident>) -> TokenStream {
    if fallback.is_some() {
        quote! {{
            if let Some(item) = &self.#name {
                item.get_range()
            } else {
                self.#fallback.get_range()
            }
        }}
    } else {
        quote! { self.#name.get_range() }
    }
}

pub fn named(fields: &FieldsNamed) -> TokenStream {
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

    let first_body = generate_for_fallback(first_name, first_fallback);

    if fields.named.len() == 1 {
        first_body
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

        let last_body = generate_for_fallback(last_name, last_fallback);

        quote! {
            Ok(crate::types::Range::new(
                #first_body?.start,
                #last_body?.end,
            ))
        }
    }
}

pub fn unnamed(fields: &FieldsUnnamed) -> TokenStream {
    let len = fields.unnamed.len();

    if len == 1 {
        quote! { self.0.get_range() }
    } else {
        let last = len - 1;

        quote! {
            Ok(crate::types::Range::new(
                self.0.get_range()?.start,
                self.#last.get_range()?.end,
            ))
        }
    }
}
