
# Serde

Toql structs usually have a lot of `Option` types to make fields selectable with a query.
Let's look how to attribute them with serde for smooth interaction.

This requires the feature `serde` for Toql:

```toml
[dependencies]
toql = {version = "0.4", features=["serde"]}
 ```

## Serializing
It's nice to omit unselected fields. This can easily achieved with `#[serde(skip_serializing_if = "Option::is_none")]`

### Serialize example
```rust
    use toql::prelude::{Toql, Join};
    use serde::Serialize;
#   #[derive(Toql, Serialize)]
#   struct Address {
#       #[toql(key)]
#       id: u64
#   }

#[derive(Toql, Serialize)]
struct User {
    #[toql(key)]
    id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[toql(join)]
    address: Option<Option<Join<Address>>> // Selectable left join
}
```

## Deserializing
Your server needs deserializing either 
- when creating a new item 
- or when updating an existing item



### Deserialize example:

```rust, ignore
    use toql::prelude::Toql;
    #[derive(Toql)]
    #[toql(auto_key)]
    struct User {
    
        // Serde `default` allows missing field `id` in Json
        //  Needed typically for insert and auto key
        #[serde(default)] 
        #[toql(key)]
        id: u64
        
        // No Serde attribute
        // Field must always be present in Json, but may be `null` -> `Option::None`
        // Fields that are `None` wont be updated.
        name: Option<String>

        // Never deserialize expressions
        #[serde(skip_deserializing)]  
        #[toql(sql = "(SELECT COUNT(*) From Book b WHERE b.author_id = ..id)")]
        pub number_of_books: Option<u64>,
    
        // See comment below
        #[serde(default, deserialize_with="des_double_option")]
        address: Option<Option<Join<Address>>> 
    }
```

Notice the double `Option` on the selectable left join `address`. 
When deserializing from JSON the following mapping works:
 
|JSON | Rust|
|-----|-----|
| undefined| None|
| null | Some(None)|
| value | Some(Some(value))|

To make this happen you need a custom deserialization function:

```rust
use serde::{Deserializer, Deserialize};

pub fn des_double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
```

Now you get the following:
- If you omit address in your JSON `#[serde(default)]` kicks in and you get `None`.
- If you send `"addess": null`, you get `Some(None)`.
- If you send `"address: {"id": 5}"`, you get `Some(Some(Join::Key(AddressKey{id:5})))`.
- If you send `"address: {"id": 5, ...}"`, you get `Some(Some(Join::Entity(Address{id:5, ...})))`.

Toql update will now work as expected.






