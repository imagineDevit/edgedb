# EdgedbSet

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
            <td rowspan="2"> <strong style="color: #008200">field</strong> </td>
            <td rowspan="2"> Yes </td>
            <td rowspan="2"> The <strong style="color: #91b362">field</strong> attribute represents the edgedb set field declaration. </td>
            <td><i style="color: yellow">column_name</i></td>
            <td>Yes</td>
            <td>The column_name in the egdedb table. It's defined when the struct field do not match the column name.</td>
        </tr>
        <tr>
             <td><i style="color: yellow">assignment</i></td>
            <td>Yes</td>
            <td>
                The <i style="color: #d4c89f">assignment</i> option corresponds to the value to field assignment type.
                This option can take 3 values:
                <ul>
                    <li><i style="color:#ff7733;">Assign</i> <br> It's the defautl value, that corresponds to basic assignment </li>
                    <li><i style="color:#ff7733;">Concat</i> <br> used to concatenate and corresponds to symbol <strong style="color:#9d00ec;"> ++ </strong></li>
                    <li><i style="color:#ff7733;">Push</i> <br> used to push a new value to a vec and corresponds to symbol <strong style="color:#9d00ec;"> += </strong></li>
                </ul>
            </td>
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
            The <strong style="color: #91b362">param</strong> attribute represents the query parameter label associated to the annotated field. </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">nested_query</strong> </td>
            <td>yes</td>
            <td colspan="4">This attribute indicates that the field references an edgedb <strong>select</strong> or <strong>insert</strong> query</td>
        </tr>
    </tbody>
</table>
<br><br>

This is an usage example  👇

```rust
      #[derive(EdgedbSet)]
      pub struct MySet {
          #[field(column_name="first_name", assignment = "Concat")]
          #[scalar(type="str")]
          pub name: String,
          #[scalar(type="enum", name="State", module="default")]
          pub status: Status
      }
      
      #[derive(EdgedbEnum)]
      pub enum Status {
          Open, Closed
      }

    fn main() {
        let set = MySet {
            name: "Joe".to_owned(),
            status: Status::Open
        };

        assert_eq!(r#"
           set { 
                first_name := .first_name ++ (select <str>$name), 
                status := (select <default::State>$status)
           }
        "#, set.to_edgeql());
    }
```