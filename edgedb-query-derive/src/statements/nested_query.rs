use std::convert::TryFrom;

use syn::Field;

use crate::constants::{NESTED_QUERY, SET};
use crate::queries::QueryField;
use crate::tags::set_tag::{SetTag, SetTagBuilder};
use crate::tags::{build_tags_from_field, TagBuilders, Tagged};

// region NestedQueryField
#[derive(Debug, Clone)]
pub struct NestedQueryField {
    pub field: QueryField,
    pub set_tag: Option<SetTag>
}

impl TryFrom<(&Field, bool)> for NestedQueryField {
    type Error = syn::Error;

    fn try_from((field, set): (&Field, bool)) -> Result<Self, Self::Error> {

        let (tag, tags) = if set {
            let mut set_tag_builder = TagBuilders::SetBuilder(SetTagBuilder::default());
            build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut set_tag_builder])?;
            let set_tag_builder: SetTagBuilder = set_tag_builder.into();
            (Some(set_tag_builder.build(field)?), vec![NESTED_QUERY, SET])
        } else {
            (None, vec![NESTED_QUERY])
        };

        Ok(Self {
            field: QueryField::try_from((field, tags))?,
            set_tag: tag
        })
    }
}

impl NestedQueryField {
    
    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(true, None)
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(false, self.set_tag.clone())
    }
    
}

// endregion NestedQueryField