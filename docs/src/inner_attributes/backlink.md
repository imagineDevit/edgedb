# Back Link 

_**Back Link**_ attribute is used in a query result struct. It indicates that the field's value is the result of a [backlink](https://www.edgedb.com/docs/edgeql/select#backlinks).

      #[back_link(module, source_table, target_table, target_column)]

<br>

| Argument      | Optional | Description                                                |
|---------------|----------|------------------------------------------------------------|
| module        | yes      | The edgeDB module. <br/> <br/>**_By default_** : 'default' |
| source_table  | no       | The backlink source table name                             |
| target_table  | no       | The backlink target table name                             |
| target_column | no       | The backlink target column name                            |

<br>


### Usage 

```rust
    struct UserResult {
       #[back_link(
            module="users",
            source_table="User",
            target_table="Friend",
            target_column="friend"
        )]
        friend: Friend,
    }
````