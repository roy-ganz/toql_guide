# Field handlers

It's possible to write an own field handler. Do it, because

 - You want to build an SQL expression with a function.
 - You want to support a database function through [`FN`](../5-query-language/4-filter.md)
 - You want to abuild a filter condition with a function

 
 ## Filter on fields

 Let's support a length function `LLE` , so that we can filter on maximum  word length

 ```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
    use toql::prelude::{ToqlApi, DefaultFieldHandler, FieldHandler, 
                SqlExpr, SqlBuilderError, FieldFilter, Cache, Toql,
                ParameterMap, sql_expr, query};
    use toql::mock_db::MockDb;

    struct LengthFieldHandler{
        // The default field handler gives us default filters 
        // such as `EQ`, `NE`, ...
        default_handler: DefaultFieldHandler, 
    };

    impl FieldHandler for LengthFieldHandler
    {
        fn build_filter(
            &self,
            select: SqlExpr,            // Column or SQL expression
            filter: &FieldFilter,       // The filter called with this field
            aux_params: &ParameterMap,  // All aux params available
        ) -> Result<Option<SqlExpr>, SqlBuilderError> {
            match filter {
                // Support our custom LLE filter that maps 
                // to the MySQL LENGTH function
                FieldFilter::Fn(name, args) => match name.as_str() {
                    "LLE" => {
                        if args.len() != 1 {
                            return Err(SqlBuilderError::FilterInvalid( 
                                "filter `FN LLE` expects exactly 1 argument".to_string()));
                        }
                        Ok(Some(sql_expr!("LENGTH ({}) <= ?", select, &args[0])))
                    }
                    name @ _ => Err(SqlBuilderError::FilterInvalid(name.to_string())),
                },
                _ => self.default_handler.build_filter(select, filter, aux_params),
            }
        }

    }

    // Getter method for mapper
    pub fn length_field_handler() -> impl FieldHandler {
        LengthFieldHandler{
            default_handler: DefaultFieldHandler::new(), 
        }
    }

    #[derive(Toql)]
    struct User {
        #[toql(key)]
        id: u64,

        #[toql(handler="length_field_handler")]
        name: Option<String>,
    }

    let cache = Cache::new();
    let mut toql = MockDb::from(&cache);

    let q = query!(User, "name FN LLE 5"); 
    let mut _users = toql.load_many(&q).await.unwrap(); 
    assert_eq!(toql.take_unsafe_sql(), 
            "SELECT user.id, user.name \
            FROM User user \
            WHERE LENGTH (user.name) <= 5");
#   }
```

For a bigger example, check out our [permission handler](6-appendix/4-row-access-control.md).

### Field handlers with local aux params
If you want to use the same field handler in different places 
it mightly come handy to give the field handler some local context.

This can be achieved with local aux_params:

```rust, ignore
    #[toql(sql="", 
        field_handler="smart_name_handler", 
        aux_param(name="strategy", value="quick"))]
    smart_name: String
```
The aux param `strategy` is only available in the `smart_name_handler`. Only strings values are supported.


