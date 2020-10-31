use proptest::prelude::*;
use rdftk_iri::{Port, Scheme};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_port_from_str() {
    assert!(Port::from_str("1").is_ok());
    assert!(Port::from_str("80").is_ok());
    assert!(Port::from_str("8080").is_ok());

    assert!(Port::from_str("http").is_err());
    assert!(Port::from_str("-1").is_err());
    assert!(Port::from_str("8888888888").is_err());
}

#[test]
fn test_port_default_for() {
    assert!(Port::default_for(&Scheme::https()).is_some());

    assert!(Port::default_for(&Scheme::mailto()).is_none());
}

#[test]
fn test_port_display() {
    assert_eq!(Port::new(443).to_string(), ":443");
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _ = Port::from_str(&s);
    }

    #[test]
    fn valid_port_values(p in 0..u16::MAX) {
        let s = p.to_string();
        println!("valid_port_values {:?}", s);
//        assert!(Port::is_valid(&s));
        assert!(Port::from_str(&s).is_ok());
    }
}
