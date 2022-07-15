use crate::helpers::attributes::{EdgeDbMeta, Options, Query};
use crate::utils::field_utils::get_struct_fields;
use syn::{DeriveInput, Field};

pub fn start(ast_struct: &DeriveInput) -> (String, Query, bool, Option<Field>, Vec<Field>) {
    // Struct fields
    let fields = get_struct_fields(ast_struct.clone());

    let fields_cloned = fields.clone();

    let table_name = if let Some(table_field) = fields_cloned
        .iter()
        .find(|f| EdgeDbMeta::from_field(f).is_valid())
    {
        EdgeDbMeta::from_field(table_field).value().unwrap()
    } else {
        panic!(
            r#"
            Specify the module's and the table's names 
            by adding an attribute of type () with attribute as follow:
            
            #[edgedb(module = "", table="")]
            __meta__: ()
            
        "#
        );
    };

    let fields_cloned = fields.clone();

    let (query_attr, has_result_type) = if let Some(result_field) = fields_cloned
        .iter()
        .find(|f| Query::from_field(f).has_result())
    {
        (Query::from_field(result_field), true)
    } else {
        (Query::default(), false)
    };

    let options_field = fields
        .clone()
        .into_iter()
        .find(|f| Options::from_field(f).is_some());

    let filtered_fields = fields
        .clone()
        .into_iter()
        .filter(|f| !EdgeDbMeta::from_field(f).is_valid() && Options::from_field(f).is_none())
        .collect::<Vec<Field>>();


    (
        table_name,
        query_attr,
        has_result_type,
        options_field,
        filtered_fields,
    )
}
