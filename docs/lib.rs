/* use doc_comment::doctest;

#[cfg(doctest)]
doctest!("../src/1-introduction.md"); */


#[cfg(doctest)]
mod test_doc {
  macro_rules! external_doc_test {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
  }

external_doc_test!(include_str!("1-introduction.md"));
external_doc_test!(include_str!("2-concept.md"));
external_doc_test!(include_str!("3-api/1-introduction.md"));
external_doc_test!(include_str!("3-api/2-load.md"));
external_doc_test!(include_str!("3-api/3-query-type.md"));
external_doc_test!(include_str!("3-api/4-insert.md"));
external_doc_test!(include_str!("3-api/5-update.md"));
external_doc_test!(include_str!("3-api/6-delete.md"));

// TODO external_doc_test!(include_str!("3-api/7-writing-functions.md"));

external_doc_test!(include_str!("4-derive/1-introduction.md"));
external_doc_test!(include_str!("4-derive/2-fields.md"));
external_doc_test!(include_str!("4-derive/3-optional-fields.md"));
external_doc_test!(include_str!("4-derive/4-sql-expressions.md"));
external_doc_test!(include_str!("4-derive/5-field-handlers.md"));
external_doc_test!(include_str!("4-derive/6-joins.md"));
 external_doc_test!(include_str!("4-derive/7-join-handlers.md"));
external_doc_test!(include_str!("4-derive/8-partial-tables.md"));
external_doc_test!(include_str!("4-derive/9-merges.md"));
external_doc_test!(include_str!("4-derive/10-keys.md"));
external_doc_test!(include_str!("4-derive/11-insert.md"));
external_doc_test!(include_str!("4-derive/12-update.md"));
external_doc_test!(include_str!("4-derive/13-selections.md"));
external_doc_test!(include_str!("4-derive/14-predicates.md"));
external_doc_test!(include_str!("4-derive/15-predicate-handlers.md")); 
external_doc_test!(include_str!("4-derive/16-roles.md"));
external_doc_test!(include_str!("4-derive/17-reference.md"));

external_doc_test!(include_str!("5-query-language/1-introduction.md"));
external_doc_test!(include_str!("5-query-language/2-select.md"));
external_doc_test!(include_str!("5-query-language/3-order.md"));
external_doc_test!(include_str!("5-query-language/4-filter.md"));
external_doc_test!(include_str!("5-query-language/5-selections.md"));
external_doc_test!(include_str!("5-query-language/6-predicates.md"));

external_doc_test!(include_str!("6-appendix/1-introduction.md"));
// Deps problem external_doc_test!(include_str!("6-appendix/2-mysql-enums.md"));
external_doc_test!(include_str!("6-appendix/3-row-access-control.md"));
external_doc_test!(include_str!("6-appendix/4-serde.md"));
external_doc_test!(include_str!("6-appendix/5-debugging-toql.md"));
}