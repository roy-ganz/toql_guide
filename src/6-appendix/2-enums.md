# MySQL Enums

To map enums between a column and a struct field 
- some boilerplate code for Toql is required. 
- the enum must implement the `ToString` and `FromStr` traits.

For the first there exists the `ToqlEnum` derive and for the later several crates exit. Here an example with [strum](https://crates.io/crates/strum):

With this in Cargo.toml
```toml
[dependencies]
strum = "0.22"
strum_macros = "0.22"
 ```
you can attribute your enums:
 ```rust
use toql::prelude::ToqlEnum;
use strum_macros::{Display, EnumString};

#[derive(PartialEq, EnumString, Display, ToqlEnum)]
 enum Mood {
    Happy,
    Sad
} 
```
Now _Mood_ can be used:

```rust
use toql::prelude::Toql;
#   use toql::prelude::{ToqlEnum};
#   use strum_macros::{Display, EnumString};
#   #[derive(PartialEq, EnumString, Display, ToqlEnum)]
#    enum Mood {
#       Happy,
#       Sad
#   } 

#[derive (Toql)]
struct User {
    #[toql(key)]
    id : u64,
    name: Option<String>,
    mood: Option<Mood>
}
```

