# Param 

_**Param**_ attribute represents query parameter. It's take the name of the query parameter as argument.

    #[param()]
    

### Usage 

```rust
    struct FindUser {
        #[param("username")]
        pub name: String
    }
````