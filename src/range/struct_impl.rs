use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

macro_rules! must_have_one_item {
    () => {
        return error!("Structs passed to `#[Derive(Range)]` must have at least one item.")
    };
}

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
    let Some(first) = fields.named.first() else {
        must_have_one_item!();
    };
    let Some(first_name) = first.ident.as_ref() else {
        must_have_one_item!();
    };

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
        let Some(last) = fields.named.last() else {
            must_have_one_item!();
        };
        let Some(last_name) = last.ident.as_ref() else {
            must_have_one_item!();
        };

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
