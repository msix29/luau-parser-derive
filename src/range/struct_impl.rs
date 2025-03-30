use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

#[inline]
pub fn generate(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(fields) => named(input, fields),
        Fields::Unnamed(fields) => unnamed(input, fields),
        Fields::Unit => todo!(),
    }
}

fn named(input: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let first_name = fields.named.first().unwrap().ident.as_ref().unwrap();

    if fields.named.len() == 1 {
        quote! {
            impl crate::types::GetRange for #name {
                #[inline]
                fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                    self.#first_name.get_range()
                }
            }
        }
        .into()
    } else {
        let last_name = fields.named.last().unwrap().ident.as_ref().unwrap();

        quote! {
            impl crate::types::GetRange for #name {
                #[inline]
                fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                    crate::types::Range::new(
                        self.#first_name.get_range().start,
                        self.#last_name.get_range().end,
                    )
                }
            }
        }
        .into()
    }
}

fn unnamed(input: &DeriveInput, fields: &FieldsUnnamed) -> TokenStream {
    let name = &input.ident;
    let len = fields.unnamed.len();

    if len == 1 {
        quote! {
            impl crate::types::GetRange for #name {
                #[inline]
                fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                    self.0.get_range()
                }
            }
        }
        .into()
    } else {
        let last = len - 1;

        quote! {
            impl crate::types::GetRange for #name {
                #[inline]
                fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                    crate::types::Range::new(
                        self.0.get_range().start,
                        self.#last.get_range().end,
                    )
                }
            }
        }
        .into()
    }
}
