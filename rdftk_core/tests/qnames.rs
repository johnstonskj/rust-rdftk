use rdftk_core::model::qname::QName;
use std::str::FromStr;

#[test]
fn new_qname() {
    let qname = QName::new("foo").unwrap();
    assert_eq!(qname.to_string(), ":foo".to_string());
    assert_eq!(qname.as_curie(), "[:foo]".to_string());

    let qname = QName::with_prefix("rdf", "foo").unwrap();
    assert_eq!(qname.to_string(), "rdf:foo".to_string());
    assert_eq!(qname.as_curie(), "[rdf:foo]".to_string());
}

#[test]
fn qname_from_str() {
    let qname = QName::from_str("foo");
    assert!(qname.is_ok());
    assert_eq!(qname.unwrap().to_string(), ":foo".to_string());

    let qname = QName::from_str("rdf:foo");
    assert!(qname.is_ok());
    assert_eq!(qname.unwrap().to_string(), "rdf:foo".to_string());
}

#[test]
fn qname_from_str_fail() {
    let qname = QName::from_str("");
    assert!(qname.is_err());

    let qname = QName::from_str("rdf foo");
    assert!(qname.is_err());

    let qname = QName::from_str(":foo");
    assert!(qname.is_err());

    let qname = QName::from_str("rdf::foo:bar");
    assert!(qname.is_err());
}
