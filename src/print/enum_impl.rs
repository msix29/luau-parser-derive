//! `#[Derive(Print)]` implementation for enums.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use super::{utils, CodeData};

/// An error saying that this enum variant must have at least one item.
macro_rules! must_have_one_item {
    () => {
        "Enum variants passed to `#[Derive(Print)]` must have at least one item."
    };
}

/// A private enum for source generation
enum Data {
    /// Name of available fields.
    CodeData(CodeData),

    /// A match arm (used for unit fields).
    MatchArm(TokenStream),
}

/// Generate the code for an enum.
pub fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;

    let mut match_arms_data = HashMap::new();

    for variant in data.variants.iter() {
        let name = &variant.ident;
        let mut is_named = false;
        let arm = match &variant.fields {
            Fields::Named(fields) => {
                is_named = true;
                named(fields)
            }
            Fields::Unnamed(fields) => unnamed(fields),
            Fields::Unit => Data::MatchArm(
                // quote! { Self::#name => Err(crate::types::PrintError::ErrorVariant), },
                quote! { Self::#name => "".to_string(), },
            ),
        };

        match_arms_data.insert(name, (arm, is_named));
    }

    let mut arms = Vec::new();
    let mut last_trivia_arms = Vec::new();

    for (arm, (data, is_named)) in match_arms_data {
        let (body, final_trivia) = match data {
            Data::CodeData(data) => {
                let mut all_names = data.middle.clone();
                all_names.push(data.first.clone());
                if let Some(last) = data.last.clone() {
                    all_names.push(last);
                }

                let start = if is_named {
                    quote! {Self::#arm { #(#all_names,)* }}
                } else {
                    quote! {Self::#arm( #(#all_names,)* )}
                };

                let print_without_final_trivia_body =
                    utils::generate_print_without_final_trivia(&data);
                let print_final_trivia_body = utils::generate_print_final_trivia(&data);

                (
                    quote! { #start => { #print_without_final_trivia_body } },
                    quote! { #start => { #print_final_trivia_body } },
                )
            }
            Data::MatchArm(body) => (body.clone(), body),
        };

        arms.push(body);
        last_trivia_arms.push(final_trivia);
    }

    quote! {
        impl #generics crate::types::Print for #name #generics {
            #[inline]
            #[allow(unused)]
            fn print_final_trivia(&self) -> String {
                match self {
                    #(#last_trivia_arms)*
                }
            }

            #[inline]
            #[allow(unused)]
            fn print_without_final_trivia(&self) -> String {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

/// Get the data for a named enum variant.
fn named(fields: &FieldsNamed) -> Data {
    Data::CodeData(utils::generate(
        fields
            .named
            .iter()
            .map(|field| {
                let name = field.ident.as_ref().unwrap();
                quote! { #name }
            })
            .collect(),
    ))
}

/// Get the data for an unnamed enum variant.
fn unnamed(fields: &FieldsUnnamed) -> Data {
    if fields.unnamed.is_empty() {
        return Data::CodeData(CodeData::error(must_have_one_item!()));
    }

    Data::CodeData(utils::generate(
        (0..fields.unnamed.len())
            .map(|i| {
                let ident = Ident::new(&format!("item{i}"), Span::call_site());

                quote! { #ident }
            })
            .collect(),
    ))
}
