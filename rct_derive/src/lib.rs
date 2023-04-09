extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ToTable, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // eprintln!("{:#?}", input);
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

    let field_names = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let name = name.to_string();
        quote! {
            #name.cell()
        }
    });

    let field = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            self.#name.cell()
        }
    });

    let expanded = quote! {
        use rct::ICell;
        #[automatically_derived]
        impl rct::ITable for #name {
            fn to_table(self) -> rct::Table {
                let mut table = rct::Table::new();
                let header = ::std::vec![#(#field_names,)*];
                let rows = ::std::vec![#(#field,)*];
                table
                    .add_header(header)
                    .add_row(rows);
                table
            }
        }
    };

    expanded.into()
}
