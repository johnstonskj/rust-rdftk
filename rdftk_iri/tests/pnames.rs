use rdftk_iri::{LocalName, Name, Namespace};
use std::str::FromStr;

#[test]
fn namespace_from_str() {
    let namespace = Namespace::from_str("foo:").unwrap();
    assert_eq!(namespace.to_string(), "foo:".to_string());

    let namespace = Namespace::from_str(":").unwrap();
    assert_eq!(namespace.to_string(), ":".to_string());
}

#[test]
fn new_local_name() {
    let local_name = LocalName::new_in_default(Name::new_unchecked("foo"));
    assert_eq!(local_name.to_string(), ":foo".to_string());
    assert_eq!(local_name.as_curie(), "[:foo]".to_string());

    let local_name = LocalName::new(Namespace::new_unchecked("rdf"), Name::new_unchecked("foo"));
    assert_eq!(local_name.to_string(), "rdf:foo".to_string());
    assert_eq!(local_name.as_curie(), "[rdf:foo]".to_string());
}

#[test]
fn local_name_from_str() {
    let local_name = LocalName::from_str(":foo");
    assert!(local_name.is_ok());

    assert_eq!(local_name.unwrap().to_string(), ":foo".to_string());

    let local_name = LocalName::from_str("rdf:foo");
    assert!(local_name.is_ok());
    assert_eq!(local_name.unwrap().to_string(), "rdf:foo".to_string());
}

#[test]
fn local_name_from_str_fail() {
    let local_name = LocalName::from_str("");
    assert!(local_name.is_err());

    let local_name = LocalName::from_str("rdf foo");
    assert!(local_name.is_err());

    let local_name = LocalName::from_str("rdf:foo:bar");
    assert!(local_name.is_err());

    let local_name = LocalName::from_str("rdf::");
    assert!(local_name.is_err());

    let local_name = LocalName::from_str("rdf");
    assert!(local_name.is_err());
}
