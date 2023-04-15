use proc_macro2::{TokenStream, Span};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{LitStr, Expr, Field as SynField, Data as SynData, Result, Ident, Attribute, Meta, Token, ExprLit, Lit, Index, Fields, DeriveInput};
use syn::punctuated::Punctuated;

pub struct Data {
    pub struct_name: Ident,
    fields: Vec<SynField>,
}

pub struct Field {
    pub ident: TokenStream,
    pub name: Option<LitStr>,
    pub font: Option<Expr>,
    pub color: Option<LitStr>,
    pub bg: Option<LitStr>,
    pub span: Option<Span>
}

impl Data {
    pub fn new(input: DeriveInput) -> Self {
        let struct_name = input.ident;
        let fields = match input.data {
            SynData::Struct(s) => match s.fields {
                Fields::Named(field_named) => field_named.named,
                Fields::Unnamed(_) => unimplemented!(),
                Fields::Unit => unimplemented!(),
            },
            SynData::Enum(_) => unimplemented!(),
            SynData::Union(_) => unimplemented!(),
        };

        let fields = fields.into_iter().map(|f|f).collect::<Vec<SynField>>();

        Self {
            fields,
            struct_name,
        }
    }

    pub fn get_field(&self) -> Vec<Field> {
        self.fields.iter().enumerate().map(|(index, f)| {
            Field::new(f, index).unwrap().unwrap()
        }).collect()
    }
}

impl Field {
    pub fn new(f: &SynField, index: usize) -> Result<Option<Self>> {
        let ident = f
            .ident
            .as_ref()
            .map(ToTokens::into_token_stream)
            .unwrap_or_else(|| Index::from(index).into_token_stream());
        let span = f.span();

        let mut field = get_fields(f)?;
        field.ident = ident;
        field.span = Some(span);

        Ok(Some(field))
    }
}

fn get_attrs<'a>(field: &'a SynField, attrs: &str) -> Option<&'a Attribute> {
    field.attrs.iter().find(|a| a.path().is_ident(attrs))
}

fn get_fields(f: &SynField) -> Result<Field> {
    let mut field = Field {
        ident: Default::default(),
        name: None,
        font: None,
        color: None,
        bg: None,
        span: None,
    };
    if let Some(attr) = get_attrs(f, "table") {
        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested {
            match meta {
                Meta::NameValue(meta) => match meta.path.get_ident() {
                    Some(ident) if ident == "rename" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            field.name = Some(match lit {
                                Lit::Str(lit_str) => {
                                    lit_str
                                },
                                err => {
                                    return Err(syn::Error::new_spanned(
                                        err,
                                        "Invalid value for #[table(rename = \"value\")]",
                                    ))
                                }
                            });
                        }
                    }
                    Some(ident) if ident == "color" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            field.color = Some(match lit {
                                Lit::Str(lit_str) => {
                                    lit_str
                                },
                                err => {
                                    return Err(syn::Error::new_spanned(
                                        err,
                                        "Invalid value for #[table(color = \"value\")]",
                                    ))
                                }
                            });
                        }
                    }
                    Some(ident) if ident == "bg" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = meta.value {
                            field.bg = Some(match lit {
                                Lit::Str(lit_str) => {
                                    lit_str
                                },
                                err => {
                                    return Err(syn::Error::new_spanned(
                                        err,
                                        "Invalid value for #[table(bg = \"value\")]",
                                    ))
                                }
                            });
                        }
                    }
                    Some(ident) if ident == "font" => {
                        if let Expr::Lit(ExprLit { lit, .. }) = &meta.value {
                            field.font = Some(match lit {
                                Lit::Str(lit_str) => {
                                    lit_str.parse::<Expr>()
                                }
                                err => {
                                    return Err(syn::Error::new_spanned(
                                        err,
                                        "Invalid value for #[table(font = \"value\")]",
                                    ))
                                }
                            }?)
                        }
                    }
                    _ => return Err(syn::Error::new_spanned(
                        meta,
                        "Attributes should be of type: #[table(key = \"value\", ..)]",
                    ))
                },
                _ => return Err(syn::Error::new_spanned(
                    meta,
                    "Attributes should be of type: #[table(key = \"value\", ..)]",
                ))
            };
        }
    }

    Ok(field)
}