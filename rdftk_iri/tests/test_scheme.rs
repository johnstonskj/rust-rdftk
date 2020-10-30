use proptest::prelude::*;
use rdftk_iri::{Normalize, Scheme, ValidateStr};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_parse_examples() {
    // simple string
    assert!(Scheme::from_str("aaa").is_ok());

    // string with numbers
    assert!(Scheme::from_str("tn3270").is_ok());

    // string with dots
    assert!(Scheme::from_str("iris.beep").is_ok());

    // string with hyphens
    assert!(Scheme::from_str("xcon-userid").is_ok());
}

#[test]
fn test_display_examples() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap().to_string(),
        "aaa:".to_string()
    );

    // string with numbers
    assert_eq!(
        Scheme::from_str("tn3270").unwrap().to_string(),
        "tn3270:".to_string()
    );

    // string with dots
    assert_eq!(
        Scheme::from_str("iris.beep").unwrap().to_string(),
        "iris.beep:".to_string()
    );

    // string with hyphens
    assert_eq!(
        Scheme::from_str("xcon-userid").unwrap().to_string(),
        "xcon-userid:".to_string()
    );
}

#[test]
fn test_parse_illegal() {
    assert_eq!(Scheme::is_valid(""), false, "should not be empty");
    assert!(Scheme::from_str("").is_err());

    assert_eq!(Scheme::is_valid(" "), false, "should not accept spaces");
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid(" aaa"),
        false,
        "should not leading accept spaces"
    );
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid("aa a"),
        false,
        "should not embedded accept spaces"
    );
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid("123"),
        false,
        "should not accept all numbers"
    );
    assert!(Scheme::from_str("123").is_err());

    assert_eq!(
        Scheme::is_valid("123ab"),
        false,
        "should not accept leading numbers"
    );
    assert!(Scheme::from_str("123ab").is_err());

    assert_eq!(Scheme::is_valid("a!"), false, "should not accept !");
    assert!(Scheme::from_str("a!").is_err());
}

#[test]
fn test_normalize_examples() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
    assert_eq!(
        Scheme::from_str("AAA").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
    assert_eq!(
        Scheme::from_str("aAa").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
}

#[test]
fn test_equality() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap(),
        Scheme::from_str("AAA").unwrap()
    );
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        match Scheme::from_str(&s) {
            Ok(_) => println!("Ok()"),
            Err(_) => println!("Err()"),
        };
    }

    #[test]
    fn valid_values(s in "[[:alpha:]][[[:alnum:]]+-\\.]*") {
        println!("valid_values {:?}", s);
        assert!(Scheme::from_str(&s).is_ok());
    }
}

// ------------------------------------------------------------------------------------------------
// Regression Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_unicode_gibberish() {
    assert!(Scheme::from_str("º").is_err());
}
