# FileQuery

        #[file_query(src)] {
            #[param]
        }

**_#[file_query]_** is a special query macro attribute that is based on a query source file.

The argument `src` represents the path to the source file (relative to the current working directory). This file should have `.edgeql` extension.

The decorated struct fields represent the query parameters. 

They can be annotated `#[param]` when the field name doesn't match the parameter label.


### Usage 

Consider this `.edgeql` file ( _query.edgeql_ ) located in directory _./queries_ 👇

```
    insert default::Person {
        name := <str>$name,
        places_visited := (
            insert default::City {
                name := <str>$city_name,
            }
        )
    }
```

Based on this file, we can write a file query struct like this 👇

```rust
    #[file_query(src="queries/query.edgeql")]
    pub struct InsertPerson {
        name: String,
        #[param("city_name")]
        city: String
    }
```
Field `city` represents the city's name that is under the query parameter `<str>$city_name`.


