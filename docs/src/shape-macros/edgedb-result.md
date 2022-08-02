# EdgedbResult

### List of attributes

<table>
    <thead>
        <tr>
            <th rowspan="2">Attributes</th>
            <th rowspan="2">Optional</th>
            <th rowspan="2">Description</th>
            <th colspan="3">Options</th>
        </tr>
    <tr>
            <th>Name</th>
            <th>Optional</th>
            <th>Description</th>
    </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="3"> <strong style="color: #008200">field</strong> </td>
            <td rowspan="3"> Yes </td>
            <td rowspan="3"> The <strong style="color: #91b362">field</strong> attribute represents the edgedb query result field. </td>
            <td><i style="color: yellow">column_name</i></td>
            <td>Yes</td>
            <td>The column_name in the egdedb table. It's defined when the struct field do not match the column name.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">wrapper_fn</i></td>
            <td>Yes</td>
            <td>The <i style="color: #d4c89f">wrapper_fn</i> option is the edgedb function to apply to the field.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">default_value</i></td>
            <td>Yes</td>
            <td>The default value to return when the column value is an empty set.</td>
        </tr>
        <tr>
            <td rowspan="5"> <strong style="color: #008200">back_link</strong> </td>
            <td rowspan="5"> Yes </td>
            <td rowspan="5"> 
                The <strong style="color: #91b362">back_link</strong> attribute is used when the field value is a result of a backlink.
            </td>
            <td><i style="color: yellow">module</i></td>
            <td>Yes</td>
            <td>The edgedb schema module</td>
        </tr>
        <tr>
             <td><i style="color: yellow">source_table</i></td>
            <td>No</td>
            <td>The query source edgedb table name.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">target_table</i></td>
            <td>No</td>
            <td>The name of table targeted by the backlink.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">target_column</i></td>
            <td>No</td>
            <td>The name of the targeted_table column targeted by the backlink.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">result</i></td>
            <td>No</td>
            <td>The backlink expected result type.</td>
        </tr>
    </tbody>
</table>
<br><br>

This is an example of usage of the **_field_** attribute 👇

```rust
     #[derive(EdgedbResult)]
     pub struct UserResult {
         #[field(column_name="pseudo", wrapper_fn="str_upper", default_value="john")]
         pub login: String,
     }

    fn main() {
        let shape = UserResult::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.pseudo)) ?? (select <str>'john')}")
    }
```
<br>

And then, an example of usage of **_backlink_** attribute 👇

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
Considering the edgedb module above 👆
<br><br>
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

Using **_EdgedbResult_** derive and it's **_backlink_** attribute we can do things like this 👇

```rust
    #[derive(EdgedbResult)]
    pub struct MovieResult {
        pub title: String,
    }
    
    #[derive(EdgedbResult)]
    pub struct Actor {
        pub name: String,
        #[back_link(
            module="cinema",
            source_table="Actor",
            target_table="Movie",
            target_column="actors",
            result="MovieResult"
        )]
        pub movies: Vec<MovieResult>,
    }

    fn main() {
        let rm_spaces = |s: &str| s.split_whitespace().collect::<String>();
        
        let shape = Actor::shape();
        let expected_shape = r#"
            {
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