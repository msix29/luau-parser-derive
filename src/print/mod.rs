//! The `#[Derive(Print)]` macro.

mod enum_impl;
mod struct_impl;
mod utils;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput};

/// Code data for each struct/enum variant.
struct CodeData<A = proc_macro2::TokenStream, B = A, C = A>
where
    A: ToTokens,
    B: ToTokens,
    C: ToTokens,
{
    /// The very first name.
    pub first: A,

    /// Any names excluding the first and last.
    pub middle: Vec<B>,

    /// The last name, which may or may not exist.
    pub last: Option<C>,
}

impl<A, B, C> CodeData<A, B, C>
where
    A: ToTokens + From<proc_macro2::TokenStream>,
    B: ToTokens,
    C: ToTokens,
{
    /// Create data which sends an error.
    fn error(message: &str) -> Self {
        Self {
            first: error!(message),
            middle: Vec::new(),
            last: None,
        }
    }
}

/// Generate code for the `#[Derive(Print)]` macro.
pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data_struct) => struct_impl::generate(&input, data_struct),
        Data::Enum(ref data_enum) => enum_impl::generate(&input, data_enum),
        Data::Union(_) => error!("`#[Derive(Print)]` can't be called on unions."),
    }
    .into()
}
