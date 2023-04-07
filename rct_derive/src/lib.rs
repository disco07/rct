extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Table, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    eprintln!("{:#?}", input);
    let fields = match input.data {
        Data::Struct(s) => {
            match s.fields {
                Fields::Named(field_named) => {
                    field_named.named
                }
                Fields::Unnamed(_) => unimplemented!(),
                Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!()
    };

    TokenStream::new()
}
