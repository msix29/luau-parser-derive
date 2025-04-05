use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use super::{utils, CodeData};

macro_rules! must_have_one_item {
    () => {
        "Structs passed to `#[Derive(Print)]` must have at least one item."
    };
}

enum Data {
    CodeData(CodeData),
    MatchArm(TokenStream),
}

#[inline]
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

    let mut print_arms = Vec::new();

    for (arm, (data, is_named)) in match_arms_data {
        let print_body = match data {
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

                let print_body = utils::generate_print(&data);

                quote! { #start => { #print_body } }
            }
            Data::MatchArm(body) => body,
        };

        print_arms.push(print_body);
    }

    quote! {
        impl #generics crate::types::Print for #name #generics {
            #[inline]
            fn print(&self) -> String {
                match self {
                    #(#print_arms)*
                }
            }
        }
    }
}

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
