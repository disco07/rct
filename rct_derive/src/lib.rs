extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToTable, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // eprintln!("{:#?}", input);
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(field_named) => field_named.named,
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    let field_names = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let name = name.to_string();
            quote! {
                #name.cell()
            }
        })
        .collect::<Vec<_>>();

    let field = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            quote! {
                self.#name.cell()
            }
        })
        .collect::<Vec<_>>();

    let field_all = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            quote! {
                field.#name.cell()
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        use rct::ICell;

        pub trait ITable {
            fn to_table(self) -> rct::Table;
        }

        #[automatically_derived]
        impl ITable for #name {
            fn to_table(self) -> rct::Table {
                let mut table = rct::Table::new();
                let header = ::std::vec![#(#field_names.clone(),)*];
                let rows = ::std::vec![#(#field.clone(),)*];
                table
                    .add_header(header)
                    .add_row(rows);
                table
            }
        }

        #[automatically_derived]
        impl<T> ITable for T
        where
            T: Iterator<Item = #name>,
        {
            fn to_table(self) -> rct::Table {
                let mut table = rct::Table::new();
                let header = ::std::vec![#(#field_names,)*];
                table.add_header(header);
                for field in self {
                    let rows = ::std::vec![#(#field_all,)*];
                    table.add_row(rows);
                }

                table
            }
        }
    };

    expanded.into()
}
