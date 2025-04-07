//! The `#[Derive(Range)]` macro.

mod enum_impl;
mod struct_impl;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Generate code for the `#[Derive(Range)]` macro.
pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let body = match input.data {
        Data::Struct(ref data_struct) => struct_impl::generate(data_struct),
        Data::Enum(ref data_enum) => enum_impl::generate(data_enum),
        Data::Union(_) => error!("`#[Derive(Range)]` can't be called on unions."),
    };

    quote! {
        impl #generics crate::types::GetRange for #name #generics {
            #[inline]
            fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                #body
            }
        }
    }
    .into()
}
