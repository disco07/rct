extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Field, Fields, Ident, Lit,
    Meta, Token,
};

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
            let method = extend_method(f);
            quote! {
                self.#name.cell() #method
            }
        })
        .collect::<Vec<_>>();

    let field_all = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let method = extend_method(f);
            quote! {
                field.#name.cell() #method
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        use rct::ICell;
        use rct::styles::color::{Colorizer, Font};

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
    field.attrs.iter().find(|a| a.path().is_ident(attrs))
}

// fn error_attr<T: ToTokens>(tokens: &T) -> proc_macro2::TokenStream {
//     Error::new_spanned(tokens, "expected `table(bound = \"...\", ..)`").to_compile_error()
// }

fn extend_method(field: &Field) -> proc_macro2::TokenStream {
    let mut table = proc_macro2::TokenStream::new();
    if let Some(attr) = get_attrs(field, "table") {
        let nested = attr
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .expect("error parsing ici");
        for meta in nested {
            match meta {
                Meta::NameValue(meta) => match meta.path.get_ident() {
                    Some(ident) if ident == "rename" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            if let Lit::Str(li) = lit {
                                let s = li.value();
                                table.extend(quote! {});
                            }
                        }
                    }
                    Some(ident) if ident == "color" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            if let Lit::Str(li) = lit {
                                let s = li.token();
                                table.extend(quote! {.color(#s)});
                            }
                        }
                    }
                    Some(ident) if ident == "bg" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            if let Lit::Str(li) = lit {
                                let s = li.value();
                                table.extend(quote! {.bg(#s)});
                            }
                        }
                    }
                    Some(ident) if ident == "font" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            if let Lit::Str(li) = lit {
                                let s = li.value();
                                let ident = Ident::new(&s, proc_macro2::Span::call_site());
                                table.extend(quote! {.font(#ident)});
                            }
                        }
                    }
                    _ => {
                        unimplemented!();
                    }
                },
                _ => {
                    unimplemented!();
                }
            };
        }
    }

    table
}
