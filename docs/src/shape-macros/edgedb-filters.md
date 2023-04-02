# Edgedb Filters

    #[edgedb_filters]{
        #[field]
        #[filter]
        #[and_filter]
        #[or_filter]
    }

**_#[edgedb_filters]_** attribute is used to decorate a struct that group a list of filters.

The decorated type can be used as type of a query struct field. In this case, the field is decorated with a _**#[filters]**_ attribute.


### Usage 
 
```rust
    #[edgedb_filters]
    pub struct UserFilter {
        #[field(column_name="identity.first_name", param = "first_name")]
        #[filter(operator="=",  wrapper_fn="str_lower")]
        pub name: String,
        #[or_filter(operator=">=")]
        pub age: i8
    }
    
    #[select_query]
    pub struct FindUser {
        #[filters]
        pub filters: UserFilter
    }

```
