use std::convert::TryFrom;

use syn::{Field, MetaNameValue};
use syn::Lit::Str;
use crate::constants::{BACKLINK,  DEFAULT_MODULE, EXPECT_NAMED_LIT, EXPECT_NON_EMPTY_LIT, INVALID_BACKLINK_TAG,  MODULE,  RESULT, SOURCE_TABLE, TARGET_COLUMN, TARGET_TABLE};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::BackLinkField;


// region ResultFieldTag
#[derive(Debug, Clone)]
pub struct BackLinkFieldTag {
    pub module: String,
    pub source_table: String,
    pub target_table: String,
    pub target_column: String,
    pub result: String
}

impl BackLinkFieldTag {
    pub fn build_statement(&self) -> String {
        let source = self.source_table.clone();
        let column = self.target_column.clone();
        let module = self.module.clone();
        let table = self.target_table.clone();

        format!("select {module}::{source}.<{column}[is {module}::{table}]")
    }
}
// endregion ResultFieldTag

// region ResultFieldTagOptions
pub enum BackLinkFieldTagOptions {
    Module(String),
    SourceTable(String),
    TargetTable(String),
    TargetColumn(String),
    Result(String)
}

impl TryFrom<&MetaNameValue> for BackLinkFieldTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {
        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {
            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }


            match path.get_ident().unwrap().to_string().as_str() {
                MODULE => Ok(BackLinkFieldTagOptions::Module(value.value())),
                SOURCE_TABLE => Ok(BackLinkFieldTagOptions::SourceTable(value.value())),
                TARGET_TABLE => Ok(BackLinkFieldTagOptions::TargetTable(value.value())),
                TARGET_COLUMN => Ok(BackLinkFieldTagOptions::TargetColumn(value.value())),
                RESULT => Ok(BackLinkFieldTagOptions::Result(value.value())),
                _ => Err(syn::Error::new_spanned(meta_value, INVALID_BACKLINK_TAG))
            }
        } else {
            Err(syn::Error::new_spanned(meta_value, EXPECT_NAMED_LIT))
        }
    }
}

// endregion ResultFieldTagOptions

// region ResultFieldTagBuilder
#[derive(Debug, Clone, Default)]
pub struct BackLinkFieldTagBuilder {
    pub module: Option<String>,
    pub source_table: Option<String>,
    pub target_table: Option<String>,
    pub target_column: Option<String>,
    pub result: Option<String>,
}

impl From<TagBuilders> for BackLinkFieldTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            BackLinkField(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for BackLinkFieldTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![BACKLINK]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let option = BackLinkFieldTagOptions::try_from(meta_value)?;
        match option {
            BackLinkFieldTagOptions::Module(value) => self.module = Some(value),
            BackLinkFieldTagOptions::SourceTable(value) => self.source_table = Some(value.replace(['(', ')'], "")),
            BackLinkFieldTagOptions::TargetTable(value) => self.target_table = Some(value),
            BackLinkFieldTagOptions::TargetColumn(value) => self.target_column = Some(value),
            BackLinkFieldTagOptions::Result(value) => self.result = Some(value)
        }

        Ok(())
    }
}

impl BackLinkFieldTagBuilder {
    pub fn build(self, field: &Field) -> syn::Result<BackLinkFieldTag> {
        Ok(

            BackLinkFieldTag {
                module: self.module.unwrap_or(DEFAULT_MODULE.to_owned()),
                source_table: get_value(field, self.source_table, SOURCE_TABLE)?,
                target_table: get_value(field, self.target_table, TARGET_TABLE)?,
                target_column: get_value(field, self.target_column, TARGET_COLUMN)?,
                result: get_value(field, self.result, RESULT)?,
            }
        )
    }
}

// endregion ResultFieldTagBuilder

fn get_value(field: &Field, value: Option<String>, value_name: impl Into<String>) -> syn::Result<String> {
    value.ok_or(
        syn::Error::new_spanned(
            field,
            format!("Option {} expected", value_name.into())
        )
    )
}
