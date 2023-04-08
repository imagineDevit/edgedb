# UpdateQuery

        #[update_query(module, table, result)] {
            #[field]
            #[set]
            #[sets]
            #[filter]
            #[and_filter]
            #[or_filter]
            #[filters]
        }

**_update_query_** attribute macro indicates that the struct represents an edgeDB update query.

Each field of UpdateQuery can be decorated with following tags:
- [#[field]](../inner_attributes/field.md)
- [#[filter] (#[and_filter] or #[or_filter])](../inner_attributes/filter.md)
- [#[filters]](../shape-macros/edgedb-filters.md)
- [#[set]](../inner_attributes/set.md)
- [#[sets]](../shape-macros/edgedb-sets.md)



### ⚠️ 
 - #[filter] (#[and_filter] or #[or_filter]) and #[filters] can not be used to together.
 - #[set] and #[sets] can not be used to together.
 - A field not decorated with #[filter] (#[and_filter] or #[or_filter]) is considered to be a #[set] field.
---

### Usage

Consider the following edgeDB schema 👇

```sql
    module models { 
           type Person {
                required property user_name -> str;
                required property age -> int16;
           }
    }
```
To perform an update query using edgedb-tokio we can write code as follows 👇

```rust

    #[update_query(module="models", table="Person")]
    pub struct UpdatePerson {
        #[field(column_name="user_name", param="new_name")]
        pub name: String,
        
        #[field(column_name="user_name", param="searched_name")]
        #[filter(operator="like")]
        pub filter: String
    }
    
    #[tokio::main]
    async fn main() -> Result<()> {
        let client = edgedb_tokio::create_client().await?;
        
        let update_person = UpdatePerson {
            name: "Mark".to_owned() ,
            filter: "%oe".to_owned()
        };
    
        let edge_query: EdgeQuery = update_person.to_edge_query_with_cardinality(Cardinality::One);
    
        let args = &edge_query.args.unwrap();
    
        let query = edge_query.query.as_str();
    
        if let Some(result) = client.query_required_single::<BasicResult, _>(query, args).await? {
            assert_ne!(result.id.to_string(), String::default())
        } else {
            unreachable!()
        }
        
    }
```
