use std::convert::TryFrom;
use syn::Lit::Str;
use syn::{Field, LitStr, MetaNameValue, Type};
use crate::constants::*;
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::FilterBuilder;
use crate::utils::attributes_utils::has_attribute;
use crate::utils::type_utils::is_type_name;


// region FilterTags
#[derive(Debug, Clone)]
pub enum FilterTags {
    First(FilterTag),
    Other(OtherFilterTags)
}

#[derive(Debug, Clone)]
pub enum OtherFilterTags {
    And(FilterTag),
    Or(FilterTag)
}

impl FilterTags {
    pub fn operator(&self) -> SelectFilterOperator {
        match self {
            FilterTags::First(tag) => tag.operator.clone(),
            FilterTags::Other(tag) => match tag {
                OtherFilterTags::And(tag) => tag.operator.clone(),
                OtherFilterTags::Or(tag) => tag.operator.clone()
            }
        }
    }

    pub fn wrapper_fn(&self) -> Option<String> {
        match self {
            FilterTags::First(tag) => tag.wrapper_fn.clone(),
            FilterTags::Other(tag) => match tag {
                OtherFilterTags::And(tag) => tag.wrapper_fn.clone(),
                OtherFilterTags::Or(tag) => tag.wrapper_fn.clone()
            }
        }
    }

    pub fn conjunctive(&self) -> &'static str {
        match self {
            FilterTags::First(_) => "",
            FilterTags::Other(tag) => match tag {
                OtherFilterTags::And(_) => AND,
                OtherFilterTags::Or(_) => OR
            }
        }
    }
}

// endregion FilterTags


// region FilterTag
#[derive(Debug, Clone)]
pub struct FilterTag {
    pub operator: SelectFilterOperator,
    pub wrapper_fn: Option<String>,
}
// endregion FilterTag

// region FilterTagOptions
pub enum FilterTagOptions {
    Operator(LitStr),
    WrapperFn(String),
}

impl TryFrom<&MetaNameValue> for FilterTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {

        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {

            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }

            match path.get_ident().unwrap().to_string().as_str() {
                OPERATOR => Ok(FilterTagOptions::Operator(value.clone())),
                WRAPPER_FN => Ok(FilterTagOptions::WrapperFn(value.value().replace(['(', ')'], ""))),
                _ => Err(syn::Error::new_spanned(meta_value, INVALID_FILTER_TAG))
            }
        } else {
            Err(syn::Error::new_spanned(meta_value, EXPECT_NAMED_LIT))
        }
    }
}

// endregion FilterTagOptions

// region FilterTagBuilder
#[derive(Debug, Clone, Default)]
pub struct FilterTagBuilder {
    pub operator: Option<LitStr>,
    pub wrapper_fn: Option<String>,
}

impl From<TagBuilders> for FilterTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            FilterBuilder(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for FilterTagBuilder {

    fn tag_names(&self) -> Vec<&str> {
        vec![FILTER, AND_FILTER, OR_FILTER]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let arg = FilterTagOptions::try_from(meta_value)?;

        match arg {
            FilterTagOptions::Operator(operator) => self.operator = Some(operator),
            FilterTagOptions::WrapperFn(wrapper_fn) => self.wrapper_fn = Some(wrapper_fn),
        }

        Ok(())
    }
}

impl FilterTagBuilder {
    pub fn build(self, field: &Field, first: bool) -> syn::Result<FilterTags> {

         if let Some(operator) = self.operator {

           let tag =  FilterTag {
                operator: SelectFilterOperator::try_from((&field.ty, operator.clone()))?,
                wrapper_fn: self.wrapper_fn,
            };

             if first {
                 if has_attribute(field, FILTER) {
                     Ok(FilterTags::First(tag))
                 } else {
                     Err(syn::Error::new_spanned(field, FIRST_FILTER_EXPECTED))
                 }
             } else if has_attribute(field, AND_FILTER) {
                 Ok(FilterTags::Other(OtherFilterTags::And(tag)))
             } else if has_attribute(field, OR_FILTER) {
                 Ok(FilterTags::Other(OtherFilterTags::Or(tag)))
             } else {
                 Err(syn::Error::new_spanned(field, AND_OR_FILTER_EXPECTED))
             }

        } else {
            Err(syn::Error::new_spanned(field, EXPECT_OPERATOR))
        }

    }
}
// endregion FilterTagBuilder

// region: SelectFilterOperator
#[derive(Debug, Clone)]
pub enum SelectFilterOperator {
    Exists,
    NotExists,
    Is,
    IsNot,
    Like,
    ILike,
    In,
    NotIn,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqual,
    LesserThanOrEqual,
}

impl SelectFilterOperator {

    pub fn statement(&self) -> &'static str {
        match self {
            SelectFilterOperator::Is => "=",
            SelectFilterOperator::IsNot => "!=",
            SelectFilterOperator::Like => "like",
            SelectFilterOperator::ILike => "ilike",
            SelectFilterOperator::In => "in",
            SelectFilterOperator::NotIn => "not in",
            SelectFilterOperator::GreaterThan => ">",
            SelectFilterOperator::LesserThan => "<",
            SelectFilterOperator::GreaterThanOrEqual => ">=",
            SelectFilterOperator::LesserThanOrEqual => "<=",
            SelectFilterOperator::Exists => "exists",
            SelectFilterOperator::NotExists => "not exists"
        }
    }

    pub fn check_exist(self) -> bool {
        matches!(self, SelectFilterOperator::Exists | SelectFilterOperator::NotExists)
    }
}

impl TryFrom<(&Type, LitStr)> for SelectFilterOperator {
    type Error = syn::Error;

    fn try_from((ty, lit): (&Type, LitStr)) -> Result<Self, Self::Error> {

        let s = lit.value();

        let check_not_accepted_type = |_: &Type| -> syn::Result<()>{

            if is_type_name(ty, "()") {
                return Err(
                    syn::Error::new_spanned(
                        ty,
                        format!("{INVALID_TYPE_TUPLE_FOR_OPERATOR} {s}"))
                );
            }

            Ok(())
        };

        let check_only_accepted_type = |ty: &Type, op: &str, tty: &str| -> syn::Result<()> {
            if !is_type_name(&ty, tty) {
                return Err(
                    syn::Error::new_spanned(
                        ty,
                        format!("{op} {ONLY_TYPE_FOR_OPERATOR} {tty}"))
                );
            }
            Ok(())
        };

        match s.to_lowercase().as_str() {
            EXISTS => {
                check_only_accepted_type(ty, EXISTS, "()")?;
                Ok(SelectFilterOperator::Exists)
            }
            NOT_EXISTS | BANG_EXISTS => {
                check_only_accepted_type(ty, NOT_EXISTS, "()")?;
                Ok(SelectFilterOperator::NotExists)
            }
            IS | EQUAL => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::Is)
            }
            IS_NOT | NOT_EQUAL => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::IsNot)
            }
            LIKE => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, LIKE, STRING)?;
                Ok(SelectFilterOperator::Like)
            }
            ILIKE => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, ILIKE, STRING)?;
                Ok(SelectFilterOperator::ILike)
            }
            IN => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, IN, VEC)?;
                Ok(SelectFilterOperator::In)
            }
            NOT_IN => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, NOT_IN, VEC)?;
                Ok(SelectFilterOperator::NotIn)
            }
            GREATER_THAN | SUP_SIGN => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::GreaterThan)
            }
            GREATER_THAN_OR_EQUAL | SUP_OR_EQ_SIGN => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::GreaterThanOrEqual)
            }
            LESSER_THAN | INF_SIGN => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::LesserThan)
            }
            LESSER_THAN_OR_EQUAL | INF_OR_EQ_SIGN => {
                check_not_accepted_type(ty)?;
                Ok(SelectFilterOperator::LesserThanOrEqual)
            }
            _ => Err(syn::Error::new_spanned(lit, INVALID_OPERATOR))
        }
    }
}

// endregion SelectFilterOperator