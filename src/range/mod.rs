mod enum_impl;
mod struct_impl;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data_struct) => struct_impl::generate(&input, data_struct),
        Data::Enum(ref data_enum) => enum_impl::generate(&input, data_enum),
        Data::Union(_) => error!("`#[Derive(Range)]` can't be called on unions."),
    }
    .into()
}
