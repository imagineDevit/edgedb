use syn::{Field, LitStr};
use crate::constants::{EXPECT_NON_EMPTY_LIT, PARAM};
use crate::tags::{TagBuilders, ValueTagBuilder};
use crate::utils::field_utils::get_field_ident;

#[derive(Debug, Clone)]
pub struct ParamTag(pub(crate) String);

#[derive(Debug, Clone, Default)]
pub struct ParamTagBuilder {
    pub value: Option<String>
}

impl ParamTagBuilder {

    pub fn build(self, field: &Field) -> syn::Result<ParamTag> {
        Ok(ParamTag(self.value.unwrap_or(get_field_ident(field).to_string())))
    }
}

impl From<TagBuilders> for ParamTagBuilder {
    fn from(builders: TagBuilders) -> Self {
        if let TagBuilders::ParamBuilder(builder) = builders {
            builder
        } else {
            unreachable!()
        }
    }
}

impl ValueTagBuilder for ParamTagBuilder {
    fn tag_names(&self) -> Vec<&str> {
        vec![PARAM]
    }

    fn arg(&mut self, lit_str: &LitStr) -> syn::Result<()> {
        let value = lit_str.value();
        if value.is_empty() {
            Err(syn::Error::new_spanned(lit_str, EXPECT_NON_EMPTY_LIT))
        } else {
            self.value = Some(value);
            Ok(())
        }
    }
}