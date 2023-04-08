use std::convert::TryFrom;

use syn::Field;

use crate::constants::{NESTED_QUERY, SET};
use crate::queries::QueryField;
use crate::tags::{build_tags_from_field, TagBuilders, Tagged};
use crate::tags::set_tag::{SetTag, SetTagBuilder};

// region NestedQueryField
#[derive(Debug, Clone)]
pub struct NestedQueryField {
    pub field: QueryField,
    pub set_tag: Option<SetTag>,
    pub parent_table_name: Option<String>
}


pub enum NestedQueryParentType {
    Query, Set
}

impl TryFrom<(&Field, NestedQueryParentType)> for NestedQueryField {
    type Error = syn::Error;

    fn try_from((field, parent_type): (&Field, NestedQueryParentType)) -> Result<Self, Self::Error> {

        let (set_tag,tags) =  match parent_type {
            NestedQueryParentType::Query => {
                (None,   vec![NESTED_QUERY])
            }
            NestedQueryParentType::Set => {
                let mut set_tag_builder = TagBuilders::SetBuilder(SetTagBuilder::default());
                build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut set_tag_builder])?;
                let set_tag_builder: SetTagBuilder = set_tag_builder.into();
                (Some(set_tag_builder.build(field)?),   vec![NESTED_QUERY, SET])
            }
        };
        Ok(Self {
            field: QueryField::try_from((field, tags))?,
            set_tag,
            parent_table_name: None
        })
    }
}

impl NestedQueryField {
    
    pub fn set_parent_table_name(&mut self, name: impl Into<String>) {
        self.parent_table_name = Some(name.into())
    }
    
    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(true, None, self.parent_table_name.clone())
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(false, self.set_tag.clone(), self.parent_table_name.clone())
    }
    
}

// endregion NestedQueryField