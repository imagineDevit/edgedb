#Options 

_**Options**_ attribute marks a field as a select option (order and pagination options). The decorated fied must be of type [edgedb_query::queries::select::SelectOptions](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/queries/select.rs)

    #[options]
    

### Usage 

```rust
    #[select_query(table="Users")]
    struct FindUser {
        #[options]
        pub options: SelectOptions
    }
````