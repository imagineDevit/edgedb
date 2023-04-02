# Query result

    #[query_result]{
        #[field]
        #[backlink]
    }

**_#[query_result]_** attribute marks a struct as a result of a edgeDB query.

When decorating a struct with _**#[query_result]**_ attribute, the resulting struct is decorated with [edgedb_derive::Queryable](https://docs.rs/edgedb-derive/latest/edgedb_derive/derive.Queryable.html) macro derive.

For this reason, a struct decorated _**#[query_result]**_ ⚠️ **must have a field `id: uuid::Uuid`** ⚠️



### Usage 

This is an example of usage of the **_field_** attribute 👇

```rust
     #[query_reqult]
     pub struct UserResult {
        pub id: uuid::Uuid,
        #[field(column_name="pseudo", wrapper_fn="str_upper", default_value="john")]
        pub login: String,
     }

    fn main() {
        let shape = UserResult::shape();
        assert_eq!(shape, "{id, login := (select <str>str_upper(.pseudo)) ?? (select <str>'john')}")
    }
```
<br>

And then, an example of usage of **_backlink_** attribute 👇

Consider the following edgeDB schema :
```sql
    module cinema {
        
        type Movie {
            required property title -> str;
            multi link actors -> Actor;
        }
        
        type Actor {
            required property name -> str;
        }
    }
```

To query the _**Actor**_ _table_ and get the actor's name and all the movies he's been in, we need to write the following query :

```sql
     select Actor
        {
            name,
            movies := (
                select Actor.<actors[is Movie] {
                    title
                }
            )
        }
```

Using **_#[query_result]_** attribute and its **_backlink_** attribute we can do things like this 👇

```rust
    #[query_result]
    pub struct MovieResult {
        pub id: uuid::Uuid,
        pub title: String,
    }
    
    #[query_result]
    pub struct Actor {
        pub id: uuid::Uuid,
        pub name: String,
        #[back_link(
            module="cinema",
            source_table="Actor",
            target_table="Movie",
            target_column="actors"
        )]
        pub movies: Vec<MovieResult>,
    }

    fn main() {
        let rm_spaces = |s: &str| s.split_whitespace().collect::<String>();
        
        let shape = Actor::shape();
        let expected_shape = r#"
            {
                id,
                name, 
                movies := (
                    select cinema::Actor.<actors[is cinema::Movie] {
                       title
                    }
                )
            }
        "#;
        assert_eq!(rm_spaces(shape.as_str()), rm_spaces(expected_shape));
    }
```