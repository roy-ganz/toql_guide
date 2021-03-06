
## The `Query` type
All load functions need a query, but how is this build?

The recommended way is to use the `query!` macro.

Alternatives are 
- to create a new `Query<T>` object and use its builder methods
- or to parse a string

This chapter does not explain the Toql query language itself, see [here](../5-query-language/1-introduction.md) to learn about that.


### The `query!` macro 
The `query!` macro will compile the provided string into Rust code. Any syntax mistakes, wrong path or field names show up 
as compiler errors! 

Queries are typesafe, so `query!` takes a type and a query expression. See here:

```rust
use toql::prelude::{query, Toql};

#[derive(Toql)]
struct User {
    #[toql(key)]
    id: u64,
    name: String
}

let user_id = 5;
let q = query!(User, "*, id eq ?",  user_id);
```
 
To include query parameters just insert a question mark in the query string and provide the parameter after the string. 

In the example above it would also be possible to put the number 5 directly into the query string, since it's a constant. 
The resulting SQL would be the same as Toql extracts the parameter in either case to prevent SQL injections.

The Toql query only works with numbers and strings, see `SqlArg`. 
However this is not be a problem: Since database columns have a type, the database is able convert a string or number into its column type.

It's also possible to include other queries into a query. Consider this:

```rust
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }
use toql::prelude::query;
let q1 = query!(User, "id eq ?", 5);
let q = query!(User, "*, {}", q1);
```

Here we include the query `q1` into `q`. Since queries are typesafe, so you can only include queries of the same type.

### Working with keys

When entities have composite keys or you want to write generic code it's easier to work with keys. Key structs are automatically derived from the `Toql` derive and are located where the struct is. Keys contain all fields from the struct that are marked with `#[toql(key)]`.

With a single key this is possible
```rust
use toql::prelude::{query, Query};
# use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }

let k = UserKey::from(5); // Easier than UserKey{id:5};
let q1 = query!(User, "id eq ?", &k);
let q2 = query!(User, "*, {}", Query::from(&k));
let q3 = query!(User, "*, {}", k);
```

With multiple keys you can do this:
```rust
use toql::prelude::{query, Query};
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }

let ks = vec![UserKey::from(1), UserKey::from(2)];

let q4 = query!(User, "*, id in ?", &ks);

let qk = ks.iter().collect::<Query<_>>();
let q5 = query!(User, "*, {}", qk);
```

The query `q4` only works for a simple key, not a composite key, whereas `qk` works for any type of key.

If you deal with entities you can get their keys from them (notice the `Keyed` trait):

```rust
use toql::prelude::{query, Keyed, Query};
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }
let e = User{id:1, name: "User 1".to_string()};

let q5 =  query!(User, "{}", e.key());
let q6 =  Query::from(e.key());
```

Both `q5` and`q6` end up the same.

Or with mutiple entities:

```rust
use toql::prelude::{query, MapKey, Query};
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }

let es = vec![ User{ id:1, name:"User 1".to_string()}, 
                User{ id:2, name: "User 2".to_string()}];

let qk = es.iter().map_key().collect::<Query<_>>();
let q7 = query!(User, "*, {}", qk);
```

Do you like the `collect` style? There is a nifty implementation detail:
If you collect keys, they will always be concatenated with *OR*, queries however will be concatenated with *AND*.

Compare `q8` and `q10` here:
```rust
use toql::prelude::{query, Query};
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }

let ks = vec![UserKey{id:5}, UserKey{id:6}];
let q8 :Query<User> = ks.into_iter().collect(); 
assert_eq!(q8.to_string(), "(id EQ 5;id EQ 6)");

let q9 = query!(User, "name");
let q10 :Query<User> = vec![q9, q8].into_iter().collect(); 
assert_eq!(q10.to_string(), "name,(id EQ 5;id EQ 6)");

```


### The `Into<Query>` trait

In the example above the query `q3` is build with a `UserKey`. This is possible because `UserKey` implements `Into<Query<User>>`.
You can also implement this trait for you own types. Let's assume a book category.

#### Example 1: Adding an enum filter to the query
```rust
use toql::prelude::{query, Query, Toql};

#[derive(Toql)]
struct Book {
    #[toql(key)]
    id: u64,
    category: Option<String>
}

enum BookCategory {
    Novel,
    Cartoon
}
impl Into<Query<Book>> for BookCategory {
    fn into(self) -> Query<Book> {
       query!(Book, "category EQ ?", 
       match self {
        Novel => "NOVEL",
        Cartoon => "CARTOON"    
       })
    }
}

// Now use it like so
let q = query!(Book, "*, {}", BookCategory::Novel);
assert_eq!(q.to_string(), "*,category EQ 'NOVEL'");
```



#### Example 2: Adding an authorization filter to the query

```rust
use toql::prelude::{QueryWith, Query, Field, QueryFields, Toql};
   
#[derive(Toql)]
struct Book {
    #[toql(key)]
    id: u64,
    category: Option<String>,
    #[toql(join)]
    author: Option<User>
}
#[derive(Toql)]
struct User {
    #[toql(key)]
    id: u64,
    name: Option<String>,
}

struct Auth {
    user_id: u64
}
impl Into<Query<Book>> for Auth {
    fn into(self) -> Query<Book> {

        // This time with the builder methods for educational reasons
        // In production do `query!(User, "author_id eq ?", self.user_id)`
        Query::from(Book::fields().author().id().eq(self.user_id))
    }
}
```

You may want trade typesafety for more flexibility. See the example above again, this time with the `Field` type.

```rust
use toql::prelude::{Query, Field};

struct Auth {
    author_id: u64
}

impl<T> Into<Query<T>> for Auth {
    fn into(self) -> Query<T>{
        Query::from(Field::from("authorId").eq(self.author_id))
    }
}
```
Wrong field names in `Field::from` do not show up at compile time, but at runtime.

You can use both examples like so:

```rust
use toql::prelude::query;
#   use toql::prelude::{Query, Field, Toql};
#   enum BookCategory {
#       Novel,
#       Cartoon
#   }
#   #[derive(Toql)]
#   struct Book {
#       #[toql(key)]
#       id: u64,
#       category: Option<String>
#   }
#   struct Auth {
#       author_id: u64
#   }
#   impl<T> Into<Query<T>> for Auth {
#       fn into(self) -> Query<T>{
#           Query::from(Field::from("authorId").eq(self.author_id))
#       }
#   }

let auth = Auth {author_id: 5};
let q = query!(Book, "*, {}", auth);
assert_eq!(q.to_string(), "*,authorId EQ 5");
```


### The `QueryWith` trait

The `query!` macro produces a `Query` type and can further be altered using all methods from that type.
One interesting method is `with`. It takes a `QueryWith` trait that can be implemented for any custom type to enhance the query. 
This is more powerful than `Into<Query>` because you can also access auxiliary parameters.

Aux params can be used in SQL expressions. See [here](../4-derive/4-sql-expressions.md) more information.

```rust
use toql::prelude::{QueryWith, Query};

struct Config {
    limit_pages: u64
}
impl<T> QueryWith<T> for Config {
    fn with(&self, query: Query<T>) -> Query<T> {
        query.aux_param("limit_pages", self.limit_pages)
    }
}
```

This can now be used like so:

```rust
use toql::prelude::{query, SqlArg};
#   use toql::prelude::{Toql, Query, QueryWith};
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }
#   struct Config {
#       limit_pages: u64
#   }
#   impl<T> QueryWith<T> for Config {
#       fn with(&self, query: Query<T>) -> Query<T> {
#           query.aux_param("limit_pages", self.limit_pages)
#       }
#   }

let config = Config {limit_pages: 200};
let k = UserKey::from(5);
let q = query!(User, "*, {}", k).with(config);
assert_eq!(q.to_string(), "*,id EQ 5");
assert_eq!(q.aux_params.get("limit_pages"), Some(&SqlArg::U64(200)));
```

### Parsing queries
Use the query parser to turn a string into a `Query` type. 
Only syntax errors will returns as errors, 
wrong field names or paths will be rejected later when using the query.

```rust
use toql::prelude::QueryParser;
#   use toql::prelude::Toql;
#   #[derive(Toql)]
#   struct User {
#       #[toql(key)]
#       id: u64,
#       name: String
#   }
let s = "*, id eq 5";

let q = QueryParser::parse::<User>(s).unwrap();
assert_eq!(q.to_string(), "*,id EQ 5");
```






