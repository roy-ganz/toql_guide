## Updates

There are two update functions: `update_one` and `update_many`. 

They are used like so:

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main(){
use toql::prelude::{Cache, Toql, ToqlApi, fields};
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

let mut user = User {id:27, name: "Susan".to_string(), address: None};

toql.update_one(&mut user, fields!(top)).await.unwrap();
assert_eq!(toql.take_unsafe_sql(), "UPDATE User SET name = \'Susan\' WHERE id = 27");

toql.update_one(&mut user, fields!(User, "*")).await.unwrap();

toql.update_many::<User, _>(&mut [&mut user], fields!(top)).await.unwrap();
toql.update_many(&mut [user], fields!(top)).await.unwrap();
#   }
```

In the example above all four statements do the same. 


### The fields! macro
The `fields!` macro compiles a list of fields. Any invalid path or field name shows up at compile time.

The update function will consider all fields from the field list to update. Optional fields will only 
be updated if they contain some value. See [the mapping](4-derive/12-update.md) for details.

#### Joins
You can update only the foreign key of a join or field from the join. Consider this field list:

```rust, ignore
let f = fields!(User, "*, address, address_*, address_id");
```

With `*` we consider all fields from User for updating, 
`address` will update the foreign key to `Address` in the `User` table,
`address_*` will update all simple fields in table `Address`
and finally `address_id` is ignored, since keys cannot be updated.

Notice in the example above `address` is actually a duplicate, because foreign keys are included in `*`.
It's just mentioned explicitly for the purpose of learning.

#### Merges
Updates can either 
- update existing structs in a `Vec` 
- or insert new structs in the `Vec` and delete removed structs. 

Consider this field list:

```rust, ignore
let f = fields!(User, "*, books, books_*");
```

- With `*` we consider all simple fields from User for updating (this excludes merges), 
- `books` resizes the `Vec`: It deletes all books that are linked to the user but are not found in the `books` vector and 
it inserts new book (toghether with possible [partial joins](../4-derive/8-partial-tables.md)).
- `books_*` will update all simple fields in the existing `books`.

### Example: Updating a Vec with new items.

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main(){
    use toql::prelude::{Cache, Toql, ToqlApi, fields};
    use toql::mock_db::MockDb;

    #[derive(Debug, PartialEq, Toql)]
    struct Book {
        #[toql(key)]
        id: u64,
        #[toql(key)]
        user_id: u64,
        title: Option<String>
    }

    #[derive(Debug, PartialEq, Toql)]
     #[toql(auto_key)]
    struct User {
        #[toql(key)]
        id: u64,
        name: Option<String>,
        #[toql(merge())]
        books : Option<Vec<Book>>
    }
    
    let cache = Cache::new();
    let mut toql = MockDb::from(&cache);

    let mut user = User {
        id: 27,
        name: Some("Joe".to_string()),
        books: Some(vec![
            Book{
                id: 100,
                user_id: 0,  // Invalid key
                title: Some("King Kong".to_string())
            },
            Book{
                id: 200,
                user_id: 27,
                title: Some("Batman".to_string())
            }
        ])
    };

    toql.update_one(&mut user, fields!(User, "*, books, books_*")).await.unwrap();
    assert_eq!(toql.take_unsafe_sqls(), 
   ["UPDATE Book SET title = 'Batman' WHERE id = 200 AND user_id = 27", 
    "UPDATE User SET name = 'Joe' WHERE id = 27",
    "DELETE user_books FROM Book user_books \
        JOIN User user ON user.id = user_books.user_id \
        WHERE user.id = 27 AND NOT (user_books.id = 200 AND user_books.user_id = 27)", 
    "INSERT INTO Book (id, user_id, title) VALUES (100, 27, 'King Kong')"]
    );

#   }    
```

To mark new books, add them with an invalid key. A value of `0` or an empty string `''` is considered invalid.
Normally databases start counting indexes from 1 and some databases consider an empty string like null, which is 
also forbidden as primary key. So this idea of invalid key should normally work, however check with you database.

In rare cases where this does not work. Insert and delete your `Vec` manually, using the `ToqlApi` functions.

In the example above the first book has an invalid composite key (`id`, `user_id`), because `user_id` is `0`. 
Toql will notice that and insert a new book (with the correct `user_id` of `27`). From the second book with `id 200` the field `title` will be updated.





 







