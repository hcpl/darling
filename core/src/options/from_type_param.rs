use syn::{self, Ident};

use {Result};
use codegen::FromTypeParamImpl;
use options::{ParseAttribute, ParseData, OuterFrom};

#[derive(Debug)]
pub struct FromTypeParamOptions {
    pub base: OuterFrom,
    pub bounds: Option<Ident>,
    pub default: Option<Ident>,
}

impl FromTypeParamOptions {
    pub fn new(di: &syn::DeriveInput) -> Result<Self> {
        (FromTypeParamOptions {
            base: OuterFrom::start(di),
            bounds: None,
            default: None,
        }).parse_attributes(&di.attrs)?.parse_body(&di.data)
    }
}

impl ParseAttribute for FromTypeParamOptions {
    fn parse_nested(&mut self, mi: &syn::Meta) -> Result<()> {
        self.base.parse_nested(mi)
    }
}

impl ParseData for FromTypeParamOptions {
    fn parse_variant(&mut self, variant: &syn::Variant) -> Result<()> {
        self.base.parse_variant(variant)
    }

    fn parse_field(&mut self, field: &syn::Field) -> Result<()> {
        match field.ident.as_ref().map(|v| v.as_ref()) {
            Some("bounds") => { self.bounds = field.ident.clone(); Ok(()) },
            Some("default") => { self.default = field.ident.clone(); Ok(()) },
            _ => self.base.parse_field(field)
        }
    }
}

impl<'a> From<&'a FromTypeParamOptions> for FromTypeParamImpl<'a> {
    fn from(v: &'a FromTypeParamOptions) -> Self {
        FromTypeParamImpl {
            base: (&v.base.container).into(),
            ident: v.base.ident.as_ref(),
            attrs: v.base.attrs.as_ref(),
            bounds: v.bounds.as_ref(),
            default: v.default.as_ref(),
            attr_names: v.base.attr_names.as_strs(),
            forward_attrs: v.base.forward_attrs.as_ref(),
            from_ident: v.base.from_ident,
        }
    }
}