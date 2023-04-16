//! Derive macro for `rct` crate.
//!
//!
//! Please refer to <https://docs.rs/rct/> for how to set this up.

mod fields;
mod table;
mod utils;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToTable, attributes(table))]
pub fn to_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    table::to_table(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
