
// region other
pub const SPACE : &str = " ";
pub const SCALAR_TYPE: &str = "$scalar_type$";
pub const EDGEQL: &str = "$edgeql$";
pub const INF_SIGN: &str = "<";
pub const SUP_SIGN: &str = ">";
// endregion other

// region wrapper
pub const OPTION: &str = "Option";
pub const VEC: &str = "Vec";
// endregion

// region query types
pub const INSERT: &str = "insert";
pub const SELECT: &str = "select";
pub const DELETE: &str = "delete";
pub const UPDATE: &str = "update";
// endregion query types

// region metadata
pub const DEFAULT_MODULE : &str = "default";
pub const MODULE: &str = "module";
pub const TABLE: &str = "table";
pub const RESULT: &str = "result";
pub const SRC: &str = "src";
// endregion metadata

// region tags
pub const SCALAR: &str = "scalar";
pub const SET: &str = "set";
pub const SETS: &str = "sets";
pub const OPTIONS: &str = "options";

pub const FILTERS: &str = "filters";
pub const FILTER: &str = "filter";
pub const AND_FILTER: &str = "and_filter";
pub const OR_FILTER: &str = "or_filter";
pub const OPERATOR: &str = "operator";

pub const VALUE: &str = "value";
pub const BACKLINK: &str = "back_link";
pub const TARGET_TABLE: &str = "target_table";
pub const SOURCE_TABLE: &str = "source_table";
pub const TARGET_COLUMN: &str = "target_column";

pub const FIELD: &str = "field";
pub const WRAPPER_FN: &str = "wrapper_fn";
pub const COLUMN_NAME: &str = "column_name";
pub const DEFAULT_VALUE: &str = "default_value";
pub const SET_OPTION: &str = "option";

pub const NESTED_QUERY: &str = "nested_query";

pub const LIMIT_1: &str = " limit 1";

pub const UNLESS_CONFLICT: &str = "unless_conflict";

pub const PARAM: &str = "param";

pub const AND: &str = " and";
pub const OR: &str = " or";

//endregion tags

// region operators
pub const EXISTS: &str = "exists";
pub const NOT_EXISTS: &str = "notexists";
pub const BANG_EXISTS: &str = "!exists";
pub const IS: &str = "is";
pub const EQUAL: &str = "=";
pub const IS_NOT: &str = "isnot";
pub const NOT_EQUAL: &str = "!=";
pub const LIKE: &str = "like";
pub const ILIKE: &str = "ilike";
pub const STRING: &str = "String";
pub const IN: &str = "in";
pub const NOT_IN: &str = "notin";
pub const GREATER_THAN: &str = "greaterthan";
pub const GREATER_THAN_OR_EQUAL: &str = "greaterthanorequal";
pub const SUP_OR_EQ_SIGN: &str = ">=";
pub const LESSER_THAN: &str = "lesserthan";
pub const LESSER_THAN_OR_EQUAL: &str = "lesserthanorequal";
pub const INF_OR_EQ_SIGN: &str = "<=";
// endregion operators

//region setOption

pub const ASSIGN: &str = "assign";
pub const ASSIGN_SIGN: &str = ":=";
pub const CONCAT: &str = "concat";
pub const CONCAT_SIGN: &str = "++";
pub const PUSH: &str = "push";
pub const PUSH_SIGN: &str = "+=";
// endregion setOption

// region patterns
pub const PARAM_PATTERN: &str = "\\B(\\$\\w+)\\b";

// endregion patterns

// region messages

pub const EXPECT_META: &str = "Expected a meta attribute";
pub const EXPECT_NON_EMPTY_LIT: &str= "Expected a non-empty string literal";
pub const EXPECT_LIT: &str = "Expected a string literal";
pub const EXPECT_TABLE: &str = "Expected a table name";
pub const EXPECT_SRC: &str = "Expected a src value";
pub const EXPECT_OPERATOR: &str = "Expected filter operator attribute `#[filter(operator = \"...\")]`";
pub const UNSUPPORTED_ATTRIBUTE: &str = "Unsupported attribute";
pub const FIRST_FILTER_EXPECTED: &str = "Expected first filter attribute `#[filter(...)]`";
pub const AND_OR_FILTER_EXPECTED: &str = "Expected `and` or `or` filter attribute `#[and_filter(...)]` or `#[or_filter(...)]`";

pub const ONLY_ONE_KIND_OF_TAG_EXPECTED: &str = "Only one of the following tags is expected";

pub const INVALID_INSERT_TAG: &str = r#"
    Invalid insert field tag.
    Expected "field", "nested_query" or "unless_conflict"
"#;
pub const INVALID_FIELD_TAG: &str = r#"
    Invalid field tag option.
    Expected "column_name" , "param" or "scalar"
"#;

pub const INVALID_BACKLINK_TAG: &str = r#"
    Invalid backlink tag option.
    Expected "module" , "source_table", "target_table", "target_column" or "result"
"#;

pub const INVALID_SELECT_TAG: &str = r#"
    Invalid select field tag.
    Expected "filter", "and_filter", "or_filter", "filters" or "options"
"#;

pub const INVALID_UPDATE_TAG: &str = r#"
    Invalid update field tag.
    Expected "filter", "and_filter", "or_filter", "filters", "set" or "sets"
"#;

pub const INVALID_FILTER_TAG: &str = r#"
    Invalid filter tag option.
    Expected "operator", or "wrapper_fn"
"#;

pub const INVALID_SET_OPTION: &str = r#"
    Invalid set option.
    Expected "assign", "=", "concat", "++" , "push" or "+="
"#;

pub const INVALID_SET_TAG_OPTION: &str = r#"
    Invalid set tag option.
    Expected "option"
"#;

pub const INVALID_RESULT_FIELD_TAG: &str = r#"
    Invalid result's field tag.
    Expected "field", "back_link" or nothing
"#;

pub const INVALID_ENUM_VARIANT_TAG: &str = r#"
    Invalid enum's variant tag.
    Expected "value"
"#;

pub const INVALID_DELETE_TAG: &str = r#"
    Invalid delete field tag.
    Expected "filter", "and_filter", "or_filter"or "filters"
"#;

pub const INVALID_FILTERS_TAG: &str = r#"
    Invalid filters field tag.
    Expected "filter", "and_filter" or "or_filter"
"#;


pub const INVALID_SETS_TAG: &str = r#"
    Invalid sets tag.
    Expected "set"
"#;

pub const PUSH_OPTION_ONLY_FOR_VEC: &str = "Push option only accepts a Vec type";

pub const EXPECT_LIT_OR_NAMED_LIT: &str = "Expected a literal or a named string literal";
pub const EXPECT_NAMED_LIT: &str = "Expected a named string literal";
pub const EXPECT_LIT_STR: &str = "Expected a string literal";

pub const INVALID_TYPE_TUPLE_FOR_OPERATOR: &str = "Invalid type () for operator";

pub const ONLY_TYPE_FOR_OPERATOR: &str = " operator only accepts a type ";

pub const INVALID_OPERATOR: &str = r#"
    Invalid operator.
    Expected "Exists", "NotExists", "Is", "IsNot", "Like", "ILike", "In", "NotIn", "GreaterThan", "GreaterThanOrEqual", "LesserThan", "LesserThanOrEqual"
 "#;


pub const EXPECTED_ONLY_TAGS: &str = "Expected only the following tags";
pub const ONLY_ONE_OPTIONS_TAG_EXPECTED: &str = "SelectQuery can only have one options field";
pub const ONLY_ONE_FILTERS_TAG_EXPECTED: &str = "SelectQuery can only have one filters field";
pub const ONLY_ONE_SETS_TAG_EXPECTED: &str = "UpdateQuery can only have one sets field";
pub const EITHER_ONE_FILTERS_OR_FILTER_TAG_EXPECTED: &str = "SelectQuery can only have either one `filters` or one or more `filter` fields";
pub const EITHER_ONE_SETS_OR_SET_TAG_EXPECTED: &str = "UpdateQuery can only have either one `sets` or one or more `set` or `nested_query` fields";
pub const EXPECTED_AT_LEAST_ONE_SET_FIELD: &str = "UpdateQuery must have at least one field with #[set] attribute or with no attribute";
// endregion messages

// region types

pub const BASIC_RESULT : &str = "BasicResult";

// region types


pub const __TABLENAME__ : &str = "__tablename__";