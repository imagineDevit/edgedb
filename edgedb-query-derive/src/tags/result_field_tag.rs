use std::convert::TryFrom;
use syn::{Field, MetaNameValue};
use syn::Lit::{Bool, Str};
use crate::constants::{COLUMN_NAME, DEFAULT_VALUE, EXPECT_NON_EMPTY_LIT, FIELD, INVALID_FIELD_TAG, SCALAR_TYPE, WRAPPER_FN, LINK_PROPERTY, INVALID_RESULT_FIELD_TAG, EXPECT_LIT_STR, EXPECT_LIT_BOOL};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::ResultFieldBuilder;
use crate::utils::attributes_utils::has_attribute;

// region ResultFieldTag
#[derive(Debug, Clone)]
pub struct ResultFieldTag {
    pub column_name: Option<String>,
    pub wrapper_fn: Option<String>,
    pub default_value: Option<String>,
    pub link_property: bool,
}

impl ResultFieldTag {
    pub fn build_statement(&self, f_name: String) -> String {
        let apply_link = |s: String| {
            if self.link_property {
                format!("@{s}")
            } else {
                s
            }
        };

        let mut s = match (self.column_name.clone(), self.wrapper_fn.clone()) {
            (Some(column), None) => {
                if column != f_name {
                    format!("{f_name} := .{}", apply_link(column))
                } else {
                    apply_link(f_name)
                }
            }

            (None, Some(wrapper_fn)) =>
                format!("{f_name} := (select {SCALAR_TYPE}{wrapper_fn}(.{}))", apply_link(f_name.clone())),

            (Some(column), Some(wrapper_fn)) =>
                format!("{f_name} := (select {SCALAR_TYPE}{wrapper_fn}(.{}))", apply_link(column)),

            (None, None) => apply_link(f_name)
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
    LinkProperty(bool),
}

impl TryFrom<&MetaNameValue> for ResultFieldTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {

        let MetaNameValue { ref path, lit, .. } = meta_value;

        match lit {
            Str(value) => {
                if value.value().is_empty() {
                    return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
                }

                match path.get_ident().unwrap().to_string().as_str() {
                    COLUMN_NAME => Ok(ResultFieldTagOptions::ColumnName(value.value())),
                    WRAPPER_FN => Ok(ResultFieldTagOptions::WrapperFn(value.value())),
                    DEFAULT_VALUE => Ok(ResultFieldTagOptions::DefaultValue(value.value())),
                    LINK_PROPERTY => Err(syn::Error::new_spanned(meta_value, EXPECT_LIT_BOOL)),
                    _ => Err(syn::Error::new_spanned(meta_value, INVALID_RESULT_FIELD_TAG))
                }
            }

            Bool(value) => {
                match path.get_ident().unwrap().to_string().as_str() {
                    LINK_PROPERTY => Ok(ResultFieldTagOptions::LinkProperty(value.value())),
                    COLUMN_NAME | WRAPPER_FN | DEFAULT_VALUE => Err(syn::Error::new_spanned(meta_value, EXPECT_LIT_STR)),
                    _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
                }
            }
            _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
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
    pub link_property: Option<bool>,
}

impl From<TagBuilders> for ResultFieldTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            ResultFieldBuilder(builder) => builder,
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
            ResultFieldTagOptions::DefaultValue(value) => self.default_value = Some(value),
            ResultFieldTagOptions::LinkProperty(value) => self.link_property = Some(value)
        }

        Ok(())
    }
}

impl ResultFieldTagBuilder {
    pub fn build(self, field: &Field) -> syn::Result<ResultFieldTag> {
        let all_nones = vec![
            self.column_name.clone(),
            self.wrapper_fn.clone(),
            self.default_value.clone(),
            self.link_property.map(|v| v.to_string()),
        ]
            .iter()
            .all(|o| o.is_none());

        if has_attribute(field, FIELD) && all_nones {
            Err(syn::Error::new_spanned(
                field,
                "#[field] must have at least column_name, wrapper_fn or link_property attribute",
            ))
        } else {
            Ok(ResultFieldTag {
                column_name: self.column_name.clone(),
                wrapper_fn: self.wrapper_fn.clone(),
                default_value: self.default_value,
                link_property: self.link_property.unwrap_or(false),
            })
        }
    }
}

// endregion ResultFieldTagBuilder


