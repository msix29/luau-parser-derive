use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;
use syn::{FieldsNamed, FieldsUnnamed};

macro_rules! must_have_one_item {
    () => {
        return error!("Structs passed to `#[Derive(Range)]` must have at least one item.")
    };
}

pub fn named(fields: &FieldsNamed, indexer_name: &str) -> TokenStream {
    let Some(first) = fields.named.first() else {
        must_have_one_item!();
    };
    let Some(first_name) = first.ident.as_ref() else {
        must_have_one_item!();
    };

    let indexer_name = Ident::new(indexer_name, Span::call_site());

    if fields.named.len() == 1 {
        quote! { #indexer_name.#first_name.get_range() }
    } else {
        let Some(last) = fields.named.last() else {
            must_have_one_item!();
        };
        let Some(last_name) = last.ident.as_ref() else {
            must_have_one_item!();
        };

        quote! {
            Ok(crate::types::Range::new(
                #indexer_name.#first_name.get_range()?.start,
                #indexer_name.#last_name.get_range()?.end,
            ))
        }
    }
}

pub fn unnamed(fields: &FieldsUnnamed, indexer_name: &str) -> TokenStream {
    let indexer_name = Ident::new(indexer_name, Span::call_site());
    let len = fields.unnamed.len();

    if len == 1 {
        quote! { #indexer_name.0.get_range() }
    } else {
        let last = len - 1;

        quote! {
            Ok(crate::types::Range::new(
                #indexer_name.0.get_range()?.start,
                #indexer_name.#last.get_range()?.end,
            ))
        }
    }
}
