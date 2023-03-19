use std::convert::TryFrom;
use proc_macro2::Ident;

use quote::{quote, ToTokens};
use syn::Field;
use syn::punctuated::Iter;
use crate::constants::{EITHER_ONE_SETS_OR_SET_TAG_EXPECTED, FIELD, INVALID_UPDATE_TAG, NESTED_QUERY, ONLY_ONE_SETS_TAG_EXPECTED, SELECT, SET, SETS};
use crate::builders::impl_builder::{FieldCat, ImplBuilderField};

use crate::queries::{check_duplicate_parameter_labels, QueryField};
use crate::statements::nested_query::NestedQueryField;
use crate::tags::field_tag::{FieldTag, FieldTagBuilder};
use crate::tags::set_tag::{SetOption, SetTag, SetTagBuilder};
use crate::tags::{build_tags_from_field, TagBuilders, Tagged};
use crate::utils::attributes_utils::{has_any_attribute, has_attribute, has_none_attribute};
use crate::utils::derive_utils::{nested_element_shape, nested_element_value};


// region SetStatement
#[derive(Debug, Clone)]
pub enum SetStatement {
    SimpleField(UpdateSet),
    NestedQuery(NestedQueryField),
}

impl SetStatement {
    pub fn param_field(&self) -> Option<(Ident, String)> {
        match self {
            Self::SimpleField(f) => Some((f.field.ident.clone(), f.field_tag.parameter_label.clone())),
            Self::NestedQuery(_) => None
        }
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        match self {
            SetStatement::SimpleField(f) => f.add_set_statement_quote(),
            SetStatement::NestedQuery(q) => q.add_set_statement_quote()
        }
    }

    pub fn struct_field_quote(&self) -> proc_macro2::TokenStream {
        match self {
            SetStatement::SimpleField(f) => f.field.struct_field_quote(),
            SetStatement::NestedQuery(nq) => nq.field.struct_field_quote()
        }
    }

    pub fn shape_quote(&self) -> proc_macro2::TokenStream {
        match self {
            SetStatement::SimpleField(s) => s.field.field_shape_quote(s.field_tag.parameter_label.clone()),
            SetStatement::NestedQuery(nq) => nested_element_shape(nq.field.ident.clone())
        }
    }

    pub fn value_quote(&self) -> proc_macro2::TokenStream {
        match self {
            SetStatement::SimpleField(s) => s.field.field_value_quote(),
            SetStatement::NestedQuery(nq) => nested_element_value(nq.field.ident.clone())
        }
    }


}

impl TryFrom<&Field> for SetStatement {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if has_attribute(field, NESTED_QUERY) {
            Ok(SetStatement::NestedQuery(NestedQueryField::try_from(field)?))
        } else if has_any_attribute(field, vec![SET, FIELD]) || field.attrs.is_empty() {
            Ok(SetStatement::SimpleField(UpdateSet::try_from(field)?))
        } else {
            Err(syn::Error::new_spanned(field.to_token_stream(), INVALID_UPDATE_TAG))
        }
    }
}

// region UpdateSet
#[derive(Debug, Clone)]
pub struct UpdateSet {
    pub field: QueryField,
    pub field_tag: FieldTag,
    pub set_tag: SetTag,
}

impl UpdateSet {
    pub fn build_statement(&self) -> String {
        let column_name = self.field_tag.column_name.clone();
        let scalar_type = self.field_tag.scalar_type.clone();
        let param = self.field_tag.parameter_label.clone();
        let assignment = self.set_tag.option.statement();

        match self.set_tag.option {
            SetOption::Assign => format!("{column_name} {assignment} ({SELECT} {scalar_type}${param}), "),
            SetOption::Concat => format!("{column_name} := .{column_name} ++ ({SELECT} {scalar_type}${param}), "),
            SetOption::Push => format!("{column_name} += ({SELECT} {scalar_type}${param}), ")
        }
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        let stmt = self.build_statement();
        quote! {
           set_stmt.push_str(#stmt);
        }
    }
}

impl TryFrom<&Field> for UpdateSet {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let mut field_tag_builder = TagBuilders::FieldBuilder(FieldTagBuilder::default());
        let mut set_tag_builder = TagBuilders::SetBuilder(SetTagBuilder::default());

        build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut field_tag_builder, &mut set_tag_builder])?;

        let field_tag_builder: FieldTagBuilder = field_tag_builder.into();
        let set_tag_builder: SetTagBuilder = set_tag_builder.into();

        Ok(Self {
            field: QueryField::try_from((field, vec![FIELD, SET]))?,
            field_tag: field_tag_builder.build(field)?,
            set_tag: set_tag_builder.build(field)?,
        })
    }
}

// endregion UpdateSet

// region UpdateSets
#[derive(Debug, Clone)]
pub struct UpdateSets {
    pub field: QueryField
}

impl UpdateSets {
    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        let f_name = self.field.ident.clone();
        quote!{
            let set_stmt = self.#f_name.to_edgeql();
        }
    }
}

impl TryFrom<&Field> for UpdateSets {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        Ok(Self{
            field: QueryField::try_from((field, vec![SETS]))?
        })
    }
}


// endregion UpdateSets

// region UpdateSetStatement

pub enum UpdateSetStatement {
    None,
    OneSets(UpdateSets),
    ManySet(Vec<SetStatement>)
}

impl UpdateSetStatement {
    pub fn get_parameter_labels(&self) -> Vec<(Ident, String)> {
       if let UpdateSetStatement::ManySet(sets) = self {
          sets.iter()
               .filter_map(|set| set.param_field())
               .collect::<Vec<(Ident, String)>>()
       } else {
           vec![]
       }
    }

    pub fn to_impl_builder_field(&self) -> Vec<ImplBuilderField> {
        match self {
            UpdateSetStatement::None => vec![],
            UpdateSetStatement::ManySet(filters) => {
                filters.iter().map(|stmt| {
                    match stmt {
                        SetStatement::SimpleField(f) => ImplBuilderField {
                            field: f.field.clone(),
                            field_cat: FieldCat::Simple(f.field_tag.parameter_label.clone()),
                        },
                        SetStatement::NestedQuery(f) => ImplBuilderField {
                            field: f.field.clone(),
                            field_cat: FieldCat::Nested,
                        }
                    }
                }).collect()
            }
            UpdateSetStatement::OneSets(sets) => vec![
                ImplBuilderField {
                    field: sets.field.clone(),
                    field_cat: FieldCat::Nested,
                }
            ]
        }
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        match self {
            UpdateSetStatement::None => quote!(),
            UpdateSetStatement::OneSets(set) => {
                set.add_set_statement_quote()
            }
            UpdateSetStatement::ManySet(sets) => {
                let set_stmts = sets.iter().map(|set| {
                    set.add_set_statement_quote()
                });

                quote!{
                    let mut set_stmt = "set { ".to_string();
                    #(#set_stmts)*
                    set_stmt.pop();
                    set_stmt.pop();
                    set_stmt.push_str(" }");
                }
            }
        }
    }

    pub fn struct_field_quote(&self) -> proc_macro2::TokenStream {
        match self {
            UpdateSetStatement::None => quote!(),
            UpdateSetStatement::OneSets(sets) => sets.field.struct_field_quote(),
            UpdateSetStatement::ManySet(sets) => {
                let s  =  sets.iter().map(|s| s.struct_field_quote());
                quote!(#(#s)*)
            }
        }
    }

    pub fn shape_quote(&self) -> proc_macro2::TokenStream {
        match self {
            UpdateSetStatement::None => quote!(),
            UpdateSetStatement::ManySet(sets) => {
                let shapes = sets.iter()
                    .map(|s| s.shape_quote());

                quote! {
                    #(#shapes)*
                }

            }
            UpdateSetStatement::OneSets(sets) => nested_element_shape(sets.field.ident.clone())
        }
    }

    pub fn value_quote(&self) -> proc_macro2::TokenStream {
        match self {
            UpdateSetStatement::None => quote!(),
            UpdateSetStatement::ManySet(sets) => {
                let shapes = sets.iter()
                    .map(|s| s.value_quote());

                quote! {
                    #(#shapes)*
                }

            }
            UpdateSetStatement::OneSets(sets) => nested_element_value(sets.field.ident.clone())
        }
    }

    pub fn check_duplicate_parameter_labels(&self) -> syn::Result<()> {
        check_duplicate_parameter_labels(self.get_parameter_labels())
    }
}

pub fn sets_from_fields(field_iter: Iter<Field>, exclude_tags: Vec<&str>, from_sets: bool, error_msg: &str) -> syn::Result<UpdateSetStatement> {

    let mut stmt = UpdateSetStatement::None;

    if !from_sets {
        let sets_fields: Vec<&Field> = field_iter.clone()
            .filter(|f| has_attribute(f, SETS))
            .collect::<Vec<&Field>>();

        match sets_fields.len() {
            0 => {}
            1 => {
                stmt = UpdateSetStatement::OneSets(UpdateSets::try_from(sets_fields[0])?);
            }
            _ => {
                return Err(syn::Error::new_spanned(sets_fields[1].to_token_stream(), ONLY_ONE_SETS_TAG_EXPECTED));
            }
        }
    }

    let mut sets: Vec<SetStatement> = vec![];

    for field in field_iter {
        if has_none_attribute(field, exclude_tags.clone()) && !has_attribute(field, SETS) {

           if has_any_attribute(field, vec![SET, NESTED_QUERY, FIELD])  || field.attrs.is_empty() {

                if let UpdateSetStatement::OneSets(_) = stmt {
                    return Err(syn::Error::new_spanned(field, EITHER_ONE_SETS_OR_SET_TAG_EXPECTED));
                }

                sets.push(SetStatement::try_from(field)?);
            } else {
                return Err(syn::Error::new_spanned(field, error_msg));
            }
        }

    }

    if !sets.is_empty() {
        stmt = UpdateSetStatement::ManySet(sets);
    }

    Ok(stmt)
}
// endregion UpdateSetStatement