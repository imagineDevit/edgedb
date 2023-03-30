use std::convert::TryFrom;
use syn::{Field, MetaNameValue};
use syn::Lit::Str;
use crate::constants::{EXPECT_NON_EMPTY_LIT, INVALID_FIELD_TAG,  UNLESS_CONFLICT};
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::UnlessConfictBuilder;

// region FieldTag
#[derive(Debug, Clone)]
pub struct UnlessConflictTag {
    pub on: Vec<String>,
}

// endregion FieldTag

// region FieldTagOptions
pub enum UnlessConflictTagOptions {
    On(String)
}

impl TryFrom<&MetaNameValue> for UnlessConflictTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {
        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {
            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }

            match path.get_ident().unwrap().to_string().as_str() {
                "on"=> Ok(UnlessConflictTagOptions::On(value.value())),
                _ => Err(syn::Error::new_spanned(meta_value, INVALID_FIELD_TAG))
            }
        } else {
            Ok(UnlessConflictTagOptions::On("".to_string()))
        }
    }
}

// endregion FieldTagOptions

// region FieldTagBuilder
#[derive(Debug, Clone, Default)]
pub struct UnlessConflictTagBuilder {
    pub on: Option<String>,
}

impl From<TagBuilders> for UnlessConflictTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            UnlessConfictBuilder(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for UnlessConflictTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![UNLESS_CONFLICT]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let option = UnlessConflictTagOptions::try_from(meta_value)?;
        match option {
            UnlessConflictTagOptions::On(value) => self.on = Some(value),
        }

        Ok(())
    }
}

impl UnlessConflictTagBuilder {
    pub fn build(self, field: &Field, columns: Vec<String>) -> syn::Result<UnlessConflictTag> {
        
        let ons: Vec<String> = self.on
            .unwrap_or("".to_string())
            .split(',')
            .map(|s| s.trim().to_string()).collect();

        let x : Vec<String> = ons.clone()
            .into_iter()
            .filter(|c| !columns.contains(c))
            .collect();
        
        if !x.is_empty() {
            return Err(syn::Error::new_spanned(field, format!("Following names are not column names : {x:#?}")));
        }
        
        Ok(UnlessConflictTag {
            on: ons
        })
    }
}

// endregion FieldTagBuilder


