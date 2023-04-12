extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Error, Field, Fields, Ident, Meta, parse_macro_input, Token};
use syn::punctuated::Punctuated;

#[proc_macro_derive(ToTable, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    eprintln!("{:#?}", input);
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
            T: IntoIterator<Item = #name>,
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

fn get_attrs<'a>(field: &'a Field, attrs: &str) -> Option<&'a Attribute> {
    field.attrs.iter().find(|a| { a.path().is_ident(attrs) })
}


fn error_attr<T: ToTokens>(tokens: &T) -> proc_macro2::TokenStream {
    Error::new_spanned(tokens, "expected `table(bound = \"...\", ..)`").to_compile_error()
}

fn create_method(field: &Field, i: &Ident) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let Some(attr) = get_attrs(field, "table") {
        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested {
            let meta_list = match meta {
                Meta::List(meta_list) => Ok(meta_list),
                err => Err(Error::new_spanned(err, "unrecognized table")),
            };
            for nested_meta in meta_list.unwrap().parse_nested_meta().nested.into_iter() {}
        }
    }

    Ok(proc_macro2::TokenStream::new())
}
