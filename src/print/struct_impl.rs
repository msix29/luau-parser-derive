//! `#[Derive(Print)]` implementation for structs.

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use super::{utils, CodeData};

/// An error saying that this struct must have at least one item.
macro_rules! must_have_one_item {
    () => {
        "Structs passed to `#[Derive(Print)]` must have at least one item."
    };
}

/// Generate the code for a struct.
pub fn generate(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;

    let data = match &data.fields {
        Fields::Named(fields) => named(fields),
        Fields::Unnamed(fields) => unnamed(fields),
        Fields::Unit => CodeData::error("`#[Derive(Range)]` can't be implemented on unit structs."),
    };

    let print_without_final_trivia_body = utils::generate_print_without_final_trivia(&data);
    let print_final_trivia_body = utils::generate_print_final_trivia(&data);

    quote! {
        impl #generics crate::types::Print for #name #generics {
            #[inline]
            #[allow(unused)]
            fn print_final_trivia(&self) -> String {
                #print_final_trivia_body
            }

            #[inline]
            #[allow(unused)]
            fn print_without_final_trivia(&self) -> String {
                #print_without_final_trivia_body
            }
        }
    }
}

/// Get the data for a named struct.
pub fn named(fields: &FieldsNamed) -> CodeData {
    utils::generate(
        fields
            .named
            .iter()
            .map(|field| {
                let name = field.ident.as_ref().unwrap();
                quote! { self.#name }
            })
            .collect(),
    )
}

/// Get the data for an unnamed struct.
pub fn unnamed(fields: &FieldsUnnamed) -> CodeData {
    if fields.unnamed.is_empty() {
        return CodeData::error(must_have_one_item!());
    }

    utils::generate(
        (0..fields.unnamed.len())
            .map(|i| {
                let i = Literal::usize_unsuffixed(i);
                quote! { self.#i }
            })
            .collect(),
    )
}
