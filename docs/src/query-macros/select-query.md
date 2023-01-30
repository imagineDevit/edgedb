# SelectQuery

#### List of possibles attributes :

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
            <td rowspan=2> <strong style="color: #008200">meta</strong> </td>
            <td rowspan=2>No</td>
            <td rowspan=2>
                The field annotated <i style="color: #91b362">#[meta]</i> represents the query metadata 
                and is used just to declare module and table names.
                This field must be (called __meta__, of type () ) : <br>
                <strong style="color: #c82829">__meta__ : ()</strong>
            </td>
            <td><i style="color: yellow">module</i></td>
            <td>Yes ("default") </td>
            <td>The edgedb module name </td>
        </tr>
        <tr>
            <td><i style="color: yellow">table</i></td>
            <td>No</td>
            <td>The edgedb table name </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">result</strong> </td>
            <td>yes</td>
            <td colspan="4">This attribute must also decorate the field called  <strong style="color: #c82829">__meta__ : () </strong>. <br> It's used to declare the query result type.
            The struct representing the expected result shape and must be decorated with <strong><a href="../shape-macros/edgedb-result.html">#[derive(EdgedbResult)]</a></strong> </td>
        </tr>
        <tr>
            <td rowspan="4"> <strong style="color: #008200">filter</strong> </td>
            <td rowspan="4"> Yes </td>
            <td rowspan="4"> The <strong style="color: #91b362">filter</strong> attribute represents a filter statement in a edgedb select query. </td>
            <td><i style="color: yellow">operator</i></td>
            <td>No</td>
            <td>
                The filter operator that can takes following values :
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
                Conjunctive operator that can be:
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
            <td> <strong style="color: #008200">filters</strong> </td>
            <td>Yes</td>
            <td colspan="4"><i style="color: #91b362">filters</i> attribute is used to combine all the filters in a unique annotated <strong style="color: #9d00ec">#[derive(EdgedbFilters)]</strong> <br> see <a href="../shape-macros/edgedb-filters.html"> EdgedbFilters</a></td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">options</strong> </td>
            <td>Yes</td>
            <td colspan="4"><i style="color: #91b362">options</i> attribute represents additional select options :
                <ul>
                    <li>order options</li>
                    <li>pagination options.</li>
                </ul> 
            Only a field of type <i style="color:#ff7733"> edgedb_query::queries::select::SelectOptions</i> can be decorated  <i style="color: #91b362">#[options]</i>
            </td>
        </tr>
    </tbody>
</table>
<br>


**SelectQuery** derive macro generates a select edgeql query.

Let's see how to use it:

Consider the following edgeDB schema 👇

```sql
   module models {
        scalar type Gender extending enum<Male, Female>
        
       type Person {
            required property user_name -> str;
            required property age -> int16;
            required property gender -> Gender;
            link address -> Address;
        }
   }
```

To perform a select query using [edgedb-tokio](https://github.com/edgedb/edgedb-rust) we can write code as follows 👇

```rust

#[derive(EdgedbEnum)]
pub enum Sex {
    Male,
    Female,
}

#[derive(EdgedbFilters)]
pub struct SelectPersonFilters {
    #[filter(operator = "Like", column_name="user_name")]
    pub name: String,

    #[filter(operator = "<=", conjunctive="And")]
    pub age: u16,
}

#[derive(SelectQuery)]
pub struct SelectPerson {
    #[meta(module = "models", table = "Person")]
    #[result("Person")]
    __meta__: (),

    #[filters]
    filters: SelectPersonFilters,

    #[options]
    options: SelectOptions<'static>
}

#[derive(EdgedbResult, Deserialize)]
pub struct Person {
    pub user_name: String,
    pub age: i8
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = edgedb_tokio::create_client().await?;

    let select_person = SelectPerson {
        __meta__: (),
        filters: SelectPersonFilters {
            name: "%oe".to_owned(),
            age: 18,
        },
        options: SelectOptions {
            table_name: "Person",
            module: Some("models"),
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc)
            }),
            page_options: None
        }
    };

    let edge_query: EdgeQuery = select_person.to_edge_query();

    let args = &edge_query.args.unwrap();

    let query = edge_query.query.as_str();

    if let Some(json) = client.query_json(query, args).await? {
        if let Ok(persons) = serde_json: from_str::<Vec<Person>>(json.as_ref()) {
            assert!(persons.len() > 0 );
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}
```
