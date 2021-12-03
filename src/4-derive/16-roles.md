
# Roles
It's possible to restrict access to fields and structs with boolean role expressions.

```rust
use toql::prelude::Toql;

#[derive(Toql)]
#[toql(roles(insert="poweruser", delete="poweruser"))]
struct Book {

	#[toql(key)]
	id : u64,

	#[toql(roles(load="superuser;poweruser", update="poweruser"))]
	rating: u64
}
```
The role expressions are similar to the Toql query syntax:
 - OR is expressed with ;
 - AND is expressed with ,
 - NOT is expressed with !
 - brackets are allowed

An valid role expression would be `(teacher;student), !lazy` meaning `A teacher OR student AND NOT lazy`.

Roles are provided with the context:
```rust
use toql::prelude::ContextBuilder;
use std::collections::HashSet;

let mut r = HashSet::new();
r.insert("teacher".to_string());

let context = ContextBuilder::new()
		.with_roles(r)
		.build();
```
See [here](../3-api/1-introduction.md) for how to get a backend.

Notice that roles can restrict access to columns but not to rows. 
For row access control, check out the [chapter](../6-appendix/3-row-access-control.md) in the appendix.

## Load
It's possible to restrict loading, filtering and ordering of a struct or individual fields, joins and merges.

Let's assume a `struct Book`:

```rust
use toql::prelude::{Toql, Join};
#   #[derive(Toql)]
#   #[toql(auto_key)]
#   struct User {
#   	#[toql(key)]
#   	 id: u32,
#   }
#   
#   #[derive(Toql)]
#   struct Edition {
#   	#[toql(key)]
#   	id: u64,
#   }

#[derive(Toql)]
#[toql(roles(load ="book_role"))]
struct Book {
	#[toql(key)]
	id : u64,

	#[toql(roles(load="author_role"))]
	title: Option<String>,

	#[toql(join, roles(load="author_role"))]
	author: Option<Join<User>>,

	#[toql(merge, roles(load="edition_role"))]
	editions: Option<Vec<Edition>>,
}
```

To load anything from the book, the user requires the `book_role`. 
So to load the book's title the user requires the roles `book_role` and `author_role`.

## Preselected fields
Notice that restricting preselected fields is like restricting the entire struct. See here:

```rust
use toql::prelude::Toql;

#[derive(Toql)]
#[toql(roles(load ="book_role"))]
struct Book {
	#[toql(key)]
	id : u64,

	#[toql(roles(load="author_role"))]
	title: String
}
```
Here Toql needs to load the title field in order to deserialize the struct. Because `title` is role restricted an error is raised for a missing `autor_role`.


## Wildcard behaviour
If all fields from `Book` are selected with a wildcard `*` fields that do not match the role restriction are simply skipped. However, if such a field is selected explictly in the query then an error is raised.

 ## Update
To restrict updating a struct or individual fields:

```rust
use toql::prelude::{Toql, Join};
#   #[derive(Toql)]
#   #[toql(auto_key)]
#   struct User {
#   	#[toql(key)]
#   	 id: u32,
#   }
#   
#   #[derive(Toql)]
#   struct Edition {
#   	#[toql(key)]
#   	id: u64,
#   }

#[derive(Toql)]
#[toql(roles(update="book_role"))]
struct Book {

	#[toql(key)]
	id : u64,

	#[toql(roles(update="author_role"))]
	title: Option<String>,

	#[toql(join, roles(update="author_role"))]
	author: Option<Join<User>>,

	#[toql(merge, roles(update="edition_role"))]
	editions: Option<Vec<Edition>>,
}
```

To update anything on the book, the user requires the `book_role`. 
So to update the book's title the user requires the roles `book_role` and `author_role`.

The role restriction on the `author` join applies only to the foreign key `author_id` in table `Book`, not to the joined `User`.
To restrict the joined `User` put a role restriction `#[toql(roles(update="user_role"))` on top of the struct `User`

Update restrictions on partials joins are not allowed, since partial joins have no foreign key:
Partial joins share the same primary key.

Likewise the role restriction on the `editions` merge applies only to resizing the `Vec` not to the fields on the edition table. 
See the chapter on [updating](../3-api/5-update.md) for details.

## Wildcard behaviour
Fields that have an invalid role expression are skipped for the field list `*`. However if the field name is explicitly mentioned in the field list, then an error occurs. This behaviour is similar to loading.



 ## Insert / Delete
To restrict insertion or deltetion of a struct, attribute the struct like so:

```rust, ignore
use toql::prelude::{Toql, Join};

#[derive(Toql)] {
#[toql(roles(insert="book_role", delete="book_role"))
struct Book {

	#[toql(key)]
	id : u64

	title Option<String>>,

	#[toql(join)]
	author: Option<Join<User>>,

	#[toql(merge)]
	editions: Option<Vec<Edition>>,
}
```









