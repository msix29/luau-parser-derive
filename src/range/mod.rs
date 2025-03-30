mod enum_impl;
mod struct_impl;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data_struct) => struct_impl::generate(&input, data_struct),
        Data::Enum(data_enum) => todo!(),
        Data::Union(_) => {
            let err = syn::Error::new(
                Span::call_site(),
                "`#[Derive(Range)]` can't be called on unions.",
            )
            .to_compile_error();

            quote!(#err).into()
        }
    }
}
