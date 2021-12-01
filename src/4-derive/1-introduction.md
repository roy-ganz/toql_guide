# The Toql derive
A struct must derive `Toql`. Only on a derived struct any function from the [ToqlApi](../3-api/1-introduction.md) can be called.

This derive builds _a lot_ of code. This includes

- Mapping of Toql fields to struct fields and database columns or expressions.
- Creating field methods for the query builder.
- Handling relationships through joins and merges.
- Creating Key structs.


## Example

With this simple code

 ```rust
 	use toql::prelude::Toql;
	#[derive(Toql)]
	struct User {
		#[toql(key)]
		id: u32,
		name: Option<String>
}
```

We can now do the following

```rust
 	use toql::prelude::{query, ToqlApi, Cache, Toql, Page, fields};
    use toql::mock_db::MockDb;
	use toql::row;

#    #[derive(Toql)]
#    struct User {
#      #[toql(key)]
#      id: u64,
#      age: Option<u8>
#    }

#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
    
	let cache = Cache::default();
    let mut toql = MockDb::from(&cache);
	toql.mock_rows("SELECT user.id, user.age FROM User user WHERE user.id = 5", vec![row!(5u64, 27u64)]);
  
	let q = query!(User, "id eq 5, age"); 
	let mut user = toql.load_one(&q).await.unwrap(); 

	user.age = Some(16);
	toql.update_one(&mut user, fields!(top)).await.unwrap(); 
# }
```
