use std::convert::TryFrom;
use regex::Regex;
use syn::{Field, Ident, ItemStruct};
use syn::parse::{Parse, ParseStream};
use crate::constants::{PARAM, PARAM_PATTERN};
use crate::builders::impl_builder::{FieldCat, QueryImplBuilder, ImplBuilderField};
use crate::meta_data::{SrcFile, try_get_meta};
use crate::queries::{Query, QueryField};
use crate::tags::{build_tags_from_field, Tagged};
use crate::tags::param_tag::{ParamTag, ParamTagBuilder};
use crate::tags::TagBuilders::Param;

#[derive(Debug, Clone)]
pub struct FileQuery {
    pub ident: Ident,
    pub meta: Option<SrcFile>,
    pub params: Vec<ParamField>
}

impl FileQuery {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            meta: None,
            params: vec![]
        }
    }

    pub fn with_meta(&mut self, meta: SrcFile) -> &mut Self {
        self.meta = Some(meta);
        self
    }

    pub fn validate(&self) -> syn::Result<&Self> {
        let file = try_get_meta(&self.ident.clone(), || self.meta.clone())?;

        let content = file.get_content(&self.ident)?;

        let param_regex = Regex::new(PARAM_PATTERN).unwrap();

        let param_matches = param_regex.find_iter(content.as_str())
            .map(|mat| mat.as_str().to_string())
            .collect::<Vec<String>>();

        let params_values = self.params.iter()
            .map(|f| f.param())
            .collect::<Vec<String>>();


        let param_matches = param_matches.iter()
            .map(|s| s.replace("$", ""))
            .collect::<Vec<String>>();

        let struct_params_not_query = params_values.clone().into_iter()
            .filter(|s| !param_matches.contains(s))
            .collect::<Vec<String>>();

        let query_params_not_struct = param_matches.clone().into_iter()
            .filter(|s| !params_values.contains(&s.replace("$", "")))
            .collect::<Vec<String>>();

        if struct_params_not_query.len() > 0 {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    format!(r"
                    Following struct attributes do not appear as query parameters : {:#?}
                ",struct_params_not_query),
                )
            )
        } else if query_params_not_struct.len() > 0 {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    format!(r"
                    Following query parameters do not appear as struct attribute : {:#?}
                ",query_params_not_struct),
                )
            )
        } else if param_matches != params_values {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    "Query parameters must be in the same order as struct attributes",
                )
            )
        }
        Ok(self)
    }
}

impl Query for FileQuery {
    fn get_param_labels(&self) -> Vec<(Ident, String)> {
        self.params.iter()
            .map(|f| (f.field.ident.clone(), f.param()))
            .collect()
    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder> {
        let meta = try_get_meta(&self.ident, || self.meta.clone())?;

        let fields = self.params.iter()
            .map(|f| ImplBuilderField {
                field: f.field.clone(),
                field_cat: FieldCat::Simple(f.param()),
            })
            .collect::<Vec<ImplBuilderField>>();

        Ok(QueryImplBuilder {
            struct_name: self.ident.clone(),
            fields,
            init_edgeql: meta.get_content(&self.ident)?,
            static_const_check_statements: vec![],
            edgeql_statements: vec![],
        })
    }
}

impl Parse for FileQuery {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let strukt = input.parse::<ItemStruct>()?;

        let mut query = FileQuery::new(strukt.ident.clone());

        for field in strukt.fields {
            query.params.push(ParamField::try_from(&field)?);
        }

        Ok(query)
    }
}


#[derive(Debug, Clone)]
pub struct ParamField {
    pub field: QueryField,
    pub param: ParamTag
}

impl ParamField {
    pub fn param(&self) -> String {
        self.param.0.clone()
    }
}

impl TryFrom<&Field> for ParamField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {

        let mut builders = Param(ParamTagBuilder::default());

        build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut builders])?;

        let param_tag_builder: ParamTagBuilder = builders.into();

        Ok(Self {
            field: QueryField::try_from((field, vec![PARAM]))?,
            param: param_tag_builder.build(field)?
        })
    }
}