
# Fields
Struct fields are mapped to Toql query fields and database columns by default in a predictable way:
1. Table names are UpperCamelCase.
2. Column names are snake_case.
3. Toql query fields are lowerCamelCase.
4. Toql query paths are lowerCamelCase, separated with an underscore.


## Renaming tables and columns
To adjust the default naming to an existing database scheme use the attributes `tables` and `columns` for a renaming scheme or `table` and `column` for explicit name.

Supported renaming schemes are 
- CamelCase
- snake_case
- SHOUTY\_SNAKE\_CASE
- mixedCase

#### Renaming scheme example
```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
use toql::prelude::{Toql, ToqlApi, query, Cache};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(tables="SHOUTY_SNAKE_CASE", columns="CamelCase")]
struct User {
	#[toql(key)]
  	user_id: u32,
	full_name: String,
}

let cache = Cache::default();
let mut toql = MockDb::from(&cache);
let q = query!(User, "*"); 
let mut _users = toql.load_many(&q).await.unwrap(); 
assert_eq!(toql.take_unsafe_sql(), "SELECT user.UserId, user.FullName FROM USER user");
# }
```

#### Explicit naming example
Use `table` on the struct and `column` on fields to set a explicit name.

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
use toql::prelude::{Toql, ToqlApi, query, Cache};
use toql::mock_db::MockDb;
 
#[derive(Toql)]
#[toql(table="User")]
struct UserRef {

	#[toql(key, column="id")]
	user_id: u32,
	full_name: String,
} 
 
let cache = Cache::default();
let mut toql = MockDb::from(&cache);

let q = query!(UserRef, "*"); 
let mut _users = toql.load_many(&q).await.unwrap(); 
assert_eq!(toql.take_unsafe_sql(), "SELECT user.id, user.full_name FROM User user"); 

# }
```

Use `column` also when mapping a field that is a SQL keyword. Notice the back ticks:
```rust, ignore
	#[toql(column="`order`")]
	order: u32
```

### Toql query fields

Toql query fields on a struct are always mixed case, while dependencies are separated with an underscore.

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
use toql::prelude::{Toql, ToqlApi, query, Cache};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(table="User")]
struct UserRef {

	#[toql(key, column="id")]
	id: u32,

	full_name: String,

	#[toql(join)]
	country: Country
}
#[derive(Toql)]
struct Country {

	#[toql(key)]
	id: u32,
	name: String,
}


	let cache = Cache::default();
    let mut toql = MockDb::from(&cache);
  
	let q = query!(UserRef, "id, fullName, country_id"); 
	let mut users = toql.load_many(&q).await.unwrap(); 
	assert_eq!(toql.take_unsafe_sql(), 
			"SELECT user.id, user.full_name, user_country.id, user_country.name \
				FROM User user \
				JOIN (Country user_country) \
				ON (user.country_id = user_country.id)");

# }
```

## Exclusion
Fields can be excluded in several ways
- `skip` excludes a field completely from the table, use for non-db fields.
- `skip_mut` ensures a field is never updated, automatically added for keys and SQL expressions.
- `skip_wildcard` removes a field from default [wildcard selection](../5-query-language/2-select.md), use for expensive SQL expressions or soft hiding.


```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
use toql::prelude::{Toql, ToqlApi, query, Cache};
use toql::mock_db::MockDb;

#[derive(Toql)]
#[toql(table="User")]
struct UserRef {
	
	#[toql(key, column="id")]
	id: u32,

	full_name: String,

	#[toql(skip_wildcard)]
	middle_name: String,

	#[toql(skip)]
	value: String,

}
	let cache = Cache::default();
    let mut toql = MockDb::from(&cache);

	let q = query!(UserRef, "*"); 
	let mut users = toql.load_many(&q).await.unwrap(); 
	assert_eq!(toql.take_unsafe_sql(), "SELECT user.id, user.full_name FROM User user");

#  }
```
