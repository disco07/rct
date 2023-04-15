mod fields;

extern crate proc_macro;

use fields::Data;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToTable, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = Data::new(input);
    let struct_name = &data.struct_name;
    let fields = data.get_field();

    let mut fields_name = vec![];
    let mut fields_rows = vec![];

    for field in fields {
        let name = field.name.unwrap();
        fields_name.push(quote!(#name.cell()));

        let ident = field.ident;
        let color = field.color;
        let bg = field.bg;
        let font = field.font;
        let span = field.span.unwrap();

        let mut row = quote_spanned! {span=>
            field. #ident.cell()
        }; 

        if let Some(color) = color {
            row = quote_spanned! {span=>
                #row .color(#color)
            };
        }

        if let Some(bg) = bg {
            row = quote_spanned! {span=>
                #row .bg(#bg)
            };
        }

        if let Some(font) = font {
            row = quote_spanned! {span=>
                #row .font(#font)
            };
        }

        fields_rows.push(row);
    }

    let expanded = quote! {
        use rct::ICell;

        pub trait ITable {
            fn to_table(self) -> rct::Table;
        }

        //#[automatically_derived]
        //impl ITable for #struct_name {
        //    fn to_table(self) -> rct::Table {
        //        let mut table = rct::Table::new();
        //        let header = ::std::vec![#(#fields_name,)*];
        //        let rows = ::std::vec![#(#fields_rows.clone(),)*];
        //        table
        //            .add_header(header)
        //            .add_row(rows);
        //        table
        //    }
        //}

        #[automatically_derived]
        impl<T> ITable for T
        where
            T: IntoIterator<Item = #struct_name>,
        {
            fn to_table(self) -> rct::Table {
                let mut table = rct::Table::new();
                let header = ::std::vec![#(#fields_name,)*];
                table.add_header(header);
                for field in self {
                    let rows = ::std::vec![#(#fields_rows,)*];
                    table.add_row(rows);
                }

                table
            }
        }
    };

    expanded.into()
}
