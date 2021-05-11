use rdftk_core::literal::Literal;
use std::time::Duration;

#[test]
fn untyped() {
    let value = Literal::new("a string");
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"");
}

#[test]
fn needs_escape() {
    let value = Literal::new(r#"\ta "string"#);
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "\\\\ta \\\"string");
    assert_eq!(value.to_string(), "\"\\\\ta \\\"string\"");
}

#[test]
fn string_with_language() {
    let value = Literal::with_language("a string", "en_us");
    assert!(!value.has_data_type());
    assert!(value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"@en_us");
}

#[test]
fn typed_as_string() {
    let value: Literal = Literal::string("a string");
    assert!(value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(
        value.to_string(),
        "\"a string\"^^<http://www.w3.org/2001/XMLSchema#string>"
    );
}

#[test]
fn not_typed_as_string() {
    let value: Literal = "a string".to_string().into();
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"");
}

#[test]
fn typed_as_boolean() {
    let value: Literal = true.into();
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
    let value: Literal = 1u64.into();
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
    let duration = Duration::from_secs(63542);
    let value: Literal = duration.into();
    println!("Duration Out: {}", value);
    assert!(value.has_data_type());
    assert!(!value.has_language());

    assert_eq!(
        value.to_string(),
        "\"PT63542S\"^^<http://www.w3.org/2001/XMLSchema#duration>"
    );
}
