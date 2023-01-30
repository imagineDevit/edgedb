# InsertQuery

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
                The field decorated <i style="color: #91b362">#[meta]</i> represents the query metadata 
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
            <td rowspan=3> <strong style="color: #008200">scalar</strong> </td>
            <td rowspan=3>yes</td>
            <td rowspan=3>This attribute is used to give information about the field type</td>
            <td><i style="color: yellow">type</i></td>
            <td>No</td>
            <td>The field scalar type name. If the type is an <i>enum</i>, the type is equal "enum"</td>
        </tr>
        <tr>
            <td><i style="color: yellow">module</i></td>
            <td>Yes ("default")</td>
            <td>The scalar type module </td>
        </tr>
         <tr>
            <td><i style="color: yellow">name</i></td>
            <td>No when type = "enum"</td>
            <td>The enum name </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">param</strong> </td>
            <td> Yes </td>
            <td colspan="4"> 
            The <strong style="color: #91b362">param</strong> attribute represents the query parameter label associated to the decorated field. </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">nested_query</strong> </td>
            <td>yes</td>
            <td colspan="4">This attribute indicates that the field references an edgedb <strong>select</strong> or <strong>insert</strong> query</td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">unless_conflict</strong> </td>
            <td>yes</td>
            <td colspan="4">
                This attribute is used to declare the <strong style="color: #b21e00">unless conflict on ... else</strong> clause of the query.
                This attribute must be used on field of type <strong style="color: #2b79a2">edgedb_query::queries::Conflict::UnlessConflictElse </strong>
            </td>
        </tr>
    </tbody>
</table>
<br>


**InsertQuery** derive macro generates a insert edgeql query.

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
        
       
            
        type Address {
            required property num -> int16;
            required property street -> str;
            required property city -> str;
            required property zipcode -> int32;
        }
   }
```

To perform a insert query using [edgedb-tokio](https://github.com/edgedb/edgedb-rust) we can write code as follows 👇

```rust

#[derive(EdgedbEnum)]
pub enum Sex {
    Male,
    Female,
}

#[derive(InsertQuery)]
pub struct InsertPerson {
    #[meta(module = "models", table = "Person")]
    #[result("Person")]
    __meta__: (),

    pub user_name: String,

    pub age: i8,

    #[scalar(type = "enum", module = "models", name = "Gender")]
    pub gender: Sex,

    #[nested_query]
    pub address: InsertAddress,
}

#[derive(InsertQuery)]
pub struct InsertAddress {
    #[meta(module = "models", table = "Address")]
    __meta__: (),
    pub num: i16,
    pub street: String,
    pub city: String,
    pub zipcode: i32,
}

#[derive(EdgedbResult)]
pub struct Person {
    pub user_name: String,
    pub age: i8
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let client = edgedb_tokio::create_client().await?;

    let insert_person = InsertPerson {
        __meta__: (),

        user_name: "Joe".to_owned(),
        age: 35,
        gender: Sex::Male,
        address: InsertAddress {
            __meta__: (),
            
            num: 12,
            street: "rust street".to_owned(),
            city: "rust City".to_owned(),
            zipcode: 4567
        }
    };
    
    let edge_query: EdgeQuery  = insert_person.to_edge_query();

    let args = &edge_query.args.unwrap();

    let query = edge_query.query.as_str();
    
    if let Some(json) = client.query_single_json(query, args).await? {
        if let Ok(result) = serde_json:from_str::<Person>(json.as_ref()) {
            assert_eq!(result.user_name, "Joe");
            assert_eq!(result.age, 35);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}
```
