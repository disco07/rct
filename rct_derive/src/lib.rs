extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Error, Field, Fields, Ident, Lit, Meta,
    MetaList, MetaNameValue, Path, PathSegment,
};

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
    while let Some(attr) = field.attrs.first() {
        if let Meta::List(MetaList { path, .. }) = &attr.meta {
            if !path.is_ident(attrs) {
                return None;
            }
            return Some(attr);
        }
    }
    None
}


fn error_attr<T: ToTokens>(tokens: &T) -> proc_macro2::TokenStream {
    Error::new_spanned(tokens, "expected `table(bound = \"...\", ..)`").to_compile_error()
}

fn create_method(field: &Field, i: &Ident) -> proc_macro2::TokenStream {
    if let Some(Attribute { meta, .. }) = get_attrs(field, "table") {
        if let Meta::List(MetaList { tokens, .. }) = meta {
            if let Some(TokenTree::Ident(i)) = tokens.clone().into_iter().next() {
                if i != "each" {
                    return error_attr(meta);
                }
            }
            match tokens.clone().into_iter().nth(1).unwrap() {
                TokenTree::Punct(p) => assert_eq!(p.as_char(), '='),
                tt => panic!("expected '=' found {}", tt),
            }
            let literal = match tokens.clone().into_iter().nth(2).unwrap() {
                TokenTree::Literal(l) => Lit::new(l),
                tt => panic!("found {}", tt),
            };

            match literal {
                Lit::Str(s) => {
                    let ident = Ident::new(&s.value(), field.span());
                    if *i != ident {
                        let name = &field.ident.clone().unwrap();
                        let ty = inner_type(&field.ty, "Vec").unwrap();
                        return quote! {
                            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                                if let std::option::Option::Some(ref mut value) = self.#name {
                                    value.push(#ident);
                                } else {
                                    self.#name = std::option::Option::Some(vec![#ident]);
                                }
                                self
                            }
                        };
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    proc_macro2::TokenStream::new()
}
