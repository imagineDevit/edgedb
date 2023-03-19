
use std::convert::TryFrom;
use syn::{Field, LitStr, MetaNameValue, Type};
use syn::Lit::Str;
use crate::constants::*;
use crate::tags::{NamedValueTagBuilder, TagBuilders};
use crate::tags::TagBuilders::SetBuilder;
use crate::utils::type_utils::is_type_name;

// region SetTag
#[derive(Debug, Clone)]
pub struct SetTag {
    pub option: SetOption,
}

// endregion SetTag

// region SetTagOptions
#[derive(Debug, Clone)]
pub enum SetTagOptions {
    Option(LitStr)
}

impl TryFrom<&MetaNameValue> for SetTagOptions {
    type Error = syn::Error;

    fn try_from(meta_value: &MetaNameValue) -> Result<Self, Self::Error> {
        if let MetaNameValue { ref path, lit: Str(value), .. } = meta_value {

            if value.value().is_empty() {
                return Err(syn::Error::new_spanned(value, EXPECT_NON_EMPTY_LIT));
            }

            match path.get_ident().unwrap().to_string().as_str() {
                SET_OPTION => Ok(SetTagOptions::Option(value.clone())),

                _ => Err(syn::Error::new_spanned(meta_value, INVALID_SET_TAG_OPTION))
            }
        } else {
            Err(syn::Error::new_spanned(meta_value, EXPECT_NAMED_LIT))
        }
    }
}
// endregion SetTagOptions

// region SetOption
#[derive(Debug, Clone)]
pub enum SetOption {
    Assign,
    Concat,
    Push,
}

impl SetOption {
    pub fn statement(&self) -> &str {
        match self {
            SetOption::Assign => ASSIGN_SIGN,
            SetOption::Concat => CONCAT_SIGN,
            SetOption::Push => PUSH_SIGN
        }
    }
}

impl TryFrom<(&Type, LitStr)> for SetOption {
    type Error = syn::Error;

    fn try_from((ty, lit): (&Type, LitStr)) -> Result<Self, Self::Error> {
        let s = lit.value();
        match s.to_lowercase().as_str() {
            ASSIGN | ASSIGN_SIGN => Ok(SetOption::Assign),
            CONCAT | CONCAT_SIGN => Ok(SetOption::Concat),
            PUSH | PUSH_SIGN => {
                if is_type_name(ty, VEC) {
                    Ok(SetOption::Push)
                } else {
                    Err(syn::Error::new_spanned(lit, PUSH_OPTION_ONLY_FOR_VEC))
                }

            },
            _ => Err(syn::Error::new_spanned(lit, INVALID_SET_OPTION))
        }

    }
}
// endregion SetOption

// region SetTagBuilder

#[derive(Debug, Clone, Default)]
pub struct SetTagBuilder {
    pub option: Option<SetTagOptions>,
}

impl SetTagBuilder {
    pub fn build(self, field: &Field) -> syn::Result<SetTag> {
        if let Some(SetTagOptions::Option(lit)) = self.option {
            Ok(SetTag {
                option: SetOption::try_from((&field.ty, lit))?
            })
        } else {
            Ok(SetTag {
                option: SetOption::Assign
            })
        }
    }
}

impl From<TagBuilders> for SetTagBuilder {
    fn from(value: TagBuilders) -> Self {
        match value {
            SetBuilder(builder) => builder,
            _ => unreachable!()
        }
    }
}

impl NamedValueTagBuilder for SetTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![SET]
    }

    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()> {
        let option = SetTagOptions::try_from(meta_value)?;
        self.option = Some(option);
        Ok(())
    }
}

// endregion SetTagBuilder