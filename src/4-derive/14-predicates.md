
# Predicates
All normal filtering is based on fields, [see here](../5-query-language/4-filter.md). 
However sometimes you may have a completely different filter criteria that can't be mapped on fields. 

An example is the MySQL full text search. Let's do it:

```rust
use toql::prelude::Toql;
#[derive(Toql)]
#[toql(predicate(name = "search", 
		    sql = "MATCH (..firstname, ..lastname) AGAINST (?  IN BOOLEAN MODE)"),
      predicate(name = "street", 
		    sql = "EXISTS( SELECT 1 FROM User u \
                JOIN Address a ON (u.address_id = a.id) \
		 	    WHERE a.street = ? AND u.id = ..id)"))]
struct User {

 #[toql(key)]
 id: u64,

 firstname: String,
 lastname: String,
}
```

With the two predicates above you can seach for users that have a certain name with `@search 'peter'` 
and retrieve all users from a certain street with `@street 'elmstreet'`.

The question marks in the predicate are replaced by the arguments provided. 
Arguments can also be used to build an `ON` predicate in a join. See [on aux param](6-joins.md).

## Predicate naming
Predicate names are made up with letters, digits and underscores. They must begin with a letter.

While you can use any case in the derive definition, the derive will transform the name into `mixedCase` to follow
the naming scheme of the struct fields. So a predicate name such as `My_Address_contains` corresponds 
in the Toql query to `@myAddressContains` and the builder method wqould be called with `.my_address_contains()`.

Also names must be different than the fields names in a struct. Otherwise a duplicate definition error will occur.
This is currently a limitation in the library.

## Reference

The full predicate syntax is

```rust, ignore
    predicate(
      name="..",
      sql="..",
      handler="..",
      on_aux_param(name"..", index = ..),
      count_filter
``` 

where 
- __name__ is the name of the predicate. It can be called in a Toql query with `@name ..`. 
  If a predicate is defined on a joined struct, that predicate can be called with a path
  `@path_name ..`. See [predicates in the query](../5-query-language/6-predicates.md) for more details.
- __sql__ is a raw SQL expression. Use `?` to insert a predicate param in the SQL, 
  `..` for the table alias and `<aux_param>` for an aux param value.
- __handler__ allows a custom predicate handler (build SQL with a function). 
  Provide a function name without parenthesis that return a struct that implement `toql::prelude::PredicateHandler`
- __on_aux_param__ sets an aux param to the value of a predicate argument. Index refers to the argument in the SQL expression with 0 being the first `?`. This aux param is only available when building `ON` conditions for joins.
  and can only be used when the predicate takes exactly one argument. See [example](6-joins.md).
- __count_filter__ makes Toql to use that a predicate for [count queries](../3-api/2-load.md). 
 