use language_tags::LanguageTag;
use rdftk_core::model::literal::Literal;
use std::time::Duration;

#[test]
fn untyped() {
    let value = Literal::plain("a string");
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"");
}

#[test]
fn needs_escape() {
    let value = Literal::plain(r#"\ta "string"#);
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "\\\\ta \\\"string");
    assert_eq!(value.to_string(), "\"\\\\ta \\\"string\"");
}

#[test]
fn string_with_language() {
    let value = Literal::with_language("a string", LanguageTag::parse("en-us").unwrap());
    assert!(!value.has_data_type());
    assert!(value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"@en-US");
}

#[test]
fn typed_as_string() {
    let value = Literal::from("a string");
    assert!(value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(
        value.to_string(),
        "\"a string\"^^<http://www.w3.org/2001/XMLSchema#string>"
    );
}

#[test]
fn typed_as_boolean() {
    let value = Literal::from(true);
    assert!(value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "true");
    assert_eq!(
        value.to_string(),
        "\"true\"^^<http://www.w3.org/2001/XMLSchema#boolean>"
    );
}

#[test]
fn typed_as_long() {
    let value = Literal::from(1_i64);
    assert!(value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "1");
    assert_eq!(
        value.to_string(),
        "\"1\"^^<http://www.w3.org/2001/XMLSchema#long>"
    );
}

#[test]
fn typed_as_ulong() {
    let value = Literal::from(1_u64);
    assert!(value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "1");
    assert_eq!(
        value.to_string(),
        "\"1\"^^<http://www.w3.org/2001/XMLSchema#unsignedLong>"
    );
}

#[test]
fn typed_as_duration() {
    let value = Literal::from(Duration::from_secs(63542));
    println!("Duration Out: {}", value);
    assert!(value.has_data_type());
    assert!(!value.has_language());

    assert_eq!(
        value.to_string(),
        "\"PT63542S\"^^<http://www.w3.org/2001/XMLSchema#duration>"
    );
}

#[test]
fn test_no_datatype_is_less() {
    let lhs = Literal::plain("value-1");
    assert!(!lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::from("value-1");
    assert!(rhs.has_data_type());
    assert!(!rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_plain_less() {
    let lhs = Literal::plain("value-1");
    assert!(!lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::plain("value-2");
    assert!(!rhs.has_data_type());
    assert!(!rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_plain_equal() {
    let lhs = Literal::plain("value-1");
    assert!(!lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::plain("value-1");
    assert!(!rhs.has_data_type());
    assert!(!rhs.has_language());
    assert!(lhs == rhs);
}

#[test]
fn test_typed_less() {
    let lhs = Literal::from("value-1");
    assert!(lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::from("value-2");
    assert!(rhs.has_data_type());
    assert!(!rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_typed_equal() {
    let lhs = Literal::from("value-1");
    assert!(lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::from("value-1");
    assert!(rhs.has_data_type());
    assert!(!rhs.has_language());
    assert!(lhs == rhs);
}

#[test]
fn test_no_language_is_less() {
    let lhs = Literal::plain("value-1");
    assert!(!lhs.has_data_type());
    assert!(!lhs.has_language());
    let rhs = Literal::with_language("value-1", LanguageTag::parse("en-US").unwrap());
    assert!(!rhs.has_data_type());
    assert!(rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_language_is_less() {
    let lhs = Literal::with_language("value-1", LanguageTag::parse("en-GB").unwrap());
    assert!(!lhs.has_data_type());
    assert!(lhs.has_language());
    let rhs = Literal::with_language("value-1", LanguageTag::parse("en-US").unwrap());
    assert!(!rhs.has_data_type());
    assert!(rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_with_language_is_less() {
    let lhs = Literal::with_language("value-1", LanguageTag::parse("en-US").unwrap());
    assert!(!lhs.has_data_type());
    assert!(lhs.has_language());
    let rhs = Literal::with_language("value-2", LanguageTag::parse("en-US").unwrap());
    assert!(!rhs.has_data_type());
    assert!(rhs.has_language());
    assert!(lhs < rhs);
    assert!(rhs > lhs);
}

#[test]
fn test_with_language_equal() {
    let lhs = Literal::with_language("value-1", LanguageTag::parse("en-US").unwrap());
    assert!(!lhs.has_data_type());
    assert!(lhs.has_language());
    let rhs = Literal::with_language("value-1", LanguageTag::parse("en-US").unwrap());
    assert!(!rhs.has_data_type());
    assert!(rhs.has_language());
    assert!(lhs == rhs);
}
