
# Selections
Selections are a list of fields and can be defined on a struct. 
A Toql query can then select the selection instead of all the individual fields. 

## Builtin selections
For more information about selections, see also the [chapter](../5-query-language/5-selections.md) in the query language.

#### The mutable selection
The builtin `$mut` selection selects all fields that are mutable.
This can be used for debugging purposes.
The fields of this selection cannot be user defined.

#### The all selection
The `$all` selects all fields from a struct. This includes fields that are marked with `skip_wildcard`.
Joins and merges are not selected.
The fields of this selection cannot be user defined.

#### The standart selection
The standart selection `$std` behaves like any other user defined selection.
However Toql supports this standart selection with the special short alias `$` in Toql queries.

If the derive doesn't map the `$std` selection a query asking for it will fail. 
This is the same behaviour as for any other user defined selection.

#### The count selection
To build count queries Toql considers filters on fields that are part of the `$cnt`
selection. 

Predicates that should also be considered when building a count selection must be marked 
with the `count_filter` flag. See the next chapter on [predicates](14-predicates.md).

## Example

```rust
use toql::prelude::Toql;
 
#[derive(Toql)]
#[toql(selection(name="std", fields="*, address_street"))]
#[toql(selection(name="tiny", fields="id, name"))]
struct User {

 #[toql(key)]
 id: u64,

 name: Option<String>,

 #[toql(join)]
 address: Option<Address>
}
 
#[derive(Toql)]
struct Address {

 #[toql(key)]
 id: u64,

 street: Option<String>
}
```

The selections above can now be used in a query. Instead of asking for `name, address_street` it is possible to ask for `$std` or just `$`.












