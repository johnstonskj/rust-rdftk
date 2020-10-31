use proptest::prelude::*;
use rdftk_iri::Query;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_default_query() {
    let query = Query::default();
    assert!(query.is_empty());
    assert_eq!(query.value(), "");
    assert_eq!(query.to_string(), "?".to_string());
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _ = Query::from_str(&s);
    }
}
