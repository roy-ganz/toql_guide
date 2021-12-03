# SQL expressions
Toql is an SQL friendly ORM. Instead of mapping a struct field to a column you can also map it
to a raw SQL expression. There are small syntax enhancements to work with aliases and auxiliary parameters.

#### Alias axample

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
    use toql::prelude::{Toql, ToqlApi, query, Cache};
    use toql::mock_db::MockDb;

    #[derive(Toql)]
    struct User {
        #[toql(key)]
        id: u64,

        #[toql(sql="(SELECT COUNT(*) FROM Books b WHERE b.author_id = ..id)")]
        number_of_book:u64
    }

    let cache = Cache::default();
    let mut toql = MockDb::from(&cache);

	let q = query!(User, "id"); 
	let mut _users = toql.load_many(&q).await.unwrap(); 
	assert_eq!(toql.take_unsafe_sql(), 
			"SELECT user.id, (SELECT COUNT(*) FROM Books b WHERE b.author_id = user.id) \
            FROM User user");
#   }
```

Notice the `..` ! This special alias will be replaced with the alias created for _User_.

To use aux params in a SQL query use the `<param_name>` syntax. 

#### Aux params example

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
    use toql::prelude::{Toql, ToqlApi, query, Cache, ContextBuilder};
    use toql::mock_db::MockDb;
    use std::collections::HashMap;

    #[derive(Toql)]
    struct User {
        #[toql(key)]
        id: u64,

        #[toql(sql="(SELECT <page_limit>)")]
        page_limit:u64,

        #[toql(sql="(SELECT COUNT(*) FROM Films f WHERE f.age >= <age>)")]
        age_rated_films:u64
    }
 
    let mut p = HashMap::new();
    p.insert("page_limit".into(), 200.into());
    p.insert("age".into(), 57.into());

    let context = ContextBuilder::new().with_aux_params(p).build();
    let cache = Cache::new();
    let mut toql = MockDb::with_context(&cache, context);

	let q = query!(User, "id"); 
	let mut _users = toql.load_many(&q).await.unwrap(); 
	assert_eq!(toql.take_unsafe_sql(), 
			"SELECT user.id, (SELECT 200), (SELECT COUNT(*) FROM Films f WHERE f.age >= 57) FROM User user");
#   }
```
In the example *page_limit* might come from a server configuration. 
It would typically be put in the [context](../3-api/1-introduction.md) and can be used in SQL expressions.

The aux param *age* might be taken from the authorisation token and put as an aux param into the context or query. 
Here it restricts the number of films.

## Other uses of raw SQL
There are other places you can use raw SQL:
 - [Predicates](14-predicates.md)
 - [Custom Merge](9-merges.md)