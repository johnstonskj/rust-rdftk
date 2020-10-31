use proptest::prelude::*;
use rdftk_iri::{Normalize, Path};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_resolve_no_relatives() {
    let base = Path::from_str("/b/c/d;p").unwrap();
    assert_eq!(
        base.resolve(&Path::from_str("g").unwrap()).unwrap(),
        Path::from_str("/b/c/g").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("g/").unwrap()).unwrap(),
        Path::from_str("/b/c/g/").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("/g").unwrap()).unwrap(),
        Path::from_str("/g").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("").unwrap()).unwrap(),
        Path::from_str("/b/c/d;p").unwrap()
    );
}

#[test]
fn test_resolve_relatives() {
    let base = Path::from_str("/b/c/d;p").unwrap();
    assert_eq!(
        base.resolve(&Path::from_str("./g").unwrap()).unwrap(),
        Path::from_str("/b/c/g").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str(".").unwrap()).unwrap(),
        Path::from_str("/b/c").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("./").unwrap()).unwrap(),
        Path::from_str("/b/c/").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("./././").unwrap()).unwrap(),
        Path::from_str("/b/c/").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("..").unwrap()).unwrap(),
        Path::from_str("/b").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../").unwrap()).unwrap(),
        Path::from_str("/b/").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../g").unwrap()).unwrap(),
        Path::from_str("/b/g").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../..").unwrap()).unwrap(),
        Path::from_str("").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../../").unwrap()).unwrap(),
        Path::from_str("/").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../../../../").unwrap())
            .unwrap(),
        Path::from_str("").unwrap()
    );
    assert_eq!(
        base.resolve(&Path::from_str("../../g").unwrap()).unwrap(),
        Path::from_str("/g").unwrap()
    );
}

#[test]
fn test_is_normalized() {
    assert!(Path::from_str("/").unwrap().is_normalized());
    assert!(Path::from_str("a").unwrap().is_normalized());
    assert!(Path::from_str("/a").unwrap().is_normalized());
    assert!(Path::from_str("a/b/c").unwrap().is_normalized());

    assert!(!Path::from_str("a/./b/c").unwrap().is_normalized());
    assert!(!Path::from_str("a/./././b/c").unwrap().is_normalized());
    assert!(!Path::from_str("a/../b/c").unwrap().is_normalized());
    assert!(!Path::from_str("a/b/../../c").unwrap().is_normalized());
    assert!(!Path::from_str("./a/./b/./c/.").unwrap().is_normalized());
    assert!(!Path::from_str("a/b/c/..").unwrap().is_normalized());
    assert!(!Path::from_str("../a/b/c").unwrap().is_normalized());
}

#[test]
fn test_normalize() {
    assert_eq!(
        Path::from_str("a/b/c").unwrap().normalize().unwrap(),
        Path::from_str("a/b/c").unwrap()
    );
    assert_eq!(
        Path::from_str("a/./b/c").unwrap().normalize().unwrap(),
        Path::from_str("a/b/c").unwrap()
    );
    assert_eq!(
        Path::from_str("a/./././b/c").unwrap().normalize().unwrap(),
        Path::from_str("a/b/c").unwrap()
    );
    assert_eq!(
        Path::from_str("a/../b/c").unwrap().normalize().unwrap(),
        Path::from_str("b/c").unwrap()
    );
    assert_eq!(
        Path::from_str("a/b/../../c").unwrap().normalize().unwrap(),
        Path::from_str("c").unwrap()
    );
    assert_eq!(
        Path::from_str("./a/./b/./c/.")
            .unwrap()
            .normalize()
            .unwrap(),
        Path::from_str("a/b/c").unwrap()
    );
    assert_eq!(
        Path::from_str("a/b/c/..").unwrap().normalize().unwrap(),
        Path::from_str("a/b").unwrap()
    );
    assert_eq!(
        Path::from_str("../a/b/c").unwrap().normalize().unwrap(),
        Path::from_str("a/b/c").unwrap()
    );
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _ = Path::from_str(&s);
    }
}
