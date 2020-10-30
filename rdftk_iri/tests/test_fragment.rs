use rdftk_iri::{Fragment, ValidateStr};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_is_valid() {
    assert!(Fragment::is_valid(""));
    assert!(Fragment::is_valid("aaa"));
    assert!(Fragment::is_valid("aaa123"));
    assert!(Fragment::is_valid("aaa-123"));
    assert!(Fragment::is_valid("aaa_123"));
    assert!(Fragment::is_valid("aaa(123)"));
}

#[test]
fn test_is_not_valid() {
    assert!(!Fragment::is_valid(" "));
}

#[test]
fn test_from_str() {
    assert_eq!(Fragment::from_str("").unwrap().value(), &"".to_string());
    assert_eq!(
        Fragment::from_str("aaa").unwrap().value(),
        &"aaa".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa123").unwrap().value(),
        &"aaa123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa-123").unwrap().value(),
        &"aaa-123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa_123").unwrap().value(),
        &"aaa_123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa(123)").unwrap().value(),
        &"aaa(123)".to_string()
    );
}

#[test]
fn test_display() {
    assert_eq!(Fragment::from_str("").unwrap().to_string(), "#".to_string());
    assert_eq!(
        Fragment::from_str("aaa").unwrap().to_string(),
        "#aaa".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa123").unwrap().to_string(),
        "#aaa123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa-123").unwrap().to_string(),
        "#aaa-123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa_123").unwrap().to_string(),
        "#aaa_123".to_string()
    );
    assert_eq!(
        Fragment::from_str("aaa(123)").unwrap().to_string(),
        "#aaa(123)".to_string()
    );
}
