# EdgedbFilters

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
            <td rowspan="4"> <strong style="color: #008200">filter</strong> </td>
            <td rowspan="4"> No </td>
            <td rowspan="4"> The <strong style="color: #91b362">filter</strong> attribute represents a filter statement in a edgedb select query. </td>
            <td><i style="color: yellow">operator</i></td>
            <td>No</td>
            <td>
                The <i>operator</i> option that can takes following values :
                <ul>
                    <li>Exists</li>
                    <li>NotExists</li>
                    <li>Is ( or <strong>=</strong> )</li>
                    <li>IsNot ( or <strong>!=</strong> )</li>
                    <li>Like</li>
                    <li>ILike</li>
                    <li>In</li>
                    <li>NotIn</li>
                    <li>GreaterThan ( or <strong> > </strong> )</li>
                    <li>GreaterThanOrEqual ( or <strong> >= </strong> )</li>
                    <li>LessThan ( or <strong> < </strong> )</li>
                    <li>LessThanOrEqual ( or <strong> <= </strong> )</li>
                </ul>
            </td>
        </tr>
        <tr>
            <td><i style="color: yellow">conjunctive</i></td>
            <td>No (if it's not the first filter) </td>
            <td>
                The <i>conjunctive</i> option can be:
                <ul>
                    <li>And</li>
                    <li>Or</li>
                </ul>
            </td>
        </tr>
        <tr>
            <td><i style="color: yellow">column_name</i></td>
            <td>Yes</td>
            <td>The column_name on which the filter is applied. Only if the column name differs from the rust struct field name.</td>
        </tr>
        <tr>
            <td><i style="color: yellow">wrapper_fn</i></td>
            <td>Yes</td>
            <td>The edgedb function to apply to the column value while filtering.</td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">param</strong> </td>
            <td> Yes </td>
            <td colspan="4"> 
            The <strong style="color: #91b362">param</strong> attribute represents the query parameter label associated to the annotated field. </td>
        </tr>
    </tbody>
</table>
<br><br>


When applying **EdgedbFilters** derive to a struct, the **_edgedb_query::queries::filter::Filter_** trait implementation is generated for this one.

```rust
    pub trait Filter {
        fn to_edgeql(&self, table_name: &str) -> String;
        fn to_edge_value(&self) -> edgedb_protocol::value::Value;
    }
```
- _**to_edgeql()**_ returns the filter statement.
- _**to_edge_value()**_ returns the statement arguments values
<br>

Consider the following struct 👇 
```rust

    #[derive(EdgedbFilters)]
    pub struct UserFilter {
        #[filter(operator="=", column_name="identity.first_name", wrapper_fn="str_lower")]
        #[param("username")]
        pub name: String,
        #[filter(operator=">=",  conjunctive="And")]
        pub age: i8,
    }
```

```rust
    fn main() {
        let filter = UserFilter {
            name: "Joe".to_string(),
            age: 18
        };
    
        let query = filter.to_edgeql("users::User");
    
        let value: Value = filter.to_edge_value();

        println!("QUERY -> {:#?}", query);
        println!("VALUE -> {:#?}", value);
    }
```

#### output :
```
⌲ QUERY -> "filter str_lower(users::User.identity.first_name) = (select <str>$username) and users::User.age >= (select <int16>$age)"
```
```
⌲ VALUE -> Object {
    shape: ObjectShape(
        ObjectShapeInfo {
            elements: [
                ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: Some(
                        One,
                    ),
                    name: "username",
                },
                ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: Some(
                        One,
                    ),
                    name: "age",
                },
            ],
        },
    ),
    fields: [
        Some(
            Str(
                "Joe",
            ),
        ), 
        Some(
            Int16(
                18,
            ),
        ),
    ],
}
```
