#[macro_use]
mod macros;

mod print;
mod range;

use proc_macro::TokenStream;

#[proc_macro_derive(Range, attributes(range_or))]
#[inline]
pub fn range(input: TokenStream) -> TokenStream {
    range::generate(input)
}

#[proc_macro_derive(Print)]
#[inline]
pub fn print(input: TokenStream) -> TokenStream {
    print::generate(input)
}
