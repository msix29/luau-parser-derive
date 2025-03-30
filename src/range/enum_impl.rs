use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

#[inline]
pub fn generate(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let mut match_arms = Vec::new();
    let name = &input.ident;

    for variant in data.variants.iter() {
        let name = &variant.ident;
        let arm = match &variant.fields {
            Fields::Named(fields) => named(name, fields),
            Fields::Unnamed(fields) => unnamed(name, fields),
            Fields::Unit => {
                // return error!("`#[Derive(Range)]` can't be implemented on unit variants.")
                quote! { Self::#name => Err(crate::types::GetRangeError::ErrorVariant) }
            }
        };

        match_arms.push(arm);
    }

    quote! {
        impl crate::types::GetRange for #name {
            #[inline]
            fn get_range(&self) -> Result<crate::types::Range, crate::types::GetRangeError> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
}

fn named(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let Some(first) = fields.named.first() else {
        must_have_one_item!();
    };
    let Some(first_name) = first.ident.as_ref() else {
        must_have_one_item!();
    };

    if fields.named.len() == 1 {
        quote! {
            Self::#name { #first_name, .. } => #first_name.get_range(),
        }
    } else {
        let Some(last) = fields.named.last() else {
            must_have_one_item!();
        };
        let Some(last_name) = last.ident.as_ref() else {
            must_have_one_item!();
        };

        quote! {
            Self::#name{ #first_name, #last_name, .. } => crate::types::Range::new(
                #first_name.get_range().start,
                #last_name.get_range().end,
            ),
        }
    }
}

fn unnamed(name: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    if fields.unnamed.len() == 1 {
        quote! {
            Self::#name(item) => item.get_range(),
        }
    } else {
        quote! {
            Self::#name(item1, .., item2) => crate::types::Range::new(
                item1.get_range().start,
                item2.get_range().end,
            ),
        }
    }
}
