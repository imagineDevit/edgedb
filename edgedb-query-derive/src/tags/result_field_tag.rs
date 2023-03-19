use std::convert::TryFrom;
use syn::{Field, MetaNameValue};
use syn::Lit::Str;
use crate::constants::{COLUMN_NAME, DEFAULT_VALUE, EXPECT_NAMED_LIT, EXPECT_NON_EMPTY_LIT, FIELD, INVALID_FIELD_TAG, SCALAR_TYPE, WRAPPER_FN};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::ResultField;
use crate::utils::attributes_utils::has_attribute;

// region ResultFieldTag
#[derive(Debug, Clone)]
pub struct ResultFieldTag {
    pub column_name: Option<String>,
    pub wrapper_fn: Option<String>,
    pub default_value: Option<String>,
}

impl ResultFieldTag {
    pub fn build_statement(&self, f_name: String) -> String {

        let mut s = match (self.column_name.clone(), self.wrapper_fn.clone()) {
            (Some(column), None) => {
                if column != f_name {
                    format!("{f_name} := .{column}")
                } else {
                    f_name
                }
            }

            (None, Some(wrapper_fn)) =>
                format!("{f_name} := (select {SCALAR_TYPE}{wrapper_fn}(.{f_name}))"),

            (Some(column), Some(wrapper_fn)) =>
                format!("{f_name} := (select {SCALAR_TYPE}{wrapper_fn}(.{column}))"),

            (None, None) => f_name

        };

        if let Some(v) = self.default_value.clone() {
            s = format!("{s} ?? (select {SCALAR_TYPE}'{v}')");
        }

        s

    }
}
// endregion ResultFieldTag

// region ResultFieldTagOptions
pub enum ResultFieldTagOptions {
    ColumnName(String),
    WrapperFn(String),
    DefaultValue(String),
}

impl TryFrom<&MetaNameValue> for ResultFieldTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {
        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {
            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }


            match path.get_ident().unwrap().to_string().as_str() {
                COLUMN_NAME => Ok(ResultFieldTagOptions::ColumnName(value.value())),
                WRAPPER_FN => Ok(ResultFieldTagOptions::WrapperFn(value.value())),
                DEFAULT_VALUE => Ok(ResultFieldTagOptions::DefaultValue(value.value())),
                _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
            }
        } else {
            Err(syn::Error::new_spanned(meta_value, EXPECT_NAMED_LIT))
        }
    }
}

// endregion ResultFieldTagOptions

// region ResultFieldTagBuilder
#[derive(Debug, Clone, Default)]
pub struct ResultFieldTagBuilder {
    pub column_name: Option<String>,
    pub wrapper_fn: Option<String>,
    pub default_value: Option<String>,
}

impl From<TagBuilders> for ResultFieldTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            ResultField(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for ResultFieldTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![FIELD]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let option = ResultFieldTagOptions::try_from(meta_value)?;
        match option {
            ResultFieldTagOptions::ColumnName(value) => self.column_name = Some(value),
            ResultFieldTagOptions::WrapperFn(value) => self.wrapper_fn = Some(value.replace(['(', ')'], "")),
            ResultFieldTagOptions::DefaultValue(value) => self.default_value = Some(value)
        }

        Ok(())
    }
}

impl ResultFieldTagBuilder {
    pub fn build(self, field: &Field) -> syn::Result<ResultFieldTag> {

        let all_nones = vec![
            self.column_name.clone(),
            self.wrapper_fn.clone(),
            self.default_value.clone()]
            .iter().all(|o| o.is_none());

        if has_attribute(field, FIELD) && all_nones {
            Err(syn::Error::new_spanned(
                field,
                "#[field] must have at least column_name or wrapper_fn attribute"
            ))
        } else {
            Ok(ResultFieldTag {
                column_name: self.column_name.clone(),
                wrapper_fn: self.wrapper_fn.clone(),
                default_value: self.default_value
            })
        }

    }
}

// endregion ResultFieldTagBuilder


