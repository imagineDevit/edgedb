
use syn::{LitStr, Variant};
use crate::constants::{EXPECT_NON_EMPTY_LIT, VALUE};
use crate::tags::{TagBuilders, ValueTagBuilder};

#[derive(Debug, Clone)]
pub struct ValueTag(pub(crate) String);

#[derive(Debug, Clone, Default)]
pub struct EnumValueTagBuilder {
    pub value: Option<String>
}

impl EnumValueTagBuilder {

    pub fn build(self, variant: &Variant) -> syn::Result<ValueTag> {
        Ok(ValueTag( self.value.unwrap_or(variant.ident.to_string())))
    }
}

impl From<TagBuilders> for EnumValueTagBuilder {
    fn from(builders: TagBuilders) -> Self {
        if let TagBuilders::EnumValue(builder) = builders {
            builder
        } else {
            unreachable!()
        }
    }
}

impl ValueTagBuilder for EnumValueTagBuilder {

    fn tag_names(&self) -> Vec<&str> {
        vec![VALUE]
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