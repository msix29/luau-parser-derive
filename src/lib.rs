#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]
#![warn(clippy::missing_const_for_fn)]
#![deny(unused_must_use)]
#![deny(dead_code)]
#![deny(unused_assignments)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::too_many_arguments)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]

#[macro_use]
mod macros;

mod print;
mod range;

use proc_macro::TokenStream;

/// The `#[Derive(Range)]` macro.
#[proc_macro_derive(Range, attributes(range_or))]
#[inline]
pub fn range(input: TokenStream) -> TokenStream {
    range::generate(input)
}

/// The `#[Derive(Print)]` macro.
#[proc_macro_derive(Print)]
#[inline]
pub fn print(input: TokenStream) -> TokenStream {
    print::generate(input)
}
