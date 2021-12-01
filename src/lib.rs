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

/* external_doc_test!(include_str!("1-introduction.md"));
external_doc_test!(include_str!("2-concept.md"));
external_doc_test!(include_str!("3-api/1-introduction.md"));
external_doc_test!(include_str!("3-api/2-load.md"));
external_doc_test!(include_str!("3-api/3-query-type.md"));
external_doc_test!(include_str!("3-api/4-insert.md"));
external_doc_test!(include_str!("3-api/5-update.md"));
external_doc_test!(include_str!("3-api/6-delete.md")); */

// TODO external_doc_test!(include_str!("3-api/7-writing-functions.md"));

external_doc_test!(include_str!("4-derive/1-introduction.md"));
// BUG skip_wildcrad external_doc_test!(include_str!("4-derive/2-fields.md"));
/*external_doc_test!(include_str!("4-derive/3-optional-fields.md"));
external_doc_test!(include_str!("4-derive/4-sql-expressions.md"));
external_doc_test!(include_str!("4-derive/5-field-handlers.md"));*/
// BUG double definition external_doc_test!(include_str!("4-derive/6-joins.md"));
external_doc_test!(include_str!("4-derive/7-join-handlers.md"));







}