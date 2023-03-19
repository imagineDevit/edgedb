use std::convert::TryFrom;
use std::io::{ErrorKind, Read};

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Ident, parse::{Parse, ParseStream}, Token};

use crate::constants::{BASIC_RESULT, DEFAULT_MODULE, EXPECT_LIT, EXPECT_META, EXPECT_NON_EMPTY_LIT, EXPECT_SRC, EXPECT_TABLE, MODULE, RESULT, SRC, TABLE, UNSUPPORTED_ATTRIBUTE};

macro_rules! add_meta {
    ($param_name: ident, $param_value: ident, $builder: ident, $with_result: ident, $with_src: ident) => {
        let value = $param_value.clone();
         match $param_value {
             syn::Lit::Str(s) => {
                 if s.value().is_empty() {
                     return Err(syn::Error::new_spanned(
                         value.to_token_stream(),
                         EXPECT_NON_EMPTY_LIT
                     ));
                 } else {
                    $builder.arg(DataType::try_from(($param_name, $with_result, $with_src))?, s.value());
                 }
             },
             _ => {
                 return Err(syn::Error::new_spanned(
                     $param_value.to_token_stream(),
                     EXPECT_LIT
                 ));
             }
         }
    }
}

trait Builder {
    type T;

    fn arg(&mut self, meta: DataType, value: String);

    fn build(self) -> syn::Result<Self::T>;
}

// region DataType
#[derive(Debug)]
enum DataType {
    Module,
    Table,
    Result,
    Src
}

impl TryFrom<(Ident, bool, bool)> for DataType {
    type Error = syn::Error;

    fn try_from((value, with_result, with_src): (Ident, bool, bool)) -> Result<Self, Self::Error> {

        match value.to_string().as_str() {
            MODULE => Ok(DataType::Module),
            TABLE => Ok(DataType::Table),
            RESULT => {
                if with_result {
                    Ok(DataType::Result)
                } else {
                    Err(syn::Error::new_spanned(value.to_token_stream(), format!("{UNSUPPORTED_ATTRIBUTE} `{value}`")))
                }
            },
            SRC => {
                if with_src {
                    Ok(DataType::Src)
                } else {
                    Err(syn::Error::new_spanned(value.to_token_stream(), format!("{UNSUPPORTED_ATTRIBUTE} `{value}`")))
                }
            },
            _ => Err(syn::Error::new_spanned(value.to_token_stream(), format!("{UNSUPPORTED_ATTRIBUTE} `{value}`")))
        }
    }
}
// endregion DataType

// region TableInfo
#[derive(Debug, Clone)]
pub struct TableInfo {
    pub module: String,
    pub table: String,
}

impl TableInfo {
    pub fn table_name(&self) -> String {
        format!("{}::{}", self.module, self.table)
    }
}

impl Parse for TableInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut builder = TableInfoBuilder::default();

        loop {
            if !input.peek(Ident) { break; }

            let param_name = input.parse::<Ident>()?;

            input.parse::<Token![=]>()?;

            let param_value = input.parse::<syn::Lit>()?;

            add_meta!(param_name, param_value, builder, false, false);

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        builder.build()
    }
}
// endregion TableInfo

// region TableInfoBuilder
#[derive(Default)]
pub struct TableInfoBuilder {
    pub module: Option<String>,
    pub table: Option<String>,
}

impl Builder for TableInfoBuilder {
    type T = TableInfo;

    fn arg(&mut self, meta: DataType, value: String) {
        match meta {
            DataType::Module => self.module = Some(value),
            DataType::Table => self.table = Some(value),
            _ => {}
        }
    }

    fn build(self) -> syn::Result<Self::T> {
        if let Some(table) = self.table {
            Ok(TableInfo {
                module: self.module.unwrap_or(DEFAULT_MODULE.to_owned()),
                table,
            })
        } else {
            Err(syn::Error::new_spanned(
                TABLE.to_token_stream(),
                EXPECT_TABLE,
            ))
        }
    }
}

// endregion TableInfoBuilder

// region QueryMetaData

#[derive(Debug, Clone)]
pub struct QueryMetaData {
    pub meta: TableInfo,
    pub result: Option<String>,
}

impl QueryMetaData {
    pub fn table_name(&self) -> String {
        self.meta.table_name()
    }

    pub fn result_quote(&self) -> proc_macro2::TokenStream {
        if let Some(result) = &self.result {
            let tty = Ident::new(result.as_str(), Span::call_site());
            quote! {
                use edgedb_query::ToEdgeShape;

                let shape = #tty::shape();
                query.push_str(shape.as_str());
            }
        } else {
            quote!()
        }
    }

    pub fn result(&self) -> Ident {
        self.result.clone()
            .or(Some(BASIC_RESULT.to_string()))
            .map(|r| Ident::new(r.as_str(), Span::call_site()))
            .unwrap()
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }
}

impl Parse for QueryMetaData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut builder = QueryMetaDataBuilder::default();

        loop {
            if !input.peek(Ident) { break; }

            let param_name = input.parse::<Ident>()?;

            input.parse::<Token![=]>()?;

            let param_value = input.parse::<syn::Lit>()?;

            add_meta!(param_name, param_value, builder, true, false);

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        builder.build()
    }
}

// endregion QueryMetaData

// region QueryMetaDataBuilder
#[derive(Default)]
pub struct QueryMetaDataBuilder {
    pub meta_builder: TableInfoBuilder,
    pub result: Option<String>,
}

impl Builder for QueryMetaDataBuilder {
    type T = QueryMetaData;

    fn arg(&mut self, meta: DataType, value: String) {
        match meta {
            DataType::Module | DataType::Table => self.meta_builder.arg(meta, value) ,
            DataType::Result => self.result = Some(value),
            _ => {}
        }
    }

    fn build(self) -> syn::Result<Self::T> {
        Ok(QueryMetaData {
            meta: self.meta_builder.build()?,
            result: self.result,
        })
    }
}

// endregion QueryMetaDataBuilder

// region SrcFile
#[derive(Debug, Clone)]
pub struct SrcFile(pub(crate) String);

impl SrcFile {
    pub fn get_content(&self, ident: &Ident) -> syn::Result<String> {

        let mut s = String::default();

        match std::env::current_dir() {
            Ok(mut dir) => {
                dir.push(self.0.clone());
                let path = dir.as_path();
                match std::fs::File::open(path) {
                    Ok(mut file) => {
                        match file.read_to_string(&mut s) {
                            Ok(_) => {
                                if s.is_empty() {
                                    Err(syn::Error::new_spanned(ident.to_token_stream(), "Source file content cannot be empty" ))
                                } else {
                                    Ok(s)
                                }
                            },
                            Err(_) => Err(syn::Error::new_spanned(ident.to_token_stream(), "Cannot read source file" ))
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::NotFound {
                            Err(syn::Error::new_spanned(
                                ident.to_token_stream(),
                                format!("Source file '{}' is not found", path.to_str().unwrap())
                            ))
                        } else {
                            Err(syn::Error::new_spanned(
                                ident.to_token_stream(),
                                e.to_string()
                            ))
                        }

                    }
                }
            }
            Err(_) => Err(syn::Error::new_spanned(
                ident.to_token_stream(),
                String::from("Failed to retrieve current project dir ")))
        }
    }
}

impl Parse for SrcFile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut builder = SrcFileBuilder::default();

        loop {
            if !input.peek(Ident) { break; }

            let param_name = input.parse::<Ident>()?;

            input.parse::<Token![=]>()?;

            let param_value = input.parse::<syn::Lit>()?;

            add_meta!(param_name, param_value, builder, false, true);

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        builder.build()
    }
}


// endregion SrcFile

// region SrcFileBuilder
#[derive(Default)]
pub struct SrcFileBuilder {
    pub src: Option<String>
}

impl Builder for SrcFileBuilder {
    type T = SrcFile;

    fn arg(&mut self, meta: DataType, value: String) {
        if let DataType::Src = meta {
            self.src = Some(value)
        }
    }

    fn build(self) -> syn::Result<Self::T> {
        if let Some(src) = self.src {
            Ok(SrcFile(src))
        } else {
            Err(syn::Error::new_spanned(
                SRC.to_token_stream(),
                EXPECT_SRC,
            ))
        }
    }
}

// endregion SrcFileBuilder

// region Common functions

pub fn try_get_meta<F, R>(struct_name: &Ident, get_meta: F) -> syn::Result<R>
    where F: FnOnce() -> Option<R> {
    match get_meta() {
        Some(meta) => Ok(meta),
        None => Err(syn::Error::new_spanned(struct_name.to_token_stream(), EXPECT_META))
    }
}

// endregion Common functions