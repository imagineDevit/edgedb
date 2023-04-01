use proc_macro::{TokenStream};
use std::convert::TryFrom;
use proc_macro2::Span;
use quote::quote;
use syn::{Field, Ident, ItemStruct};
use syn::parse::{Parse, ParseStream};
use crate::constants::{BACKLINK, EXPECTED_ID_FIELD, FIELD, ID, INVALID_RESULT_FIELD_TAG, LIMIT_1, SCALAR_TYPE, VEC};
use crate::queries::QueryField;
use crate::tags::backlink_field_tag::{BackLinkFieldTag, BackLinkFieldTagBuilder};
use crate::tags::{build_tags_from_field, Tagged};
use crate::tags::result_field_tag::{ResultFieldTag, ResultFieldTagBuilder};
use crate::tags::TagBuilders::{BackLinkFieldBuilder, ResultFieldBuilder};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::format_scalar;
use crate::utils::type_utils::is_type_name;

pub struct QueryResult {
    pub ident: Ident,
    pub fields: Vec<ResultField>,
}


impl QueryResult {
    fn new(ident: Ident) -> Self {
        Self {
            ident,
            fields: vec![],
        }
    }

    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {
        let struct_name = self.ident.clone();

        let fields = self.fields.iter();

        let id_field = fields.clone().find(|f| f.field.ident == ID)
            .ok_or_else(|| syn::Error::new(Span::call_site(), EXPECTED_ID_FIELD))?;

        let id_ty = id_field.field.ty.clone();

        let fields_quote = fields.clone()
            .map(|f| f.field.struct_field_quote());

        let field_shapes = fields.clone()
            .map(|f| f.shape_quote());

        let add_field = fields.map(|f|{
            let f_name =f.field.ident.clone().to_string();
            quote! {
                fields.push(#f_name);
            }
        });

        let tokens = quote! {

            const _: () = {
                use std::marker::PhantomData;
                struct Id(PhantomData<uuid::Uuid>);
                let _ = Id(PhantomData::<#id_ty>);
            };

            #[derive(Debug, Clone, Default, edgedb_derive::Queryable)]
            pub struct #struct_name {
                #(#fields_quote)*
            }

            impl edgedb_query::ToEdgeShape for #struct_name {
                fn shape() -> String {

                    use edgedb_query::ToEdgeScalar;

                    let mut query = "{".to_string();
                    #(#field_shapes)*
                    query.pop();
                    query.push_str("}");
                    query
                }
            }

            impl edgedb_query::ToEdgeScalar for #struct_name {
                fn scalar() -> String {
                    String::default()
                }
            }

            impl edgedb_query::EdgeResult for #struct_name {
                fn returning_fields() -> Vec<&'static str> {
                    let mut fields = vec![];
                    #(#add_field)*
                    fields
                }
            }

        };

        Ok(tokens.into())
    }
}

impl Parse for QueryResult {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let strukt = input.parse::<ItemStruct>()?;

        let mut result = QueryResult::new(strukt.ident.clone());

        for field in strukt.fields {
            result.fields.push(ResultField::try_from(&field)?)
        }

        Ok(result)
    }
}


// region ResultTags
#[derive(Clone)]
pub enum ResultTags {
    NoTag,
    FieldTag(ResultFieldTag),
    BackLink(BackLinkFieldTag),
}
// endregion ResultTags

// region ResultField
pub struct ResultField {
    pub field: QueryField,
    pub tag: ResultTags,
}

impl ResultField {

    fn shape_quote(&self) -> proc_macro2::TokenStream {
        let f_name = self.field.ident.clone().to_string();
        let tty = self.field.ty.clone();

        match self.tag.clone() {
            ResultTags::NoTag => {
                quote! {
                    let shape = #tty::shape();
                    if shape.is_empty() {
                        query.push_str(#f_name);
                        query.push_str(",");
                    } else {
                        let s = format!("{} : {}", #f_name, shape);
                        query.push_str(s.as_str());
                        query.push_str(",");
                    }
                }
            }
            ResultTags::FieldTag(tag) => {
                let stmt = tag.build_statement(f_name);
                let scalar = SCALAR_TYPE.to_string();
                let format_scalar = format_scalar();
                quote! {
                    let mut scalar: String = #tty::scalar();
                    #format_scalar;
                    let p = #stmt.to_owned().replace(#scalar, scalar.as_str());
                    query.push_str(p.as_str());
                    query.push_str(",");
                }
            }

            ResultTags::BackLink(tag) => {
                let ql = tag.build_statement();
                let result_type = Ident::new(tag.result.as_str(), Span::call_site());
                let is_vec = is_type_name(&tty, VEC);

                let limit = if is_vec {
                    ""
                } else {
                    LIMIT_1
                };

                quote! {
                    let rs = #result_type::shape();
                    let s = format!("{} := ({}{}{})", #f_name, #ql, rs, #limit);
                    query.push_str(s.as_str());
                    query.push_str(",");
                }
            }
        }

    }
}

impl TryFrom<&Field> for ResultField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if field.attrs.is_empty() {
            Ok(Self {
                field: QueryField::try_from((field, vec![]))?,
                tag: ResultTags::NoTag,
            })
        } else if has_attribute(field, FIELD) {
            let mut builders = ResultFieldBuilder(ResultFieldTagBuilder::default());

            build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut builders])?;

            let tag_builder: ResultFieldTagBuilder = builders.into();

            Ok(Self {
                field: QueryField::try_from((field, vec![FIELD]))?,
                tag: ResultTags::FieldTag(tag_builder.build(field)?),
            })
        } else if has_attribute(field, BACKLINK) {
            let mut builders = BackLinkFieldBuilder(BackLinkFieldTagBuilder::default());

            build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut builders])?;

            let tag_builder: BackLinkFieldTagBuilder = builders.into();

            Ok(Self {
                field: QueryField::try_from((field, vec![BACKLINK]))?,
                tag: ResultTags::BackLink(tag_builder.build(field)?),
            })
        } else {
            Err(
                syn::Error::new_spanned(
                    field,
                    INVALID_RESULT_FIELD_TAG,
                )
            )
        }
    }
}

// endregion ResultField