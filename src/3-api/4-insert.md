## Inserts

There are two insert functions: `insert_one`, and `insert_many`. 

The are used like so:

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main(){
use toql::prelude::{Cache, Toql, ToqlApi, paths};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(auto_key)]
struct User{
    #[toql(key)]
    id: u64,
    name: String,
    address: Option<String>
}

let cache = Cache::new();
let mut toql = MockDb::from(&cache);

let mut u = User { 
                id:0, 
                name: "Joe".to_string(), 
                address: None
            };
toql.insert_one(&mut u, paths!(top)).await.unwrap();
assert_eq!(toql.take_unsafe_sql(), "INSERT INTO User (name, address) VALUES ('Joe', DEFAULT)");
assert_eq!(u.id, 100);

toql.insert_one(&mut u, paths!(User, "")).await.unwrap();
toql.insert_many::<User,_>(&mut [&mut u], paths!(top)).await.unwrap();
#   }
```

In the example above the first `insert_one` will insert `u` into the database, 
load back the generated id and sets it on `u`. 
The second `insert_one` makes a copy of `u` and again refreshes its `id` field.

Optional fields that are `None` will insert the default value of the database. See [the mapping](../4-derive/11-insert.md) for details.


### The paths! macro
The `paths!` macro compiles a path list. Any invalid path will show up at compile time. The insert functions insert all referenced joins and merges from such a path list.

Let's assume a struct `User` that has a `Vec` of `books` with each book joining a `publisher`. A path list like `paths!(User, "books_publisher")` will 
- insert all fields from `User`.
- if the `id` of `User` is marked as [auto key](../4-derive/10-keys.md), Toql will load the generated `id` from the database and update `User` and the composite key in `books`.
- insert all fields from the joined `publisher`.
- handle the `id` of publisher like the one in `books`.
- insert the merged books with the correct composite keys.

Cool stuff!

If you only want to insert a `publisher`, then you must call `insert` on a `publisher` object.

The paths in the list may overlap. In the path list `paths!(User, "books_publisher, books_printedBy")` `book` appears twice. No problem it will only be inserted once.


### Skipping values
When inserting a struct some fields are excluded:
- Fields that map to an [SQL expression](../4-derive/4-sql-expressions.md)
- Fields or structs with the atribute `skip_mut`
- Joins with custom ON predicate
- Merges with custom [sql_join](../4-derive/9-merges.md)


### Partial tables
It is possible to split up a table into multiple tables sharing the same primary key. See [here](../4-derive/8-partial-tables).

If a path in the path list refers to a struct that contains joins marked as `partial table` then these
joins will also be inserted. There is no need to mention these dependencies in the path list.


### Key dependencies
The order of SQL execution is based on key dependencies between structs. 







 







