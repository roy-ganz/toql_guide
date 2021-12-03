# Insert
When you insert a struct, all fields, including foreign keys of joins, will be inserted. 

Check [the API](../3-api/4-insert.md) for details.

## Default values
For Selectable fields in a struct that are `None` the default value will be inserted.
If you have not defined a default value in your database you must ensure that the field in the struct can't be `None`. 
This can be done through prior validation.


## Insert behaviour 
The insert behaviour depends on the type amd mapping of a field:

```rust
use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct Phone {
#   
#   	#[toql(key)]
#   	number: String,
#   	prepaid : Option<bool>
#   }
#   
#   #[derive(Toql)]
#   struct Address {
#   	#[toql(key)]
#   	id: u64,
#   	street: Option<String>
#   }
  
#[derive(Toql)]
struct User {
	#[toql(key)]
	id: u64,                    
	
	username: String,		// Value
	realname: Option<String>,	// Default or value
	address: Option<Option<String>>,// Nullable column: Default, value or NULL

	#[toql(preselect)]
	info: Option<String>, 	// Nullable column: Value or NULL

	#[toql(join())]
	address1: Option<Address>, 	// Selectable inner Join: Foreign key is inserted or default

	#[toql(join())]
	address2: Option<Option<Address>>,// Selectable left join: Default, value or NULL

	#[toql(join())]
	address3: Address, 		// Inner Join: Foreign key or default

	#[toql(join(), preselect)]
	address4: Option<Address>,	// Selectable inner join: Foreign key or default

	#[toql(merge())]
	phones1: Vec<Phone>,		// No change on table 'User'

	#[toql(merge())]
	phones2: Option<Vec<Phone>> // No change on table 'User'
}
```

When the path list requires to insert a dependency too, 
left joins and optional merges will only be inserted, if they contains a value.

 



