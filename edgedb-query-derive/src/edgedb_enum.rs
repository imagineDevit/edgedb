use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{ ItemEnum};
use syn::parse::{Parse, ParseStream};
use crate::constants::{INVALID_ENUM_VARIANT_TAG, VALUE};
use crate::tags::{build_tags_from_field, Tagged};
use crate::tags::TagBuilders::EnumValueBuilder;
use crate::tags::value_tag::{EnumValueTagBuilder, ValueTag};

pub struct EdgedbEnum {
    pub ident: Ident,
    pub variants: Vec<EdgedbEnumVariant>,
}

impl EdgedbEnum {
    fn new(ident: Ident) -> Self {
        Self {
            ident,
            variants: Vec::new(),
        }
    }

    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {

        let enum_name = self.ident.clone();

        let name = enum_name.to_string();

        let variants = self.variants.iter();

        let enum_vars = variants.clone().map(|v| {
            let value = v.value.clone();
            quote!(#value,)
        });

        let v_idents = variants.map(|v|{
            let value = v.value.clone();
            let enum_value = v.tag.0.clone();
            quote!(#enum_name::#value => #enum_value.to_owned())
        });

        let tokens = quote!{

            #[derive(Debug, Clone)]
            pub enum #enum_name {
                #(#enum_vars)*
            }

            impl edgedb_query::ToEdgeQl for #enum_name {
                fn to_edgeql(&self) -> String {
                    match self {
                        #(#v_idents,)*
                    }
                }
            }

            impl edgedb_query::ToEdgeScalar for #enum_name {
                fn scalar() -> String {
                    #name.to_owned()
                }
            }

             impl edgedb_query::ToEdgeValue for #enum_name {
                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                    use edgedb_query::ToEdgeQl;
                    edgedb_protocol::value::Value::Enum(edgedb_protocol::codec::EnumValue::from(self.to_edgeql().as_str()))
                }
            }

            impl ToString for #enum_name {
                fn to_string(&self) -> String {
                    use edgedb_query::ToEdgeQl;
                    self.to_edgeql()
                }
            }

        };

        Ok(tokens.into())
    }
}

impl Parse for EdgedbEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let e_num = input.parse::<ItemEnum>()?;

        let mut enum_ = Self::new(e_num.ident);

        for variant in e_num.variants {

            if variant.attrs.iter().any(|a| (a.path.segments.clone().len() != 1) || (a.path.segments[0].ident.to_string().as_str() != VALUE)) {
               return Err(syn::Error::new_spanned(
                   variant,
                   INVALID_ENUM_VARIANT_TAG
               ))
            }


            let mut builders = EnumValueBuilder(EnumValueTagBuilder::default());

            build_tags_from_field(&Tagged::EnumVariant(variant.clone()), vec![&mut builders])?;

            let tag_builder: EnumValueTagBuilder = builders.into();

            enum_.variants.push(EdgedbEnumVariant {
                value: variant.ident.clone(),
                tag: tag_builder.build(&variant)?,
            });
        }

        Ok(enum_)
    }
}

pub struct EdgedbEnumVariant {
    pub value: Ident,
    pub tag: ValueTag,
}