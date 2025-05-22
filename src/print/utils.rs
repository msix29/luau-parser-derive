//! Helpful functions for the `#[Derive(Print)]` macro.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use super::CodeData;

/// Get the data for the passed names.
pub fn generate<T: ToTokens + Clone>(names: &[T]) -> CodeData<T> {
    assert!(!names.is_empty());

    if names.len() == 1 {
        CodeData {
            first: names[0].clone(),
            middle: Vec::new(),
            last: None,
        }
    } else if names.len() == 2 {
        CodeData {
            first: names[0].clone(),
            middle: Vec::new(),
            last: Some(names[1].clone()),
        }
    } else {
        let first = names[0].clone();
        let middle = names[1..names.len() - 1].to_vec();
        let last = Some(names[names.len() - 1].clone());

        CodeData {
            first,
            middle,
            last,
        }
    }
}

/// Generate the `print_without_final_trivia` function depending on the passed data.
pub fn generate_print_without_final_trivia<A, B, C>(data: &CodeData<A, B, C>) -> TokenStream
where
    A: ToTokens,
    B: ToTokens,
    C: ToTokens,
{
    let first = &data.first;
    let middle = &data.middle;
    let last = &data.last;

    let last_operation = last.as_ref().map_or_else(
        || quote! {},
        |last| quote! { string.push_str(&#last.print_without_final_trivia()); },
    );

    quote! {
        let mut string = #first.print_without_final_trivia();
        #(string.push_str(&#middle.print_without_final_trivia());)*
        #last_operation

        string
    }
}

/// Generate the `print_final_trivia` function depending on the passed data.
#[inline]
pub fn generate_print_final_trivia<A: ToTokens>(data: &CodeData<A>) -> TokenStream {
    let before_last = data.middle.last().unwrap_or(&data.first);

    data.last.as_ref().map_or_else(
        || quote! { #before_last.print_final_trivia() },
        |last| {
            quote! {
                let maybe_final_trivia = #last.print_final_trivia();
                if maybe_final_trivia.is_empty() {
                    #before_last.print_final_trivia()
                } else {
                    maybe_final_trivia
                }
            }
        },
    )
}
