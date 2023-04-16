use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_quote, DeriveInput, GenericParam, Generics};

use crate::{fields::Data, utils::new_generic};

pub fn to_table(input: DeriveInput) -> syn::Result<TokenStream> {
    let data = Data::new(&input)?;
    let struct_name = data.struct_name;
    let fields = data.get_field();

    // Add a bound `T: ::std::fmt::Display` to every type parameter T.
    let (generics, new_generic) = new_generic(input.to_owned().generics);
    let generics = add_trait_bounds(generics, &new_generic, struct_name);
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

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

        pub trait Tabler {
            fn to_table(self) -> rct::Table;
        }

        #[automatically_derived]
        impl #impl_generics Tabler for #new_generic #where_clause
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

    Ok(expanded)
}

// Add a bound `T: ::std::fmt::Display` to every type parameter T.
fn add_trait_bounds(
    mut generics: Generics,
    new_generic: &syn::Ident,
    struct_name: &syn::Ident,
) -> Generics {
    let mut type_params = generics
        .to_owned()
        .params
        .into_iter()
        .filter_map(|param| {
            if let GenericParam::Type(syn::TypeParam { ident, .. }) = param {
                Some(quote!(#ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    type_params.pop();

    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            // Do not add bound for new generic types
            if *new_generic == type_param.ident {
                type_param
                    .bounds
                    .push(parse_quote!(IntoIterator<Item = #struct_name<#(#type_params,)*>>));
                continue;
            }
            type_param.bounds.push(parse_quote!(::std::fmt::Display));
        }
    }

    generics
}
