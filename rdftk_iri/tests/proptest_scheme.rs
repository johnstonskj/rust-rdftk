use rdftk_iri::Scheme;
use std::str::FromStr;

use proptest::prelude::*;

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

#[test]
fn test_unicode_gibberish() {
    assert!(Scheme::from_str("º").is_err());
}
