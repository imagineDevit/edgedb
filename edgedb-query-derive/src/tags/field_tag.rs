use std::convert::TryFrom;
use syn::{Field, MetaNameValue};
use syn::Lit::{Bool, Str};
use crate::constants::{COLUMN_NAME, EXPECT_NON_EMPTY_LIT, FIELD, INVALID_FIELD_TAG, PARAM, SCALAR, LINK_PROPERTY, EXPECT_LIT_BOOL, EXPECT_LIT_STR, INF_SIGN, SUP_SIGN};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::FieldBuilder;
use crate::tags::utils::{get_column_name, validate_link_property};
use crate::utils::type_utils::{get_scalar, match_scalar};

// region FieldTag
#[derive(Debug, Clone)]
pub struct FieldTag {
    pub column_name: String,
    pub parameter_label: String,
    pub scalar_type: String,
}


// endregion FieldTag

// region FieldTagOptions
pub enum FieldTagOptions {
    ColumnName(String),
    ParameterLabel(String),
    ScalarType(String),
    LinkProperty(bool)
}

impl TryFrom<&MetaNameValue> for FieldTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {

        let MetaNameValue { ref path, lit, .. } = meta_value;

        match lit {
            Str(value) => {
                if value.value().is_empty() {
                    return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
                }

                match path.get_ident().unwrap().to_string().as_str() {
                    COLUMN_NAME => Ok(FieldTagOptions::ColumnName(value.value())),
                    PARAM => Ok(FieldTagOptions::ParameterLabel(value.value())),
                    SCALAR => Ok(FieldTagOptions::ScalarType(value.value())),
                    LINK_PROPERTY => Err(syn::Error::new_spanned(meta_value, EXPECT_LIT_BOOL)),
                    _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
                }
            }

            Bool(value) => {
                match path.get_ident().unwrap().to_string().as_str() {
                    LINK_PROPERTY => Ok(FieldTagOptions::LinkProperty(value.value())),
                    COLUMN_NAME | PARAM | SCALAR => Err(syn::Error::new_spanned(meta_value, EXPECT_LIT_STR)),
                    _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
                }
            }
            _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
        }

    }
}

// endregion FieldTagOptions

// region FieldTagBuilder
#[derive(Debug, Clone, Default)]
pub struct FieldTagBuilder {
    pub column_name: Option<String>,
    pub parameter_label: Option<String>,
    pub scalar_type: Option<String>,
    pub link_property: Option<bool>
}

impl From<TagBuilders> for FieldTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            FieldBuilder(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for FieldTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![FIELD]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let option = FieldTagOptions::try_from(meta_value)?;
        match option {
            FieldTagOptions::ColumnName(value) => self.column_name = Some(value),
            FieldTagOptions::ParameterLabel(value) => self.parameter_label = Some(value),
            FieldTagOptions::ScalarType(value) => self.scalar_type = Some(value),
            FieldTagOptions::LinkProperty(value) => self.link_property = Some(value)
        }

        Ok(())
    }
}

impl FieldTagBuilder {

    pub fn build(&self, field: &Field) -> syn::Result<FieldTag> {

        validate_link_property(self.column_name.clone(), self.link_property, field)?;

        let scalar_type = if let Some(mut scalar) = self.scalar_type.clone() {
            match_scalar(&field.ty, scalar.clone())?;

            if !scalar.starts_with(INF_SIGN) {
                scalar = format!("{INF_SIGN}{scalar}");
            }

            if !scalar.ends_with(SUP_SIGN) {
                scalar = format!("{scalar}{SUP_SIGN}");
            }

            scalar
        } else {
            get_scalar(&field.ty)?
        };

        Ok(FieldTag {
            column_name: get_column_name(self.column_name.clone(), self.link_property, field),
            parameter_label: self.parameter_label.clone().unwrap_or(field.ident.as_ref().unwrap().to_string()),
            scalar_type,
        })
    }
}

// endregion FieldTagBuilder


