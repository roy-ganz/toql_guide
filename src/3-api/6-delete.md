## Deletes

There are two delete functions: `delete_one`, and `delete_many`. 

Both functions take a predicate and delete all rows that match that predicate. 

`delete_one` takes a key. It will build a filter predicate from that and delete the row that coresponds to the key.

`delete_many` builds a predicate from the filters of the `Query` argument. Field filters and predicates and considered, whereas field selects and selections in the query are ignored. See the [query language](5-query-lanuage/1-introduction.md) for details.

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main(){
use toql::prelude::{ToqlApi, Cache, Toql, query, Keyed};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(auto_key)]
struct User{
    #[toql(key)]
    id: u64,
    name: String,
}

let cache = Cache::new();
let mut toql = MockDb::from(&cache);

toql.delete_one(UserKey::from(5)).await.unwrap();
assert_eq!(toql.take_unsafe_sql(), "DELETE user FROM User user WHERE user.id = 5");

let u = User {id: 5, name: "John".to_string()};
toql.delete_one(u.key()).await.unwrap();
assert_eq!(toql.take_unsafe_sql(), "DELETE user FROM User user WHERE user.id = 5");

toql.delete_many(query!(User, "id eq 5")).await.unwrap();
assert_eq!(toql.take_unsafe_sql(), "DELETE user FROM User user WHERE user.id = 5");
# }
```

### Cascading
`delete` does not do any cascading by itself. It just deletes rows from a _single_ table. 
To cascade your deletes you must configure your database relations 
and tell the database what to do with your joined rows: Delete them too or just set the foreign key to NULL.

Check the manual for 
- [MySQL](https://dev.mysql.com/doc/refman/8.0/en/create-table-foreign-keys.html) 
- [Postgres](https://www.postgresql.org/docs/8.2/ddl-constraints.html#DDL-CONSTRAINTS-FK)
