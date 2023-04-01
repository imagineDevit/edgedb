# Nested Query 

_**Nested Query**_ attribute indicates that the field type is a query.

    #[nested_query]
    

### Usage 

```rust
    
    struct FindUser {
        #[nested_query]
        pub credentials: FindCredentials
    }
    
    struct FindCredentials {
        ...
    }
````