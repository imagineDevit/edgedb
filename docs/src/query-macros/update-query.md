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
    
        let edge_query: EdgeQuery = update_person.to_edge_query();
    
        let args = &edge_query.args.unwrap();
    
        let query = edge_query.query.as_str();
    
        if let Some(json) = client.query_json(query, args).await? {
            if let Ok(result) = serde_json: from_str::<Vec<BasicResult>>(json.as_ref()) {
                assert!(persons.len() > 0 );
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        
    }
```
