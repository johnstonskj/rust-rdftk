use rdftk_core::simple::literal::literal_factory;
use std::time::Duration;

#[test]
fn untyped() {
    let literals = literal_factory();
    let value = literals.literal("a string");
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"");
}

#[test]
fn needs_escape() {
    let literals = literal_factory();
    let value = literals.literal(r#"\ta "string"#);
    assert!(!value.has_data_type());
    assert!(!value.has_language());
    assert_eq!(value.lexical_form(), "\\\\ta \\\"string");
    assert_eq!(value.to_string(), "\"\\\\ta \\\"string\"");
}

#[test]
fn string_with_language() {
    let literals = literal_factory();
    let value = literals.with_language_str("a string", "en-us").unwrap();
    assert!(!value.has_data_type());
    assert!(value.has_language());
    assert_eq!(value.lexical_form(), "a string");
    assert_eq!(value.to_string(), "\"a string\"@en-us");
}

#[test]
fn typed_as_string() {
    let literals = literal_factory();
    let value = literals.string("a string");
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
    let literals = literal_factory();
    let value = literals.boolean(true);
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
    let literals = literal_factory();
    let value = literals.long(1);
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
    let literals = literal_factory();
    let value = literals.unsigned_long(1);
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
    let literals = literal_factory();
    let duration = Duration::from_secs(63542);
    let value = literals.duration(duration);
    println!("Duration Out: {}", value);
    assert!(value.has_data_type());
    assert!(!value.has_language());

    assert_eq!(
        value.to_string(),
        "\"PT63542S\"^^<http://www.w3.org/2001/XMLSchema#duration>"
    );
}
