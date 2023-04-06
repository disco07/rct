use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Table, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // TokenStream::new()
}