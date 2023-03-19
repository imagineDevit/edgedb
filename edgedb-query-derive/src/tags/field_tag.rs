use std::convert::TryFrom;
use syn::{Field, MetaNameValue};
use syn::Lit::Str;
use crate::constants::{COLUMN_NAME, EXPECT_NAMED_LIT, EXPECT_NON_EMPTY_LIT, FIELD, INVALID_FIELD_TAG, PARAM, SCALAR};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::Field;
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
}

impl TryFrom<&MetaNameValue> for FieldTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {
        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {
            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }

            match path.get_ident().unwrap().to_string().as_str() {
                COLUMN_NAME => Ok(FieldTagOptions::ColumnName(value.value())),
                PARAM => Ok(FieldTagOptions::ParameterLabel(value.value())),
                SCALAR => Ok(FieldTagOptions::ScalarType(value.value())),
                _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
            }
        } else {
            Err(syn::Error::new_spanned(meta_value, EXPECT_NAMED_LIT))
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
}

impl From<TagBuilders> for FieldTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            Field(builder) => builder,
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
            FieldTagOptions::ScalarType(value) => self.scalar_type = Some(value)
        }

        Ok(())
    }
}

impl FieldTagBuilder {
    pub fn build(self, field: &Field) -> syn::Result<FieldTag> {
        let scalar_type = if let Some(mut scalar) = self.scalar_type.clone() {
            match_scalar(&field.ty, scalar.clone())?;

            if !scalar.starts_with('<') {
                scalar = format!("<{scalar}");
            }

            if !scalar.ends_with('>') {
                scalar = format!("{scalar}>");
            }

            scalar
        } else {
            get_scalar(&field.ty)?
        };

        Ok(FieldTag {
            column_name: self.column_name.unwrap_or(field.ident.as_ref().unwrap().to_string()),
            parameter_label: self.parameter_label.unwrap_or(field.ident.as_ref().unwrap().to_string()),
            scalar_type,
        })
    }
}

// endregion FieldTagBuilder


