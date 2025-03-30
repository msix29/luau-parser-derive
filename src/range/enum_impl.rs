use proc_macro::TokenStream;
use syn::{DataEnum, DeriveInput};

#[inline]
pub fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    todo!()
}
