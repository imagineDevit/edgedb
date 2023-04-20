//! Edgedb-query-derive provide a bunch procedural macros in order to facilitate writing of queries when using edgedb-rust crate.

extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::insert_query::InsertQuery;
use crate::{meta_data::{TableInfo, QueryMetaData, }, queries::Query};
use crate::delete_query::DeleteQuery;
use crate::edgedb_enum::EdgedbEnum;
use crate::edgedb_filters::EdgedbFilters;
use crate::edgedb_sets::EdgedbSets;
use crate::file_query::FileQuery;
use crate::meta_data::{SrcFile, SrcValue};
use crate::query_result::QueryResult;
use crate::select_query::SelectQuery;
use crate::update_query::UpdateQuery;

mod constants;
mod utils;
mod queries;
mod tags;
mod statements;
mod insert_query;
mod select_query;
mod update_query;
mod delete_query;
mod builders;
mod meta_data;
mod file_query;
mod query_result;
mod edgedb_enum;
mod edgedb_filters;
mod edgedb_sets;

/// Create an insert edgeDB query
///
/// ## Usage
///
/// ```rust
///     use edgedb_query::{ToEdgeQuery, EdgeQuery};
///     use edgedb_query::queries::conflict::{UnlessConflictElse, Conflict};
///     use edgedb_query_derive::{
///             insert_query,
///             select_query,
///             query_result,
///             edgedb_enum
///     };
///
///     #[insert_query(module ="users", table="User", result="UserResult")]
///     pub struct InsertUser {
///         #[field(param="first_name")]
///         pub name: String,
///         pub surname: Option<String>,
///         pub age: i32,
///         pub major: bool,
///         pub vs: Vec<String>,
///         #[field(scalar = "<users::Gender>")]
///         pub gender: Sex,
///         #[nested_query]
///         pub wallet: Wallet,
///         #[unless_conflict(on="username, surname")]
///         pub find_user: UnlessConflictElse<FindUser>
///     }
///
///     #[edgedb_enum]
///     pub enum Sex {
///         #[value("male")]
///         Male,
///         #[value("female")]
///         _Female,
///     }
///
///     #[insert_query(module = "users", table = "Wallet")]
///     pub struct Wallet {
///         pub money: i16,
///     }
///
///     #[select_query(module = "users", table = "User")]
///     pub struct FindUser {
///         #[filter(operator="Is")]
///         #[field(column_name="name")]
///         pub user_name: String
///     }
///
///     #[query_result]
///     pub struct UserResult {
///         pub id: uuid::Uuid,
///         pub name: String,
///     }
///
///     async fn main() {
///         let insert_user = InsertUser {
///             name: "Joe".to_string(),
///             surname: Some("Henri".to_string()),
///             age: 35,
///             major: true,
///             vs: vec!["vs1".to_string()],
///             gender: Sex::Male,
///             wallet: Wallet {
///                 money: 0,
///             },
///             find_user: UnlessConflictElse {
///                 else_query: FindUser{
///                     user_name: "Joe".to_string(),
///                 },
///             }
///         };
///
///         let query  = insert_user.to_edge_query();
///
///         let client = edgedb_tokio::create_client().await.unwrap();
///
///         let user: UserResult = client
///                     .query_single::<UserResult, _>(query.query.as_str(), &query.args.unwrap())
///                     .await
///                     .unwrap();
///
///     }
/// ```
#[proc_macro_attribute]
pub fn insert_query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as QueryMetaData);

    parse_macro_input!(item as InsertQuery)
        .with_meta(meta)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create a select edgeDB query
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::{edgedb_filters, query_result, select_query};
///     use edgedb_query::models::edge_query::{ToEdgeQuery, EdgeQuery};
///     use edgedb_query::queries::select::{OrderDir, OrderOptions, SelectOptions};
///
///     #[select_query(module = "users", table = "User", result = "UserResult")]
///     pub struct SelectQuery {
///         #[filter(operator = "Is")]
///         pub name: String,
///
///         #[and_filter(operator = "GreaterThan")]
///         pub age: i8,
///
///         #[options]
///         options: SelectOptions
///     }
///
///     #[query_result]
///     pub struct UserResult {
///         pub id: uuid::Uuid,
///         pub name: String,
///         pub age: i8,
///     }
///
///     async fn main() {
///          let client = edgedb_tokio::create_client().await.unwrap();
///
///          let select_query =  SelectQuery {
///             options: SelectOptions {
///                 order_options: Some(OrderOptions {
///                     order_by: "name".to_string(),
///                     order_direction: Some(OrderDir::Desc)
///                 }),
///                 page_options: None
///             },
///             name:  "Joe".to_string(),
///             age: 18
///         };
///
///         let query = select_query.to_edge_query();
///
///         let user: UserResult = client
///                 .query_single::<UserResult, _>(query.query.as_str(), &query.args.unwrap())
///                 .await
///                 .unwrap();
///     }
/// ```
///
#[proc_macro_attribute]
pub fn select_query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as QueryMetaData);

    parse_macro_input!(item as SelectQuery)
        .with_meta(meta)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create an update edgeDB query
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::{update_query};
///     use edgedb_query::BasicResult;
///     use edgedb_query::models::edge_query::ToEdgeQuery;
///
///     #[update_query(module = "users", table = "User")]
///     pub struct UpdateUser {
///          pub name: String,
///
///          #[filter(operator = "=", wrapper_fn = "str_lower")]
///          #[field(column_name = "identity.first_name")]
///          pub first_name: String,
///
///          #[and_filter(operator = ">=")]
///          pub age: i8,
///      }
///
///     async fn main() {
///         let client = edgedb_tokio::create_client().await.unwrap();
///
///         let update_query = UpdateUser {
///             name: "Joe".to_string(),
///             first_name: "Henri".to_string(),
///             age: 18,
///         };
///
///         let query = update_query.to_edge_query();
///
///         let result: BasicResult = client
///                 .query_single::<BasicResult, _>(query.query.as_str(), &query.args.unwrap())
///                 .await
///                 .unwrap();
///     }
/// ```
#[proc_macro_attribute]
pub fn update_query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as QueryMetaData);

    parse_macro_input!(item as UpdateQuery)
        .with_meta(meta)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create a delete edgeDB query
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::{delete_query};
///     use edgedb_query::models::edge_query::ToEdgeQuery;
///
///     #[delete_query(module ="users", table="User")]
///     pub struct DeleteUsersByAge {
///
///         #[filter(operator="=")]
///         pub age: i16
///     }
///
///     async fn main() {
///          let client = edgedb_tokio::create_client().await.unwrap();
///          let delete_query = DeleteUsersByAge {
///             age: 18,
///         };
///
///         let query = delete_query.to_edge_query();
///
///         let _ = client
///                 .query_single_json(query.query.as_str(), &query.args.unwrap())
///                 .await
///                 .unwrap();
///     }
/// ```
#[proc_macro_attribute]
pub fn delete_query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as TableInfo);

    parse_macro_input!(item as DeleteQuery)
        .with_meta(meta)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create an edgeDB query based on a source file
///
/// ## Usage
///
///
/// _queries.edgeql_
/// ``` sql
///     insert users::User {
///         name := <str>$user_name,
///         age := <int16>$age,
///         friend := (
///             select users::User {
///                 name,
///                 age,
///             }
///             filter .name = <str>$friend_name
///         )
///     }
/// ```
/// ```rust
///     use edgedb_query_derive::{file_query};
///     use edgedb_query::BasicResult;
///     use edgedb_query::models::edge_query::ToEdgeQuery;
///
///     #[file_query(src="queries.edgeql")]
///     pub struct AddUser {
///         #[param("user_name")]
///         pub name: String,
///         pub age: i8,
///         #[param("friend_name")]
///         pub friend: String,
///     }
///
///     async fn main() {
///
///         let client = edgedb_tokio::create_client().await.unwrap();
///
///         let add_user = AddUser {
///             name: "Joe".to_string(),
///             age: 18,
///             friend: "Henri".to_string(),
///         };
///
///         let query = add_user.to_edge_query();
///
///         let result = client
///                 .query_single::<BasicResult, _>(query.query.as_str(), &query.args.unwrap())
///                 .await
///                 .unwrap();
///     }
/// ```
#[proc_macro_attribute]
pub fn file_query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as SrcFile);

    parse_macro_input!(item as FileQuery<SrcFile>)
        .with_meta(meta)
        .validate()
        .and_then(|q| q.to_token_stream())
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create an edgeDB query based on a source file
///
/// ## Usage

/// ```rust
///     use edgedb_query_derive::{query};
///     use edgedb_query::BasicResult;
///     use edgedb_query::models::edge_query::ToEdgeQuery;
///
///     #[query(value=r#"
///         insert users::User {
///             name := <str>$user_name,
///             age := <int16>$age,
///             friend := (
///                 select users::User {
///                     name,
///                     age,
///                 }
///             filter .name = <str>$friend_name
///             )
///         }"#
///     )]
///     pub struct AddUser {
///         #[param("user_name")]
///         pub name: String,
///         pub age: i8,
///         #[param("friend_name")]
///         pub friend: String,
///     }
///
///     async fn main() {
///
///         let client = edgedb_tokio::create_client().await.unwrap();
///
///         let add_user = AddUser {
///             name: "Joe".to_string(),
///             age: 18,
///             friend: "Henri".to_string(),
///         };
///
///         let query = add_user.to_edge_query();
///
///         let result = client
///                 .query_single::<BasicResult, _>(query.query.as_str(), &query.args.unwrap())
///                 .await
///                 .unwrap();
///     }
/// ```
#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {

    let meta = parse_macro_input!(attr as SrcValue);

    parse_macro_input!(item as FileQuery<SrcValue>)
        .with_meta(meta)
        .validate()
        .and_then(|q| q.to_token_stream())
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Represents a query result
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::query_result;
///
///     #[query_result]
///     pub struct UserWithFriendAndFieldAndWrapperFn {
///         pub id: uuid::Uuid,
///         #[field(column_name="pseudo", wrapper_fn="str_upper", default_value="john")]
///         pub login: String,
///         pub identity: Identity,
///         #[back_link(
///             module="users",
///             source_table="User",
///             target_table="Friend",
///             target_column="friend"
///         )]
///         pub friend: Friend,
///     }
/// ```
#[proc_macro_attribute]
pub fn query_result(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as QueryResult)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Represents an edgeDB enum type
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::edgedb_enum;
///
///     #[edgedb_enum]
///     pub enum Sex {
///         #[value("man")]
///         Male,
///         #[value("woman")]
///         Female,
///     }
/// ```
#[proc_macro_attribute]
pub fn edgedb_enum(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as EdgedbEnum)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Represents a list of edgeDB query filters
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::edgedb_filters;
///
///     #[edgedb_filters]
///     pub struct MyFilter {
///         #[field(column_name="identity.first_name", param = "first_name")]
///         #[filter(operator="=",  wrapper_fn="str_lower")]
///         pub name: String,
///         #[or_filter(operator=">=")]
///         pub age: i8
///     }
/// ```
#[proc_macro_attribute]
pub fn edgedb_filters(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as EdgedbFilters)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Create an list of update edgeDB query sets
///
/// ## Usage
///
/// ```rust
///     use edgedb_query_derive::edgedb_sets;
///
///
///     #[edgedb_sets]
///     pub struct MySet {
///         #[field(column_name="first_name", param = "user_name", scalar="<str>")]
///         #[set(option="Concat")]
///         pub name: String,
///         #[field(scalar="default::State")]
///         pub status: Status,
///         #[nested_query]
///         pub users: FindUsers
///     }
/// ```
#[proc_macro_attribute]
pub fn edgedb_sets(_: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(item as EdgedbSets)
        .to_token_stream()
        .unwrap_or_else(|e| e.to_compile_error().into())
}
