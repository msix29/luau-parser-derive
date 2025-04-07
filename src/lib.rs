//! # Luau Parser Derive
//!
//! This crate provides helpful derive procedural macros. It's only meant to be used
//! by the [luau-parser](https://github.com/msix29/luau-parser/) crate.

#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![allow(unused)]
#![warn(clippy::absolute_paths)]

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
