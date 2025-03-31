use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Expr, Field, FieldsNamed, FieldsUnnamed, Lit, Meta};

macro_rules! must_have_one_item {
    () => {
        return error!("Structs passed to `#[Derive(Range)]` must have at least one item.")
    };
}

fn get_fallback(field: &Field, name: &Ident) -> (bool, Option<Ident>) {
    let mut fallback_field = None;
    let mut found_attribute = false;

    for attr in &field.attrs {
        if attr.path().is_ident("range_or") {
            found_attribute = true;

            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(literal) = &meta.value {
                    if let Lit::Str(lit_str) = &literal.lit {
                        fallback_field = Some(syn::Ident::new(&lit_str.value(), name.span()));
                    }
                }
            }
        }
    }

    (found_attribute, fallback_field)
}

fn generate_for_fallback(
    indexer_name: &Ident,
    name: &Ident,
    fallback: Option<Ident>,
) -> TokenStream {
    if fallback.is_some() {
        quote! {{
            if let Some(item) = &#indexer_name.#name {
                item.get_range()
            } else {
                #indexer_name.#fallback.get_range()
            }
        }}
    } else {
        quote! { #indexer_name.#name.get_range() }
    }
}

pub fn named(fields: &FieldsNamed, indexer_name: &str) -> TokenStream {
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

    let indexer_name = Ident::new(indexer_name, Span::call_site());
    let first_body = generate_for_fallback(&indexer_name, first_name, first_fallback);

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

        let last_body = generate_for_fallback(&indexer_name, last_name, last_fallback);

        quote! {
            Ok(crate::types::Range::new(
                #first_body?.start,
                #last_body?.end,
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
