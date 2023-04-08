use quote::ToTokens;
use syn::{Attribute, Field, LitStr, MetaNameValue, NestedMeta, Variant};
use syn::Meta::{List, NameValue};
use syn::NestedMeta::{Lit, Meta};
use crate::constants::{EXPECT_LIT_OR_NAMED_LIT, EXPECT_LIT_STR, EXPECT_NAMED_LIT, ONLY_ONE_KIND_OF_TAG_EXPECTED, UNLESS_CONFLICT};
use crate::tags::backlink_field_tag::BackLinkFieldTagBuilder;
use crate::tags::field_tag::FieldTagBuilder;
use crate::tags::filter_tag::FilterTagBuilder;
use crate::tags::param_tag::ParamTagBuilder;
use crate::tags::result_field_tag::ResultFieldTagBuilder;
use crate::tags::set_tag::SetTagBuilder;
use crate::utils::path_utils::path_ident_equals;

pub mod field_tag;
pub mod filter_tag;
pub mod set_tag;
pub mod param_tag;
pub mod result_field_tag;
pub mod backlink_field_tag;
pub mod value_tag;
pub mod unless_conflict_tag;
pub mod utils;


pub trait NamedValueTagBuilder {
    fn tag_names(&self) -> Vec<&str>;
    fn arg(&mut self, meta_value: &MetaNameValue) -> syn::Result<()>;
}

pub trait ValueTagBuilder {
    fn tag_names(&self) -> Vec<&str>;
    fn arg(&mut self, lit_str: &LitStr) -> syn::Result<()>;
}

// region TagBuilders
pub enum TagBuilders {
    FieldBuilder(FieldTagBuilder),
    FilterBuilder(FilterTagBuilder),
    SetBuilder(SetTagBuilder),
    ParamBuilder(ParamTagBuilder),
    ResultFieldBuilder(ResultFieldTagBuilder),
    BackLinkFieldBuilder(BackLinkFieldTagBuilder),
    EnumValueBuilder(value_tag::EnumValueTagBuilder),
    UnlessConfictBuilder(unless_conflict_tag::UnlessConflictTagBuilder)
}


impl TagBuilders {
    pub fn arg(&mut self, nested: &NestedMeta) -> syn::Result<()> {
        match nested {
            Lit(syn::Lit::Str(lit)) => {
                match self {
                    TagBuilders::ParamBuilder(builder) => builder.arg(lit),
                    TagBuilders::EnumValueBuilder(builder) => builder.arg(lit),
                    _ => Err(syn::Error::new_spanned(nested, EXPECT_NAMED_LIT))
                }
            }
            Meta(NameValue(ref meta_value)) => {
                match self {
                    TagBuilders::FieldBuilder(builder) => builder.arg(meta_value),
                    TagBuilders::FilterBuilder(builder) => builder.arg(meta_value),
                    TagBuilders::SetBuilder(builder) => builder.arg(meta_value),
                    TagBuilders::ResultFieldBuilder(builder) => builder.arg(meta_value),
                    TagBuilders::BackLinkFieldBuilder(builder) => builder.arg(meta_value),
                    TagBuilders::UnlessConfictBuilder(builder) => builder.arg(meta_value),
                    _ => Err(syn::Error::new_spanned(nested, EXPECT_LIT_STR))
                }
            }
            _ => Err(syn::Error::new_spanned(nested, EXPECT_LIT_OR_NAMED_LIT))
        }
    }

    pub fn tag_names(&self) -> Vec<&str> {
        match self {
            TagBuilders::FieldBuilder(builder) => builder.tag_names(),
            TagBuilders::FilterBuilder(builder) => builder.tag_names(),
            TagBuilders::SetBuilder(builder) => builder.tag_names(),
            TagBuilders::ParamBuilder(builder) => builder.tag_names(),
            TagBuilders::ResultFieldBuilder(builder) => builder.tag_names(),
            TagBuilders::BackLinkFieldBuilder(builder) => builder.tag_names(),
            TagBuilders::EnumValueBuilder(builder) => builder.tag_names(),
            TagBuilders::UnlessConfictBuilder(builder) => builder.tag_names()
        }
    }
}

// endregion TagBuilders

pub enum Tagged {
    StructField(Field),
    EnumVariant(Variant),
}

impl Tagged {
    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Tagged::StructField(f) => &f.attrs,
            Tagged::EnumVariant(v) => &v.attrs
        }
    }

    pub fn tokens(&self) -> Box<dyn ToTokens + '_> {
        match self {
            Tagged::StructField(f) => Box::new(f),
            Tagged::EnumVariant(v) =>Box::new(v)
        }
    }
}

pub fn build_tags_from_field(tagged: &Tagged, builders: Vec<&mut TagBuilders>) -> syn::Result<()> {

    let build_fn = |att: &Attribute, builder: &mut TagBuilders| -> syn::Result<()> {
        let meta = att.parse_meta()?;
        match meta {
            List(list) => {
                for ref nested in list.nested {
                    builder.arg(nested)?
                }
            }
            _ => {
                if let Some((true, _)) = path_ident_equals(&att.path, UNLESS_CONFLICT) {
                    return Ok(())
                }
                return Err(syn::Error::new_spanned(meta, EXPECT_NAMED_LIT));
            }
        }

        Ok(())
    };

    for builder in builders.into_iter(){

        let mut atts = tagged.attrs().iter()
            .filter(|att| builder.tag_names().contains(&att.path.get_ident().unwrap().to_string().as_str()));

        if atts.clone().count() > 1 {
            return Err(syn::Error::new_spanned(tagged.tokens().as_ref(), format!("{ONLY_ONE_KIND_OF_TAG_EXPECTED}, {:#?}", builder.tag_names())));
        }

        let att: Option<&Attribute> = atts.next();

        if let Some(att) = att {
            build_fn(att, builder)?;
        }
    }

    Ok(())
}
