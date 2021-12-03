
# Joins
A struct can refer to another struct. This is done with a SQL join. 

Joins are added to the SQL statement when
- requested in the query, like so: `phone1_id`
- or joins are preselected.


#### Join mapping example

```rust
use toql::prelude::Toql;

#[derive(Toql)]
struct Phone {
	#[toql(key)]
	country: u8,
	#[toql(key)]
	number: u64,
}

#[derive(Toql)]
struct User {

	#[toql(key)]
	 id: u32,	

	 name: Option<String>,

	 #[toql(join())]  
	 phone1 : Phone, 		// Always selected inner join

	 #[toql(join())]  
	 phone2 : Option<Phone>, // Selectable inner join

	 #[toql(join())]  
	 phone3 : Option<Option<Phone>>, // Selectable left join

	 #[toql(join(), preselect)]  
	 phone4 : Option<Phone>, // Always selected left join
}
```
Notice how `Option` makes the difference between an inner join and a left join.

## Renaming joined columns
By default foreign keys are calulated by the primary columns of the join and the field name of the join.
For the above it would be *phone1_id*, *phone2_id*, *phone3_id* and *phone4_id*.


If your naming scheme differs from that default behaviour, use the `columns` attribute:
```rust, ignore
#[toql(join(columns(self="mobile1_id", other="id")))]  
phone1 : Phone 
```

For a composite key use `columns` multiple times.


## Custom ON predicate

It's possible to restrict the join with a `ON` SQL predicate. 

Here an example of a translated country name. 

```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
	use toql::prelude::{Toql, ToqlApi,Cache, query, ContextBuilder};
	use toql::mock_db::MockDb;
	use std::collections::HashMap;

	#[derive(Toql)]
	struct User {
		#[toql(key)]
		id: u32,	

		#[toql(join)]
		country: Option<Country>
	}
 
	#[derive(Toql)]
	struct Country {

		#[toql(key)]
		id: u32,	

		#[toql(join(
				columns(self = "id", other = "id"), 
				on_sql = "...language_id=<interface_language_id>"))]
		pub translation: Option<CountryTranslation>
	}

	#[derive(Toql)]
	pub struct CountryTranslation {

		#[toql(key)]
		pub id: String,
		
		pub title: String,
	}

  
    let mut p = HashMap::new();
    p.insert("interface_language_id".into(), "en".into());
    
    let context = ContextBuilder::new().with_aux_params(p).build();
    let cache = Cache::new();
    let mut toql = MockDb::with_context(&cache, context);

    let q = query!(User, "country_translation_title");

    let mut _users = toql.load_many(&q).await.unwrap(); 
    assert_eq!(toql.take_unsafe_sql(), 
            "SELECT user.id, user_country.id, user_country_translation.id, user_country_translation.title \
				FROM User user \
				JOIN (Country user_country \
					JOIN (CountryTranslation user_country_translation) \
					ON (user_country.id = user_country_translation.id \
						AND user_country_translation.language_id='en')) \
				ON (user.country_id = user_country.id)"); 
#   }

```
You can use any raw SQL in the `ON` predicate. Did you spot the `...` alias? 
This will resolve to the alias of the joined struct (CountryTranslation). 

Apart from `ON` predicates the `...` alias can also be used in custom merge predicates.

It is also possible to use the regular `..` alias to refer to the joining struct (Country), but we don't need it here.

You can use auxiliary parameters (here *<interface_language_id>*) in `ON` expression. 
Aux params usually come from a context, query.

However for `ON` there is a third source : Aux params may also come from [query predicates](10-predicates.md).

This allows some nifty joining, see here:

### Example with on_aux_param
```rust
#   #[tokio::main(flavor="current_thread")]
#   async fn main() {
	use toql::prelude::{Toql, ToqlApi,Cache, query, ContextBuilder};
	use toql::mock_db::MockDb;


	#[derive(Toql)]
	#[toql( predicate(	
				name = "language", 
				sql = "EXISTS(SELECT 1 FROM Country c \
					JOIN Language l ON (c.id= l.id)) WHERE l.id= ?)", 
				on_aux_param(name="language_id", index = 0)
			))]
	struct Country {

		#[toql(key)]
		id: u32,	

		#[toql(join(on_sql = "...code = <language_id>"))]
		pub language: Option<Language>
	}

	#[derive(Toql)]
	pub struct Language {

		#[toql(key)]
		pub code: String,
		
		pub title: String,
	}
	let cache = Cache::new();
    let mut toql = MockDb::from(&cache);

    let q = query!(Country, "@language 'fr'");

    let mut _users = toql.load_many(&q).await.unwrap(); 
    assert_eq!(toql.take_unsafe_sql(), 
            "SELECT user.id, user_country.id, user_country_translation.id, user_country_translation.title \
				FROM User user \
				JOIN (Country user_country \
					JOIN (CountryTranslation user_country_translation) \
					ON (user_country.id = user_country_translation.id \
						AND user_country_translation.language_id='en')) \
				ON (user.country_id = user_country.id)"); 
#   }
```

Above we add a predicate that allows to filter all countries by a language.
There can be multiple countries that speak the same language.

The predicate takes the one argument (`?`) and adds it to the aux_params for custom joins (`on_param`). 

When the predicate is used in a Toql query, lets say  `*, @language 'fr'` the SQL will return countries that speak french.
In addition it will add `fr` to the aux_params when doing the custom join.  

So each country will contain the `language` field with information about french.

It's somehow hacky, but it works and is useful in 1-n situations when you want 1-1 .


## Insert / update implications

Toql can insert joins with renamed columns and no custom `ON` expression, 
because key propagation is done internally through common column names.
Joins with custom `ON` expressions can't be inserted or updated, they are read only.




## The Join struct
Joining directly another struct is not ergonomic when you want to update the struct. 
Thats why the `Join` enum exists. It can either take a struct value or just its key.

Consider this

```rust, ignore
use toql::prelude::Toql;
# 	#[derive(Toql)]
# 	struct Phone {
#		#[toql(key)]
#		country: u8,
#		#[toql(key)]
#		number: u64,
#	}

#[derive(Toql)]
struct User {

	#[toql(key)]
	 id: u32,	

	#[toql(join)]
	 phone: Phone
}
```

Here when we want to set a new `Phone` for the user, we need to provide a full `Phone` struct
even tough we only want to set a new value for the foreign key `phone_id` in `User`.
This feels unnesseary and `toql::prelude::Join` comes to our rescue:

```rust,ignore
use toql::prelude::{Toql, Join};
# 	#[derive(Toql)]
# 	struct Phone {
#		#[toql(key)]
#		country: u8,
#		#[toql(key)]
#		number: u64,
#	}

#[derive(Toql)]
struct User {

	#[toql(key)]
	 id: u32,	

	#[toql(join)]
	 phone: Join<Phone>,

	 #[toql(join)]
	 phone2: Option<Option<Join<Phone>>>
}
```

This has the following advantages:
 - Loads as normal, `Join` will always hold a full value.
 - Updating the `phone_id` column in User requires only a `PhoneKey`.
   This key can be always be taken out from `Join`.
 - Web clients can send in keys or full entities. `Join` will deserialize into whatever is possible.
  
For working with joins in your code checkout the `toql::prelude::join!` or `toql::prelude::rval_join!` macros.


## Sidenote for SQL generation

If you watch the generated SQL joins, you will notice that JOIN statements look slightly more complicated from Toql than you may expect.

This is because Toql builds correctly nested JOIN statements that reflect the dependencies among the joined structs. Any SQL builder that simply concatenates inner joins and left joins may accidentally turn left joins into inner joins. This database behaviour is not well known and usually surprises users - Toql avoids this.








