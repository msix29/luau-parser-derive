use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use super::utils;

#[inline]
pub fn generate(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(fields) => named(input, fields),
        Fields::Unnamed(fields) => unnamed(input, fields),
        Fields::Unit => error!("`#[Derive(Range)]` can't be implemented on unit structs."),
    }
}

fn named(input: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let body = utils::named(fields, "self");

    quote! {
        impl crate::types::GetRange for #name {
            #[inline]
            fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                #body
            }
        }
    }
}

fn unnamed(input: &DeriveInput, fields: &FieldsUnnamed) -> TokenStream {
    let name = &input.ident;
    let body = utils::unnamed(fields, "self");

    quote! {
        impl crate::types::GetRange for #name {
            #[inline]
            fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                #body
            }
        }
    }
}
