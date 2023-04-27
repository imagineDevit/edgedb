# Query

        #[query(value)] {
            #[param]
        }

**_#[query]_** is a query macro attribute that takes directly the query string as value parameter.

The argument `value` represents the query string value.

The decorated struct fields represent the query parameters. 

They can be annotated `#[param]` when the field name doesn't match the parameter label.


### Usage 

```rust
    #[query(value=r#"
        insert default::Person {
            name := <str>$name,
            places_visited := (
                insert default::City {
                    name := <str>$city_name,
                }
            )
        }
    "#)]
    pub struct InsertPerson {
        name: String,
        #[param("city_name")]
        city: String
    }
```
Field `city` represents the city's name that is under the query parameter `<str>$city_name`.


