# Keys
Toql requires you to add the attribute `key` to the field that correspond to the primary key in your database.

For composite keys mark multiple fields with the `key` attribute.

For internal reasons keys **must always** be 
- the first fields in a struct
- and must not be optional.

This means that keys are preselected and therefore always loaded 
when a Toql query selects fields from the struct.

#### Example:
```rust
use toql::prelude::Toql;

#[derive(Toql)]
struct User {
  #[toql(key)]
	id: u64,
	name: Option<String>
}
```

## Auto key
If your primary key is generated in your database you can tell this with `[#toql(auto_key)]`. 

Inserts will then ignore the primary key in the struct and instead load the newly generated key from the database.
The new key is then stored in the struct field. 

Notice this can't be used with joined keys.

#### Example:
```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
use toql::prelude::{Toql, ToqlApi,Cache, query, ContextBuilder, paths};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(auto_key)]
struct User {
  #[toql(key)]
  id: u64,
  name: Option<String>
}

let cache = Cache::new();
let mut toql = MockDb::from(&cache);
let mut user = User {id:0, name: Some("Mary".to_string())};

toql.insert_one(&mut user, paths!(top)).await.unwrap(); 

assert_eq!(toql.take_unsafe_sql(), "INSERT INTO User (name) VALUES ('Mary')");
assert_eq!(user.id, 100);
#   }
```


## Joins
*Inner* joins can also have the `key` attribute. This is useful for association tables.

For a join used as a key the SQL builder takes the primary key of the joined struct to guess the foreign key columns.

#### Example:
```rust
use toql::prelude::Toql;

#[derive(Toql)]
struct Language {

  #[toql(key)] 
  code: String,

  name: String
}

#[derive(Toql)]
struct UserLanguage {

  #[toql(key)] 
  user_id: u64,

  #[toql(key, join)]  
  language: Language 
}
```
In the example above Toql assumes that the database table `UserLanguage`  has a composite key made up of the two columns `user_id` and `language_code`. You can change this assumption, see [here](6-joins.md).

## Generated key struct
The Toql derive creates for every struct a corresponding key struct. The key struct contains only the fields marked as key form the derived stuct.

Keys are useful to
  - delete a value with `delete_one`.
  - build a [query](../3-api/2-load.md). 
  - update a [join](6-joins.md).

Keys can be serialized and deserialized with serde, if the `serde` feature is enabled.
This allows web clients to send either a full entity or just the key of it, 
if they want to update some foreign key.


#### Example
```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
#   use toql::prelude::{ToqlApi,Cache, query, ContextBuilder};
#   use toql::mock_db::MockDb;
  use toql::prelude::Toql;

   #[derive(Toql)]
   struct User {
     #[toql(key)]
	    id: u64,
	    name: Option<String>
   }

#   let cache = Cache::new();
#   let mut toql = MockDb::from(&cache);

let key = UserKey::from(10);
toql.delete_one(key).await.unwrap();
# }
```

## Unkeyable fields
Only columns and inner joins can be used as keys. Merged fields (`Vec<T>`) and fields that map to an SQL expression (`#[toql(sql="..")`) cannot be used as keys.
